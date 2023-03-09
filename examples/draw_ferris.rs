#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]

use embedded_graphics::{
    draw_target::DrawTarget,
    image::{Image, ImageRaw, ImageRawLE},
    pixelcolor::Rgb565,
    prelude::*,
};
use trowel::{App, AppResult, Buttons};

struct DrawFerris {
    /// Frame count
    frame: i32,
}

impl<T,E> App<T, E> for DrawFerris
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
            // We only need to draw the image once for it to persist.

            let image_raw: ImageRawLE<Rgb565> =
                ImageRaw::new(include_bytes!("../assets/ferris.raw"), 86);

            let image: Image<_> = Image::new(&image_raw, Point::new(34, 33));

            image.draw(display)?;
        }
        Ok(())
    }
}

#[cfg_attr(all(target_arch = "arm", target_os = "none"), cortex_m_rt::entry)]
fn main() -> ! {
    trowel::run(&mut DrawFerris { frame: 0 });
}
