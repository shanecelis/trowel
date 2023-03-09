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
use embedded_hal::digital::v2::OutputPin;
use rp2040_hal::clocks::Clock;
use st7735_lcd;
use st7735_lcd::Orientation;
use fugit::RateExtU32;

use embedded_graphics::draw_target::{DrawTarget, DrawTargetExt};

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use trowel::App;

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then performs some example
/// SPI transactions, then goes to sleep.

struct DrawFerris {
    frame : i32,
}

impl<T> App<T,Rgb565> for DrawFerris
    where T : DrawTarget<Color = Rgb565, Error = ()> {
    fn init(&mut self) {
        self.frame = 0;
    }

    fn update(&mut self) {
        self.frame += 1;
    }

    fn draw(&mut self, display: &mut T) {
        if self.frame == 0 {
            // display.set_offset(0, 25);

            let image_raw: ImageRawLE<Rgb565> =
                ImageRaw::new(include_bytes!("../assets/ferris.raw"), 86);

            let image: Image<_> = Image::new(&image_raw, Point::new(34, 8));

            image.draw(display).unwrap();
        }
    }

}

#[entry]
fn main() -> ! {
    trowel::run(&mut DrawFerris { frame : 0 });
}

// End of file
