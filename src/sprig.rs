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
use panic_probe as _;

use rp2040_hal as hal;

use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb565, prelude::*};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use fugit::RateExtU32;
use rp2040_hal::{clocks::Clock, timer::{Alarm0, monotonic::Monotonic}};
use st7735_lcd::{Orientation, ST7735};
use rtic_monotonic::Monotonic as RticMonotonic;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access.
use crate::{App, Buttons, AppResult};
use hal::pac;
use embedded_alloc::Heap;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;
use core::cell::RefCell;
use embedded_time::{fraction::Fraction, Clock, Instant as EInstant};

/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
/// if your board has a different frequency.
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[global_allocator]
static HEAP: Heap = Heap::empty();

static mut MONOTONIC_CLOCK: Option<MonotonicClock> = None;

pub fn init_heap() {
    if HEAP.used() == 0 && HEAP.free() == 0 {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 12_000;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }
}

type Monotonic0 = Monotonic<Alarm0>;

struct MonotonicClock(RefCell<Monotonic0>);
impl MonotonicClock {
    fn new(monotonic : Monotonic0) -> Self {
// https://docs.rs/rp2040-hal/latest/rp2040_hal/timer/monotonic/struct.Monotonic.html
        Self(monotonic.into())
    }
}

// https://docs.rs/embedded-time/0.12.1/embedded_time/clock/trait.Clock.html
impl EClock for MonotonicClock {
    // type T = Monotonic0::Instant::NOM;
    type T = u64;

    const SCALING_FACTOR: Fraction = Fraction::new(1, 1_000_000);

    fn try_now(&self) -> Result<EInstant<Self>, Error> {
        match self.0.try_borrow_mut() {
            Ok(mut m) => Ok(EInstant::<MonotonicClock>::new(m.now().ticks())),
            Err(_) => Err(Error::Unspecified)
        }
    }
}

impl Default for FpsApp {
    fn default() -> Self {
        FpsApp::new(unsafe { MONOTONIC_CLOCK.take() }.expect("could not get clock"))
    }
}

// <
//         ST7735<
//             hal::Spi<hal::spi::Enabled, pac::SPI0, 8>,
//             hal::gpio::Pin<Gpio22, hal::gpio::Output<hal::gpio::PushPull>>,
//             hal::gpio::Pin<Gpio26, hal::gpio::Output<hal::gpio::PushPull>>,
//         >,
//         (),
//     >
/// The `run` function configures the RP2040 peripherals, then runs the app.
pub fn run(app: impl App) -> ! {
    run_with(move || app);
}


pub fn run_with<F,A>(app_maker: F) -> !
        where F : FnOnce() -> A, A : App
    {
    init_heap();

    // Grab our singleton objects.
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver--needed by the clock setup code.
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks.
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
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
    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        16.MHz(),
        &embedded_hal::spi::MODE_0,
    );

    let mut disp = ST7735::new(spi, dc, rst, true, false, 160, 128);

    disp.init(&mut delay).unwrap();
    disp.set_orientation(&Orientation::Landscape).unwrap();
    disp.clear(Rgb565::BLACK).unwrap();

    let mut timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);
    // let mut alarm = hal::Timer::new(pac.TIMER, &mut pac.RESETS);
    let alarm = timer.alarm_0().unwrap();
    let monotonic = Monotonic::new(timer, alarm);
    let monotonic_clock = MonotonicClock::new(monotonic);
    unsafe { MONOTONIC_CLOCK = Some(monotonic_clock); }

    // Wait until the screen cleared otherwise the screen will show random
    // pixels for a brief moment.
    lcd_led.set_high().unwrap();

    // We could turn on the MCU's led.
    // led.set_high().unwrap();
    let mut app = app_maker();

    app.init().expect("error initializing");

    // let mut fps_app = FpsApp::new().expect("error init fps app");

    // let mut fps_counter =
    // let character_style = MonoTextStyle::new(&FONT_7X13, Rgb565::WHITE);
    // let fps_position = Point::new(5, 15);

    let mut buttons;
    loop {
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
        app.draw(&mut disp).expect("error drawing");
        // fps_app.draw(&mut disp).expect("error fps");
        // let fps = fps_counter.tick();
        // Text::new(&format!("FPS: {fps}"), fps_position, character_style).draw(&mut disp).expect("error on fps");
    }
}
