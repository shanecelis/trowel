/* Original code[1] by Shane Celis[2] Copyright (c) 2023 Hack Club[3]
   Licensed under the MIT License[4]

   [1]: https://github.com/shanecelis/trowel
   [2]: https://mastodon.gamedev.place/@shanecelis
   [3]: https://hackclub.com
   [4]: https://opensource.org/licenses/MIT
*/

#![no_std]

use bitflags::bitflags;
use embedded_graphics::{pixelcolor::Rgb565, prelude::DrawTarget};

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

pub type AppResult<E> = Result<(), E>;

pub trait App<T, E>
where
    T: DrawTarget<Color = Rgb565, Error = E>,
{
    fn init(&mut self) -> AppResult<E>;
    fn update(&mut self, buttons: Buttons) -> AppResult<E>;
    fn draw(&mut self, display: &mut T) -> AppResult<E>;
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
mod sprig;
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use sprig::run;

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
mod pc;
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
pub use pc::run;
