#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]

extern crate alloc;

use alloc::boxed::Box;
use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use trowel::{App, AppResult, Buttons, Error, FileSys, file_sys, WriteMode};
use genio::{Read, Write};

struct ReadFile {
    frame: i32, // Frame count

    file_contents: Box<str>,
}

impl App for ReadFile {
    fn init(&mut self) -> AppResult {
        Ok(())
    }

    fn update(&mut self, _buttons: Buttons) -> AppResult {
        self.frame += 1;

        if self.frame == 1 {

            let mut fs = file_sys().expect("Could not get file system");

            let mut file = fs.open_file("hello.txt", WriteMode::Truncate).expect("Unable to open file");
            file.write(b"What do we have here?")
                .expect("Unable to write file");
        } else if self.frame == 2 {

            let mut fs = file_sys().expect("Could not get file system");

            let mut file = fs.open_file("hello.txt", WriteMode::ReadOnly).expect("Unable to open file");
            let mut buffer = vec![0u8; 100];
            file.read(&mut buffer).expect("Unable to read file");
            // self.file_contents = Box::new(String::from_utf8(buffer)?);
            self.file_contents = Box::from(core::str::from_utf8(&buffer).unwrap());
        }
        // if let Some((size, file)) = file {
        //     self.file_contents = Box::from(core::str::from_utf8(&file[..size]).unwrap());
        // }

        Ok(())
    }

    fn draw<T, E>(&mut self, display: &mut T) -> AppResult
    where
        T: DrawTarget<Color = Rgb565, Error = E>,
    {
        if self.frame == 30 {
            // Create a new character style
            let style = MonoTextStyle::new(&ascii::FONT_7X13, Rgb565::WHITE);

            Text::new(&self.file_contents, Point::new(20, 30), style)
                .draw(display)
                .map_err(|_| Error::DisplayErr)?;
        }
        Ok(())
    }
}

#[trowel::entry]
fn main() {
    trowel::run(ReadFile {
        frame: 0,
        file_contents: Box::from("No filesystem"),
    });
}
