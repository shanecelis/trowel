// Example of running an ST7735 with an RP2040
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_graphics::{
    mono_font::{ascii, MonoTextStyle},
    pixelcolor::Rgb565,
    draw_target::DrawTarget,
    prelude::*,
    text::Text,
};
use trowel::{App, AppResult};

struct DrawFerris {
    frame : i32,                // Frame count
}

impl<T> App<T,Rgb565> for DrawFerris
    where T : DrawTarget<Color = Rgb565, Error = ()> {

    fn init(&mut self) -> AppResult {
        Ok(())
    }

    fn update(&mut self) -> AppResult {
        self.frame += 1;
        Ok(())
    }

    fn draw(&mut self, display: &mut T) -> AppResult {
        if self.frame == 1 {
            // Create a new character style
            let style = MonoTextStyle::new(&ascii::FONT_7X13, Rgb565::WHITE);

            Text::new("Hello, World!", Point::new(20, 30), style).draw(display)?;
        }
        Ok(())
    }
}

#[entry]
fn main() -> ! {
    trowel::run(&mut DrawFerris { frame : 0 });
}
