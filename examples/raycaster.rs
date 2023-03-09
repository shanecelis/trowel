// Example of running an ST7735 with an RP2040

#![no_std]
#![no_main]

// The macro for our start-up function
use cortex_m_rt::entry;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use defmt_rtt as _;
use panic_probe as _;

// Alias for our HAL crate
use rp2040_hal as hal;

// Some traits we need
//use cortex_m::prelude::*;
use embedded_graphics::image::{Image, ImageRaw, ImageRawLE};
use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::pixelcolor::raw::RawU24;
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_hal::digital::v2::OutputPin;
use rp2040_hal::clocks::Clock;
use st7735_lcd;
use st7735_lcd::Orientation;
use fugit::RateExtU32;
use embedded_hal::digital::v2::InputPin;
use embedded_graphics::{
    // mono_font::{ascii::FONT_6X10, MonoTextStyle},
    // pixelcolor::BinaryColor,
    prelude::*,
    primitives::{
        Circle,
        // PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle,
        Line
    },
    // text::{Alignment, Text},
    // mock_display::MockDisplay,
};

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use hal::pac;

use core::{
    // arch::wasm32,
    f32::consts::{FRAC_PI_2, PI, TAU},
    // panic::PanicInfo,
};
#[allow(unused_imports)]
use micromath::F32Ext;

// const DRAW_COLORS: *mut u16 = 0x14 as *mut u16;
const DRAW_COLORS: *mut u16 = 0b0001_0100 as *mut u16;
//                              fill_outline
// const GAMEPAD1: *const u8 = 0x16 as *const u8;

const BUTTON_LEFT: u8 = 16; // 00010000
const BUTTON_RIGHT: u8 = 32; // 00100000
const BUTTON_UP: u8 = 64; // 01000000
const BUTTON_DOWN: u8 = 128; // 10000000
// const HEIGHT: i32 = 160;
const HEIGHT: i32 = 128;


// const STEP_SIZE: f32 = 0.045;
const STEP_SIZE: f32 = 0.09;
const FIVE_PI_SQUARED: f32 = 5.0 * (PI * PI);

const FOV: f32 = PI / 2.7; // The player's field of view.
const HALF_FOV: f32 = FOV * 0.5; // Half the player's field of view.
const ANGLE_STEP: f32 = FOV / 160.0; // The angle between each ray.
// const WALL_HEIGHT: f32 = 100.0; // A magic number.
// const WALL_HEIGHT: f32 = HEIGHT as f32- 60.0; // A magic number.
const WALL_HEIGHT: f32 = HEIGHT as f32 - 60.0; // A magic number.
// const PALETTE : [u32; 4] = [0x2B2D24, 0x606751, 0x949C81, 0x3E74BC];
const PALETTE : [u32; 4] = [
    // 0xfff6d3,
    // 0xf9a875,
    // 0xeb6b6f,
    // 0x7c3f58,
0xe0f8cf,
0x86c06c,
0x306850,
0x071821

// #e0f8cf,
// #86c06c,
// #306850,
// #071821
];

fn to_color(c : u32) -> Rgb565 {
  Rgb565::from(Rgb888::from(RawU24::new(c)))
}

const MAP: [u16; 8] = [
    0b1111111111111111,
    0b1000001010000101,
    0b1011100000110101,
    0b1000111010010001,
    0b1010001011110111,
    0b1011101001100001,
    0b1000100000001101,
    0b1111111111111111,
];

static mut STATE: State = State {
    player_x: 1.5,
    player_y: 1.5,
    player_angle: -PI / 2.0,
};

#[no_mangle]
unsafe fn update(gamepad: u8, disp: &mut impl DrawTarget<Color = Rgb565>) {
    STATE.update(
        gamepad & BUTTON_UP != 0,
        gamepad & BUTTON_DOWN != 0,
        gamepad & BUTTON_LEFT != 0,
        gamepad & BUTTON_RIGHT != 0,
    );

    // let _ = disp.clear(to_color(PALETTE[0]));//Rgb565::BLACK);
    // Go through each column on screen and draw walls in the center.
    for (x, wall) in STATE.get_view().iter().enumerate() {
        let (height, shadow) = wall;

        // if *shadow {
        //     *DRAW_COLORS = 0x2;
        // } else {
        //     *DRAW_COLORS = 0x3;
        // }

    let color = to_color(PALETTE[if *shadow { 1 } else { 2 }]);
    let y = HEIGHT/2 - (height / 2);
    let _ = Line::new(Point::new(x as i32, y),
              Point::new(x as i32, y + *height - 1))
    .into_styled(PrimitiveStyle::with_stroke(
        color,
        1))
    .draw(disp);
    // let _ = Circle::new(Point::new(0, 0), 10)
    //     .into_styled(PrimitiveStyle::with_stroke(Rgb565::BLACK, 1))
    //     .draw(disp);

        // oval(0, 0, 10, 10);
        // vline(x as i32, 80 - (height / 2), *height as u32);
    }
}

struct State {
    player_x: f32,
    player_y: f32,
    player_angle: f32,
}

impl State {
    /// move the character
    pub fn update(&mut self, up: bool, down: bool, left: bool, right: bool) {
        // store our current position in case we might need it later
        let previous_position = (self.player_x, self.player_y);

        if up {
            self.player_x += cosf(self.player_angle) * STEP_SIZE;
            self.player_y += -sinf(self.player_angle) * STEP_SIZE;
        }

        if down {
            self.player_x -= cosf(self.player_angle) * STEP_SIZE;
            self.player_y -= -sinf(self.player_angle) * STEP_SIZE;
        }

        if right {
            self.player_angle -= STEP_SIZE;
        }

        if left {
            self.player_angle += STEP_SIZE;
        }

        // if moving us on this frame put us into a wall just revert it
        if point_in_wall(self.player_x, self.player_y) {
            (self.player_x, self.player_y) = previous_position;
        }
    }

    /// Returns 160 wall heights and their "color" from the player's perspective.
    pub fn get_view(&self) -> [(i32, bool); 160] {
        // The player's FOV is split in half by their viewing angle.
        // In order to get the ray's starting angle we must
        // add half the FOV to the player's angle to get
        // the edge of the player's FOV.
        let starting_angle = self.player_angle + HALF_FOV;

        let mut walls = [(0, false); 160];

        for (idx, wall) in walls.iter_mut().enumerate() {
            // `idx` is what number ray we are, `wall` is
            // a mutable reference to a value in `walls`.
            let angle = starting_angle - idx as f32 * ANGLE_STEP;

            // Get both the closest horizontal and vertical wall
            // intersections for this angle.
            let h_dist = self.horizontal_intersection(angle);
            let v_dist = self.vertical_intersection(angle);

            let (min_dist, shadow) = if h_dist < v_dist {
                (h_dist, false)
            } else {
                (v_dist, true)
            };

            // Get the minimum of the two distances and
            // "convert" it into a wall height.
            *wall = (
                (WALL_HEIGHT / (min_dist * cosf(angle - self.player_angle))) as i32,
                shadow,
            );
        }

        walls
    }

    /// Returns the nearest wall the ray intersects with on a horizontal grid line.
    fn horizontal_intersection(&self, angle: f32) -> f32 {
        // This tells you if the angle is "facing up"
        // regardless of how big the angle is.
        let up = fabsf(floorf(angle / PI) % 2.0) != 0.0;

        // first_y and first_x are the first grid intersections
        // that the ray intersects with.
        let first_y = if up {
            ceilf(self.player_y) - self.player_y
        } else {
            floorf(self.player_y) - self.player_y
        };
        let first_x = -first_y / tanf(angle);

        // dy and dx are the "ray extension" values mentioned earlier.
        let dy = if up { 1.0 } else { -1.0 };
        let dx = -dy / tanf(angle);

        // next_x and next_y are mutable values which will keep track
        // of how far away the ray is from the player.
        let mut next_x = first_x;
        let mut next_y = first_y;

        // This is the loop where the ray is extended until it hits
        // the wall. It's not an infinite loop as implied in the
        // explanation, instead it only goes from 0 to 256.
        //
        // This was chosen because if something goes wrong and the
        // ray never hits a wall (which should never happen) the
        // loop will eventually break and the game will keep on running.
        for _ in 0..256 {
            // current_x and current_y are where the ray is currently
            // on the map, while next_x and next_y are relative
            // coordinates, current_x and current_y are absolute
            // points.
            let current_x = next_x + self.player_x;
            let current_y = if up {
                next_y + self.player_y
            } else {
                next_y + self.player_y - 1.0
            };

            // Tell the loop to quit if we've just hit a wall.
            if point_in_wall(current_x, current_y) {
                break;
            }

            // if we didn't hit a wall on this extension add
            // dx and dy to our current position and keep going.
            next_x += dx;
            next_y += dy;
        }

        // return the distance from next_x and next_y to the player.
        distance(next_x, next_y)
    }

    /// Returns the nearest wall the ray intersects with on a vertical grid line.
    fn vertical_intersection(&self, angle: f32) -> f32 {
        // This tells you if the angle is "facing up"
        // regardless of how big the angle is.
        let right = fabsf(floorf((angle - FRAC_PI_2) / PI) % 2.0) != 0.0;

        // first_y and first_x are the first grid intersections
        // that the ray intersects with.
        let first_x = if right {
            ceilf(self.player_x) - self.player_x
        } else {
            floorf(self.player_x) - self.player_x
        };
        let first_y = -tanf(angle) * first_x;

        // dy and dx are the "ray extension" values mentioned earlier.
        let dx = if right { 1.0 } else { -1.0 };
        let dy = dx * -tanf(angle);

        // next_x and next_y are mutable values which will keep track
        // of how far away the ray is from the player.
        let mut next_x = first_x;
        let mut next_y = first_y;

        // This is the loop where the ray is extended until it hits
        // the wall. It's not an infinite loop as implied in the
        // explanation, instead it only goes from 0 to 256.
        //
        // This was chosen because if something goes wrong and the
        // ray never hits a wall (which should never happen) the
        // loop will eventually quit and the game will keep on running.
        for _ in 0..256 {
            // current_x and current_y are where the ray is currently
            // on the map, while next_x and next_y are relative
            // coordinates, current_x and current_y are absolute
            // points.
            let current_x = if right {
                next_x + self.player_x
            } else {
                next_x + self.player_x - 1.0
            };
            let current_y = next_y + self.player_y;

            // Tell the loop to quit if we've just hit a wall.
            if point_in_wall(current_x, current_y) {
                break;
            }

            // if we didn't hit a wall on this extension add
            // dx and dy to our current position and keep going.
            next_x += dx;
            next_y += dy;
        }

        // return the distance from next_x and next_y to the player.
        distance(next_x, next_y)
    }
}

fn point_in_wall(x: f32, y: f32) -> bool {
    match MAP.get(y as usize) {
        Some(line) => line & (0b1_u16 << core::cmp::min(x.floor() as u16, 15)) != 0,
        None => true,
    }
}

fn distance(a: f32, b: f32) -> f32 {
    sqrtf((a * a) + (b * b))
}

fn sinf(mut x: f32) -> f32 {
    x.sin()
    // let y = x / TAU;
    // let z = y - floorf(y);
    // x = z * TAU;

    // let sinf_imp = |x: f32| -> f32 {
    //     // these magic numbers were discovered 1400 years ago!
    //     (16.0 * x * (PI - x)) / (FIVE_PI_SQUARED - (4.0 * x * (PI - x)))
    // };

    // if x > PI {
    //     -sinf_imp(x - PI)
    // } else {
    //     sinf_imp(x)
    // }
}

fn cosf(x: f32) -> f32 {
    x.cos()
    // sinf(x + FRAC_PI_2)
}

fn tanf(x: f32) -> f32 {
    x.tan()
    // sinf(x) / cosf(x)
}

fn sqrtf(x: f32) -> f32 {
    // unsafe { core::intrinsics::sqrtf32(x) }
    x.sqrt()
}

fn floorf(x: f32) -> f32 {
    x.floor()
    // unsafe { core::intrinsics::floorf32(x) }
}

fn ceilf(x: f32) -> f32 {
    x.ceil()
    // unsafe { core::intrinsics::ceilf32(x) }
}

fn fabsf(x: f32) -> f32 {
    // unsafe { core::intrinsics::fabsf32(x) }
    x.abs()
}


/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
/// if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then performs some example
/// SPI transactions, then goes to sleep.
#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
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
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // These are implicitly used by the spi driver if they are in the correct mode
    // let _spi_sclk = pins.gpio6.into_mode::<hal::gpio::FunctionSpi>();
    // let _spi_mosi = pins.gpio7.into_mode::<hal::gpio::FunctionSpi>();
    // let _spi_miso = pins.gpio4.into_mode::<hal::gpio::FunctionSpi>();

    let _spi_sclk = pins.gpio18.into_mode::<hal::gpio::FunctionSpi>();
    let _spi_mosi = pins.gpio19.into_mode::<hal::gpio::FunctionSpi>();
    let _spi_miso = pins.gpio16.into_mode::<hal::gpio::FunctionSpi>();
    let spi = hal::Spi::<_, _, 8>::new(pac.SPI0);

    let mut lcd_led = pins.gpio17.into_push_pull_output();
    let mut led = pins.gpio25.into_push_pull_output();
    let dc = pins.gpio22.into_push_pull_output();
    let rst = pins.gpio26.into_push_pull_output();
    let w = pins.gpio5.into_pull_up_input();
    let a = pins.gpio6.into_pull_up_input();
    let s = pins.gpio7.into_pull_up_input();
    let d = pins.gpio8.into_pull_up_input();

    // Exchange the uninitialised SPI driver for an initialised one
    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        16.MHz(),
        &embedded_hal::spi::MODE_0,
    );


    let mut disp = st7735_lcd::ST7735::new(spi, dc, rst, true, false, 160, 128);

    disp.init(&mut delay).unwrap();
    disp.set_orientation(&Orientation::Landscape).unwrap();

    // let _ = disp.clear(to_color(PALETTE[0]));//Rgb565::BLACK);
    // disp.clear(Rgb565::BLACK).unwrap();
    // disp.set_offset(0, 25);

    // https://github.com/embedded-graphics/embedded-graphics
    // let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    // // Draw a triangle.
    // let yoffset = 0;
    //     Triangle::new(
    //         Point::new(16, 16 + yoffset),
    //         Point::new(16 + 16, 16 + yoffset),
    //         Point::new(16 + 8, yoffset),
    //     )
    //     .into_styled(thin_stroke)
    //     .draw(&mut disp)?;
    // Line::new(Point::new(50, 20), Point::new(60, 35))
    // .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 1))
    // .draw(&mut disp)
    // .unwrap();

    // let image_raw: ImageRawLE<Rgb565> =
    //     ImageRaw::new(include_bytes!("../../assets/ferris.raw"), 86);

    // let image: Image<_> = Image::new(&image_raw, Point::new(34, 8));

    // image.draw(&mut disp).unwrap();

    // disp.set_pixel(64, 64, Rgb565::RED.into_storage()).unwrap();
    
    // Wait until the background and image have been rendered otherwise
    // the screen will show random pixels for a brief moment
    lcd_led.set_high().unwrap();
    led.set_high().unwrap();

    let mut frame = 0;
    let mut gamepad: u8;
    loop {
        if frame % 10  == 0 {
            let _ = disp.clear(to_color(PALETTE[0]));
        }
        gamepad = 0;
        if w.is_low().unwrap() {
            gamepad |= BUTTON_UP;
        }
        if a.is_low().unwrap() {
            gamepad |= BUTTON_LEFT;
        }
        if s.is_low().unwrap() {
            gamepad |= BUTTON_DOWN;
        }
        if d.is_low().unwrap() {
            gamepad |= BUTTON_RIGHT;
        }
        unsafe {
           update(gamepad, &mut disp)
        }
        frame += 1;
    }
}

