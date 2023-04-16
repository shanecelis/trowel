#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]

use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use trowel::{App, AppResult, Buttons, Error, OptionalFS, WriteMode, FS};

struct WriteFile {
    frame: i32, // Frame count

    was_successful: bool,
}

impl App for WriteFile {
    fn init<F: FS>(&mut self, fs: &mut OptionalFS<F>) -> AppResult {
        match fs {
            Some(fs) => {
                let file = fs.write_file("hello.txt", b"Hello, World!", WriteMode::Append);
                self.was_successful = file;
            }
            None => {
                self.was_successful = false;
            }
        }

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
            // Create a new character style
            let style = MonoTextStyle::new(&ascii::FONT_7X13, Rgb565::WHITE);

            Text::new(
                if self.was_successful {
                    "Wrote message"
                } else {
                    "No SD Card"
                },
                Point::new(20, 30),
                style,
            )
            .draw(display)
            .map_err(|_| Error::DisplayErr)?;
        }
        Ok(())
    }
}

fn main() {
    trowel::run(WriteFile {
        frame: 0,
        was_successful: false,
    });
}

#[cfg_attr(all(target_arch = "arm", target_os = "none"), cortex_m_rt::entry)]
fn entry() -> ! {
    main();
    loop {}
}
