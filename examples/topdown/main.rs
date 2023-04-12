#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]

use embedded_graphics::{
    draw_target::DrawTarget,
    image::{Image, ImageRaw},
    pixelcolor::Rgb565,
    prelude::*,
};
use embedded_graphics::pixelcolor::raw::BigEndian;
use heapless::Vec;
use tinybmp::Bmp;
use trowel::{App, AppResult, Buttons, Error};

const BMP_DATA: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/topdown/sprites/player.bmp"));

const SPRITE_DATA: &[(&str, u32, u32, u32, u32)] = &[
    ("sprite1", 18, 22, 13, 21),
    ("sprite2", 66, 22, 13, 21),
    ("sprite3", 114, 22, 13, 21),
    ("sprite4", 72, 22, 13, 20),
    ("sprite5", 162, 23, 13, 20),
    ("sprite6", 210, 23, 13, 20)
];
struct DrawFerris {
    frame: i32,
}

impl App for DrawFerris {
    fn init(&mut self) -> AppResult {
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
        let bmp: Bmp<Rgb565> = Bmp::from_slice(BMP_DATA).map_err(|_| Error::DisplayErr)?;
    
        let sprite = &SPRITE_DATA[self.frame as usize % SPRITE_DATA.len()];
        let mut vec: Vec<u8, 4096> = Vec::new();
        for y in sprite.2..sprite.2 + sprite.4 {
            for x in sprite.1..sprite.1 + sprite.3 {
                if let Some(pixel) = bmp.pixel(Point::new(x as i32, y as i32)) {
                    let color_bytes = pixel.to_be_bytes();
                    vec.push(color_bytes[0]).unwrap();
                    vec.push(color_bytes[1]).unwrap();
                }
            }
        }
        let image_raw = ImageRaw::<Rgb565, BigEndian>::new(vec.as_slice(), sprite.3);
    
        let image = Image::new(&image_raw, Point::zero());
        image.draw(display).map_err(|_| Error::DisplayErr)?;
    
        Ok(())
    }    
}

fn main() {
    trowel::run(DrawFerris { frame: 10 });
}

#[cfg_attr(all(target_arch = "arm", target_os = "none"), cortex_m_rt::entry)]
fn entry() -> ! {
    main();
    loop {}
}
