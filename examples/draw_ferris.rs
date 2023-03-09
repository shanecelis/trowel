// Example of running an ST7735 with an RP2040
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_graphics::image::{Image, ImageRaw, ImageRawLE};
use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::draw_target::DrawTarget;
use trowel::App;

struct DrawFerris {
    frame : i32,                // Frame count
}

impl<T> App<T,Rgb565> for DrawFerris
    where T : DrawTarget<Color = Rgb565, Error = ()> {

    fn init(&mut self) {
    }

    fn update(&mut self) {
        self.frame += 1;
    }

    fn draw(&mut self, display: &mut T) {
        if self.frame == 1 {
            // We only need to draw the image once for it to persist.

            let image_raw: ImageRawLE<Rgb565> =
                ImageRaw::new(include_bytes!("../assets/ferris.raw"), 86);

            let image: Image<_> = Image::new(&image_raw, Point::new(34, 33));

            image.draw(display).unwrap();
        }
    }
}

#[entry]
fn main() -> ! {
    trowel::run(&mut DrawFerris { frame : 0 });
}
