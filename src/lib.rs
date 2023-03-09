#![no_std]

use bitflags::bitflags;
use embedded_graphics::prelude::DrawTarget;

bitflags! {
    pub struct Buttons: u8 {
        const W = 0b00000001;
        const A = 0b00000010;
        const S = 0b00000100;
        const D = 0b00001000;
        const I = 0b00010000;
        const J = 0b00100000;
        const K = 0b01000000;
        const L = 0b10000000;
    }
}

pub type AppResult = Result<(), ()>;

pub trait App<T, C>
where
    T: DrawTarget<Color = C>,
{
    fn init(&mut self) -> AppResult;
    fn update(&mut self, buttons: Buttons) -> AppResult;
    fn draw(&mut self, display: &mut T) -> AppResult;
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
mod sprig;
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use sprig::run;
