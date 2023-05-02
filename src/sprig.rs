/* Original code[1] Copyright (c) 2021 Andrew Christiansen[2]
   Modified code[3] by Shane Celis[4] Copyright (c) 2023 Hack Club[6]
   Licensed under the MIT License[5]

   [1]: https://github.com/sajattack/st7735-lcd-examples/blob/master/rp2040-examples/examples/draw_ferris.rs
   [2]: https://github.com/DrewTChrist
   [3]: https://github.com/shanecelis/trowel/blob/master/src/sprig.rs
   [4]: https://mastodon.gamedev.place/@shanecelis
   [5]: https://opensource.org/licenses/MIT
   [6]: https://hackclub.com
*/

// Ensure we halt the program on panic. (If we don't mention this crate it won't
// be linked.)
use defmt_rtt as _;
// use panic_halt as _;
// use embedded_graphics::{
//     mono_font::{ascii, MonoTextStyle},
//     text::Text,
// };
use panic_probe as _;

// use rp2040_hal as hal;

use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use embedded_sdmmc::SdMmcSpi;
use fugit::RateExtU32;
use rp_pico::hal::{self, pac::interrupt};
use hal::{clocks::Clock, timer::{Alarm0, monotonic::Monotonic}};
// use rp2040_hal::timer::monotonic::Monotonic;
use st7735_lcd::{Orientation, ST7735};
use rtic_monotonic::Monotonic as RticMonotonic;

// Serial port module
use hal::usb::UsbBus;
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC, UsbError};

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access.
use crate::{App, AppExt, Buttons, FpsApp};
use core::option::Option;
use embedded_alloc::Heap;
use hal::pac;
use try_default::TryDefault;
use circular_buffer::CircularBuffer;
use shared_bus::{NullMutex, SpiProxy};

use core::cell::RefCell;
use embedded_time::{fraction::Fraction, Clock as EClock, clock::Error, Instant as EInstant, duration::Microseconds};
// use core::fmt::Write;
use genio::Write;
use alloc::rc::Rc;

pub(crate) type SdMmcSpi0<'a> = SdMmcSpi<SpiProxy<'a, NullMutex<hal::Spi<hal::spi::Enabled, pac::SPI0, 8>>>,
                            hal::gpio::Pin<hal::gpio::bank0::Gpio21,
                                                hal::gpio::Output<hal::gpio::PushPull>>>;

#[global_allocator]
static HEAP: Heap = Heap::empty();

static mut STDOUT: Option<Stdout> = None;

static mut MONOTONIC_CLOCK: Option<MonotonicClock> = None;

// Taken from rp-hal-boards's pico_usb_serial_interrupt.rs example.
/// The USB Device Driver (shared with the interrupt).
static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;

/// The USB Bus Driver (shared with the interrupt).
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;

/// The USB Serial Device Driver (shared with the interrupt).
static mut USB_SERIAL: Option<SerialPort<hal::usb::UsbBus>> = None;

static mut SHARED_BUS: Option<shared_bus::BusManager<shared_bus::NullMutex<hal::Spi<hal::spi::Enabled, pac::SPI0, 8>>>> = None;
static mut FILE_SYS: Option<Result<RefSPIFS<'static>, embedded_sdmmc::Error<embedded_sdmmc::SdMmcError>>> = None;
static mut SD_SPI: Option<SdMmcSpi0<'static>> = None;

type Monotonic0 = Monotonic<Alarm0>;

mod fs;

use self::fs::{SPIFS, RefSPIFS};

pub struct MonotonicClock(RefCell<Monotonic0>);
impl MonotonicClock {
    fn new(monotonic: Monotonic0) -> Self {
        // https://docs.rs/rp2040-hal/latest/rp2040_hal/timer/monotonic/struct.Monotonic.html
        Self(monotonic.into())
    }
}

// https://docs.rs/embedded-time/0.12.1/embedded_time/clock/trait.Clock.html
impl EClock for MonotonicClock {
    // type T = Monotonic0::Instant::NOM;

    type T = u64;

    const SCALING_FACTOR: Fraction = Fraction::new(1, 1_000_000); // micro seconds

    fn try_now(&self) -> Result<EInstant<Self>, Error> {
        match self.0.try_borrow_mut() {
            Ok(mut m) => Ok(EInstant::<MonotonicClock>::new(m.now().ticks())),
            Err(_) => Err(Error::Unspecified),
        }
    }
}

impl TryDefault<FpsApp<MonotonicClock>> for FpsApp<MonotonicClock> {
    fn try_default() -> Option<Self> {
        unsafe { MONOTONIC_CLOCK.take() }.map(|clock| FpsApp::new(clock))
    }
}

pub fn try_now() -> Result<u64, &'static str> {
    let ref clock = unsafe { MONOTONIC_CLOCK.as_ref() };
    let clock = clock.ok_or("No clock setup")?;
    let now = clock.try_now()
        .map_err(|_| "Failed now")?;
    let duration = now.duration_since_epoch();
    let microseconds : Microseconds<u64> = duration.try_into()
        .map_err(|_| "Problem since epoch")?;
    Ok(microseconds.0)
}

/// The `run` function configures the RP2040 peripherals, then runs the app.
pub fn run_with<F, A>(app_maker: F) -> ()
where
    F: FnOnce() -> A,
    A: App,
{
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 12_000;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    if Some("1") == option_env!("SHOW_FPS") {
        if let Some(fps_app) = FpsApp::try_default() {
            _run_with(move || app_maker().join(fps_app));
        }
    } else {
        _run_with(app_maker);
    }
}

pub struct Stdout {
    buffer: CircularBuffer<64, u8>,
    error: Option<UsbError>
}

impl Stdout {
    fn can_drain(&self) -> bool {
        self.buffer.len() != 0
    }

    fn drain(&mut self, serial: &mut SerialPort<hal::usb::UsbBus>) {
        if self.buffer.len() != 0 {
            let (s1, s2) = self.buffer.as_slices();
            match serial.write(s1) {
                Ok(_written) => { },
                Err(err) => { self.error = Some(err);
                    return;
                }
            }
            if s2.len() != 0 {
                match serial.write(s2) {
                    Ok(_written) => { },
                    Err(err) => self.error = Some(err)
                }
            }
            self.buffer.clear();
        }
    }
}

impl Write for Stdout {
    type WriteError = UsbError;
    type FlushError = UsbError;
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::WriteError> {
        match self.error.take() {
            Some(err) => { return Err(err); },
            None => { }
        }
        for c in buf {
            self.buffer.push_back(*c);
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<(), Self::FlushError> {
        match self.error.take() {
            Some(err) => Err(err),
            None => Ok(())
        }
    }

    fn size_hint(&mut self, _bytes: usize) { }

    fn uses_size_hint(&self) -> bool {
        false
    }
}

pub fn stdout() -> &'static mut Stdout {
    unsafe { STDOUT.as_mut().expect("Could not get stdout") }
}

pub fn file_sys() -> Result<&'static mut RefSPIFS<'static>, crate::Error> {
    // Ok(fs::PCFS::new(None))
    unsafe { FILE_SYS.as_mut().unwrap().as_mut().map_err(|e| crate::Error::SdErr(e.clone())) }
}
// pub fn file_sys() -> &'static mut RefSPIFS<'static> {
//     unsafe { FILE_SYS.as_mut().unwrap() }
// }
const FPS_TARGET : u8 = 30; // frames per second
const FRAME_BUDGET : u64 = 1_000_000 /* micro seconds */ / FPS_TARGET as u64;

fn _run_with<F, A>(app_maker: F) -> ()
where
    F: FnOnce() -> A,
    A: App,
{
    // Grab our singleton objects.
    let mut pac = pac::Peripherals::take().expect("Could not get pac");
    let core = pac::CorePeripherals::take().expect("Could not get core");

    // Set up the watchdog driver--needed by the clock setup code.
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks.
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .expect("clock init failed.");

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // The single-cycle I/O block controls our GPIO pins.
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state.
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // These are implicitly used by the spi driver if they are in the correct mode
    let _spi_sclk = pins.gpio18.into_mode::<hal::gpio::FunctionSpi>();
    let _spi_mosi = pins.gpio19.into_mode::<hal::gpio::FunctionSpi>();
    let _spi_miso = pins.gpio16.into_mode::<hal::gpio::FunctionSpi>();
    let spi = hal::Spi::<_, _, 8>::new(pac.SPI0);

    let mut lcd_led = pins.gpio17.into_push_pull_output();
    let mut _led = pins.gpio25.into_push_pull_output();

    let mut l_led = pins.gpio28.into_push_pull_output();
    let mut r_led = pins.gpio4.into_push_pull_output();

    let dc = pins.gpio22.into_push_pull_output();
    let rst = pins.gpio26.into_push_pull_output();

    // Setup button pins.
    let w = pins.gpio5.into_pull_up_input();
    let a = pins.gpio6.into_pull_up_input();
    let s = pins.gpio7.into_pull_up_input();
    let d = pins.gpio8.into_pull_up_input();
    let i = pins.gpio12.into_pull_up_input();
    let j = pins.gpio13.into_pull_up_input();
    let k = pins.gpio14.into_pull_up_input();
    let l = pins.gpio15.into_pull_up_input();

    // Exchange the uninitialised SPI driver for an initialised one.
    let spi: hal::Spi<hal::spi::Enabled, pac::SPI0, 8> = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        16.MHz(),
        &embedded_hal::spi::MODE_0,
    );

    let bus: shared_bus::BusManager<shared_bus::NullMutex<hal::Spi<hal::spi::Enabled, pac::SPI0, 8>>>
        = shared_bus::BusManagerSimple::new(spi);
    unsafe {
        SHARED_BUS = Some(bus);
    }
    let bus = unsafe { SHARED_BUS.as_ref().unwrap() };
    // spi is single task/thread only.
    // let bus: &'static _ = shared_bus::new_cortexm!(hal::Spi<hal::spi::Enabled, pac::SPI0, 8> = spi).unwrap();
    // let bus: &'static _ = shared_bus::new_cortexm!(spi = spi).unwrap();
    let mut disp = ST7735::new(bus.acquire_spi(), dc, rst, true, false, 160, 128);
    let mut disp_cs = pins
        .gpio20
        .into_push_pull_output_in_state(hal::gpio::PinState::Low);

    disp.init(&mut delay).unwrap();
    disp.set_orientation(&Orientation::Landscape).unwrap();
    disp.clear(Rgb565::BLACK).unwrap();
    disp_cs.set_high().unwrap();

    let mut timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);
    // let mut alarm = hal::Timer::new(pac.TIMER, &mut pac.RESETS);
    let alarm = timer.alarm_0().unwrap();
    let monotonic = Monotonic::new(timer, alarm);
    let monotonic_clock = MonotonicClock::new(monotonic);
    unsafe {
        MONOTONIC_CLOCK = Some(monotonic_clock);
    }

    // Wait until the screen cleared otherwise the screen will show random
    // pixels for a brief moment.
    lcd_led.set_high().unwrap();

    // Initialize USB serial communication
    let usb_bus = UsbBusAllocator::new(UsbBus::new(pac.USBCTRL_REGS,pac.USBCTRL_DPRAM,clocks.usb_clock,true,&mut pac.RESETS));
    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet
        USB_BUS = Some(usb_bus);
    }

    // Grab a reference to the USB Bus allocator. We are promising to the
    // compiler not to take mutable access to this global variable whilst this
    // reference exists!
    let bus_ref = unsafe { USB_BUS.as_ref().unwrap() };

    let serial = SerialPort::new(bus_ref);

    unsafe {
        USB_SERIAL = Some(serial);
    }

    let usb_dev = UsbDeviceBuilder::new(bus_ref, UsbVidPid(0x16c0, 0x27dd))
    .manufacturer("Hack Club")
    .product("Sprig")
    .serial_number("0001")
    .device_class(USB_CLASS_CDC)
    .build();

    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet
        USB_DEVICE = Some(usb_dev);
    }

    unsafe {
        STDOUT = Some(Stdout { buffer: CircularBuffer::new(), error: None });
    }

    // Enable the USB interrupt
    unsafe {
        pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
    }

    let sdmmc_cs = pins.gpio21.into_push_pull_output();
    let sd_spi: SdMmcSpi0 = SdMmcSpi::new(bus.acquire_spi(), sdmmc_cs);
    unsafe {
        SD_SPI = Some(sd_spi);
    }
    let sd_spi: &'static mut _ = unsafe { SD_SPI.as_mut().unwrap() };

    let spifs_maybe = SPIFS::new(sd_spi);
    if spifs_maybe.is_ok() {
        let _ = l_led.set_high();
    } else {
        let _ = r_led.set_high();
    }
    unsafe { FILE_SYS = Some(spifs_maybe.map(|spifs| RefSPIFS(Rc::new(spifs)))) };
    // match spifs_maybe {
    //     Ok(spifs) => {
    //         // Successfully connected to SD Card.

    //         // let mut cont = Controller::new(block, time_source);
    //         // let volume = cont.get_volume(VolumeIdx(0)).expect("Unable to get volume");
    //         // let root = cont.open_root_dir(&volume).expect("Unable to get root dir");

    //     }
    //     Err(_) => {
    //         // Failed to connect to SD Card.
    //         let _ = r_led.set_high();

    //         // None
    //     }
    // };

    // Init the App
    // We could turn on the MCU's led.
    // led.set_high().unwrap();
    let mut app = app_maker();
    app.init().expect("error initializing");

    let mut buttons;
    let mut start : Result<u64, &'static str> = try_now();
    loop {
        // defmt::println!("Hello, world!");

        buttons = Buttons::empty();

        if w.is_low().unwrap() {
            buttons |= Buttons::W;
        }
        if a.is_low().unwrap() {
            buttons |= Buttons::A;
        }
        if s.is_low().unwrap() {
            buttons |= Buttons::S;
        }
        if d.is_low().unwrap() {
            buttons |= Buttons::D;
        }
        if i.is_low().unwrap() {
            buttons |= Buttons::I;
        }
        if j.is_low().unwrap() {
            buttons |= Buttons::J;
        }
        if k.is_low().unwrap() {
            buttons |= Buttons::K;
        }
        if l.is_low().unwrap() {
            buttons |= Buttons::L;
        }

        app.update(buttons).expect("error updating");

        if disp_cs.set_low().is_ok() {
            app.draw(&mut disp).expect("error drawing");
        }
        if disp_cs.set_high().is_ok() {
            // Might want to take note that this is required to make reading and
            // writing safe for sdcard.

            // if let Some(fs) = &mut fs {
            //     app.read_write(fs).expect("error reading/writing");
            // }
        }
        let stdout = unsafe { STDOUT.as_ref() }.expect("Could not get stdout");
        if stdout.can_drain() {
            unsafe {
                let stdout = STDOUT.as_mut().expect("Could not get stdout as mut");
                let serial = USB_SERIAL.as_mut().expect("Could not get usb serial");
                stdout.drain(serial);
            }
        }

        match start {
            Ok(s) => {
                let end = try_now();
                match end {
                    Ok(e) => {
                        // defmt::println!("FB {}, e {}, s {}", FRAME_BUDGET, e, s);
                        let x = FRAME_BUDGET as i64 - (e - s) as i64;

                        // defmt::println!("X {}", x);
                        if let Ok(leftover) = i32::try_from(x) {

                            // defmt::println!("C {}", leftover);
                            if leftover > 0 {
                                delay.delay_us(leftover as u32);
                            }
                        } else {
                            // defmt::println!("G");
                        }
                    },
                    Err(_e) => { },// defmt::println!("E {:?}", e); }
                }
                start = end;
            },
            Err(_e) => {
                // defmt::println!("F {:?}", e);
                start = try_now();
            }
        }
    };
}

/// This function is called whenever the USB Hardware generates an Interrupt
/// Request.
///
/// We do all our USB work under interrupt, so the main thread can continue on
/// knowing nothing about USB.
#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    // Grab the global objects. This is OK as we only access them under interrupt.
    let usb_dev = USB_DEVICE.as_mut().unwrap();
    let serial = USB_SERIAL.as_mut().unwrap();

    // Poll the USB driver with all of our supported USB Classes
    if usb_dev.poll(&mut [serial]) {
        let mut buf = [0u8; 64];
        match serial.read(&mut buf) {
            Err(_e) => {
                // Do nothing
            }
            Ok(0) => {
                // Do nothing
            }
            Ok(count) => {
                // Check if the message is "connected"
                let connected_msg = b"connected";
                if &buf[..count] != connected_msg {
                    // Convert to upper case
                    buf.iter_mut().take(count).for_each(|b| {
                        b.make_ascii_uppercase();
                    });

                    // Send back to the host
                    let mut wr_ptr = &buf[..count];
                    while !wr_ptr.is_empty() {
                        let _ = serial.write(wr_ptr).map(|len| {
                            wr_ptr = &wr_ptr[len..];
                        });
                    }
                }
            }
        }
    }
}
