#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]

use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use trowel::{App, AppResult, Buttons};

struct DrawFerris {
    frame: i32, // Frame count
}

impl<T, E> App<T, E> for DrawFerris
where
    T: DrawTarget<Color = Rgb565, Error = E>,
{
    fn init(&mut self) -> AppResult<E> {
        Ok(())
    }

    fn update(&mut self, _buttons: Buttons) -> AppResult<E> {
        self.frame += 1;
        Ok(())
    }

    fn draw(&mut self, display: &mut T) -> AppResult<E> {
        if self.frame == 1 {
            // Create a new character style
            let style = MonoTextStyle::new(&ascii::FONT_7X13, Rgb565::WHITE);

            Text::new("Hello, World!", Point::new(20, 30), style).draw(display)?;
        }
        Ok(())
    }
}

#[cfg_attr(all(target_arch = "arm", target_os = "none"), cortex_m_rt::entry)]
fn main() -> ! {
    trowel::run(&mut DrawFerris { frame: 0 });
}
