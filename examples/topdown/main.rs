#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]

use embedded_graphics::{
    draw_target::DrawTarget,
    image::{Image, ImageRaw},
    pixelcolor::Rgb565,
    primitives::Rectangle,
    prelude::*,
};
use embedded_graphics::pixelcolor::raw::BigEndian;
use heapless::Vec;
use tinybmp::Bmp;
use trowel::{App, AppResult, Buttons, Error};
use micromath::F32Ext;

const BMP_DATA: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/topdown/sprites/player3.bmp"));

// const SPRITE_DATA: &[(&str, u32, u32, u32, u32)] = &[
//     ("sprite1", 18, 22, 13, 21),
//     ("sprite2", 66, 22, 13, 21),
//     ("sprite3", 114, 22, 13, 21),
//     ("sprite4", 72, 22, 13, 20),
//     ("sprite5", 162, 23, 13, 20),
//     ("sprite6", 210, 23, 13, 20)
// ];

fn sprite_data(i: usize) -> (&'static str, i32, i32, u32, u32) {
   // let name = format!("sprite{i}");
   ("dummy_name", i as i32 * 16, 0, 16, 32)
}

const SPRITE_COUNT: usize = 60;

struct TopDown {
    frame: i32,
    bmp: Option<Bmp<'static, Rgb565>>,
}

impl App for TopDown {
    fn init(&mut self) -> AppResult {
        // Let's parse the bmp data once. There may be some internal allocations that
        // caused it to crash after running for a while with this in our draw loop.
        self.bmp = Some(Bmp::from_slice(BMP_DATA).map_err(|e| Error::BmpErr(e))?);
        Ok(())
    }

    fn update(&mut self, _buttons: Buttons) -> AppResult {
        self.frame += 1;
        Ok(())
    }

    fn draw<T, E>(&mut self, display: &mut T) -> AppResult
    where
        T: DrawTarget<Color = Rgb565, Error = E>,
    {
        let nth_frame = 15;
        if self.frame % nth_frame != 0 {
            return Ok(());
        }
        let sprite_index = (self.frame as f32 / nth_frame as f32).floor();

        let sprite = &sprite_data(sprite_index as usize % SPRITE_COUNT);
        let at = Point::new((160 - sprite.3 as i32) / 2, (128 - sprite.4 as i32) / 2);
        self.bmp
            .expect("no bmp set")
            .draw_sub_image(&mut display.translated(at),
                            &Rectangle::new(Point::new(sprite.1, sprite.2), Size::new(sprite.3, sprite.4)))
            .map_err(|_| Error::DisplayErr)?;

        // let mut vec: Vec<u8, 4096> = Vec::new();
        // for y in sprite.2..sprite.2 + sprite.4 {
        //     for x in sprite.1..sprite.1 + sprite.3 {
        //         if let Some(pixel) = self.bmp.expect("no bmp set").pixel(Point::new(x as i32, y as i32)) {
        //             let color_bytes = pixel.to_be_bytes();
        //             vec.push(color_bytes[0]).unwrap();
        //             vec.push(color_bytes[1]).unwrap();
        //         }
        //     }
        // }
        // let image_raw = ImageRaw::<Rgb565, BigEndian>::new(vec.as_slice(), sprite.3);

        // let image = Image::new(&image_raw, Point::zero());
        // image.draw(display).map_err(|_| Error::DisplayErr)?;
    
        Ok(())
    }    
}

#[trowel::entry]
fn main() {
    trowel::run(TopDown { frame: 10, bmp: None });
}
