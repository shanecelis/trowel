#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]

use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use trowel::{App, AppResult, Buttons, Error, WriteMode, FileSys, file_sys};
use genio::Write;

struct WriteFile {
    frame: i32, // Frame count

    was_successful: bool,
}

impl App for WriteFile {

    fn init(&mut self) -> AppResult {
        Ok(())
    }

    fn update(&mut self, _buttons: Buttons) -> AppResult {
        self.frame += 1;

        if self.frame != 1 {
            return Ok(());
        }
        let fs = file_sys().expect("Could not get file system");

        // if fs.file_exists("hello.txt").expect("Could not see if file exists") {
        //     println!("I see the file");
        // }
        // let file = fs.write_file("hello.txt", b"Hello, FS!\n", WriteMode::Append);
        let mut file = fs.open_file("hello.txt", WriteMode::Truncate)
                         .expect("Could not open file");
        file.write(b"Hello, FS!\n")
            .expect("Could not write file");
        self.was_successful = true;

        Ok(())
    }

    fn draw<T, E>(&mut self, display: &mut T) -> AppResult
    where
        T: DrawTarget<Color = Rgb565, Error = E>,
    {
        if self.frame == 30 {
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

#[trowel::entry]
fn main() {
    trowel::run(WriteFile {
        frame: 0,
        was_successful: false,
    });
}
