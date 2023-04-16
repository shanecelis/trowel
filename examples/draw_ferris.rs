/* Original code[1] Copyright (c) 2021 Andrew Christiansen[2]
   Modified code[3] by Shane Celis[4] Copyright (c) 2023 Hack Club[5]
   Licensed under the MIT License[6]

   [1]: https://github.com/sajattack/st7735-lcd-examples/blob/master/rp2040-examples/examples/draw_ferris.rs
   [2]: https://github.com/DrewTChrist
   [3]: https://github.com/shanecelis/trowel/blob/master/examples/draw_ferris.rs
   [4]: https://mastodon.gamedev.place/@shanecelis
   [5]: https://hackclub.com
   [6]: https://opensource.org/licenses/MIT
*/

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]

use embedded_graphics::{
    draw_target::DrawTarget,
    image::{Image, ImageRaw, ImageRawLE},
    pixelcolor::Rgb565,
    prelude::*,
};
use trowel::{App, AppResult, Buttons, Error, OptionalFS, FS};

struct DrawFerris {
    /// Frame count
    frame: i32,
}

impl App for DrawFerris {
    fn init<F: FS>(&mut self, _fs: &mut OptionalFS<F>) -> AppResult {
        Ok(())
    }

    fn update<F: FS>(&mut self, _buttons: Buttons, _fs: &mut OptionalFS<F>) -> AppResult {
        self.frame += 1;
        Ok(())
    }

    fn draw<T, E>(&mut self, display: &mut T) -> AppResult
    where
        T: DrawTarget<Color = Rgb565, Error = E>,
    {
        if self.frame == 1 {
            // We only need to draw the image once for it to persist.

            let image_raw: ImageRawLE<Rgb565> =
                ImageRaw::new(include_bytes!("../assets/ferris.raw"), 86);

            let image: Image<_> = Image::new(&image_raw, Point::new(34, 33));

            image.draw(display).map_err(|_| Error::DisplayErr)?;
        }
        Ok(())
    }
}

fn main() {
    trowel::run(DrawFerris { frame: 0 });
}

#[cfg_attr(all(target_arch = "arm", target_os = "none"), cortex_m_rt::entry)]
fn entry() -> ! {
    main();
    loop {}
}
