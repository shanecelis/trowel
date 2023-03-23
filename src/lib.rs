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

#[derive(Debug)]
pub enum Error {
    DisplayErr,
    AppErr
}
pub type AppResult = Result<(), Error>;

pub trait App
{
    fn init(&mut self) -> AppResult;
    fn update(&mut self, buttons: Buttons) -> AppResult;
    fn draw<T,E>(&mut self, display: &mut T) -> AppResult
        where T: DrawTarget<Color = Rgb565, Error = E>;
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
mod sprig;
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use sprig::{run, init_heap};

// #[cfg(all(target_arch = "arm", target_os = "none"))]
// pub mod buffered;


pub mod buffered;

// pub mod buffered {
// #[cfg(all(target_arch = "arm", target_os = "none"))]
//     pub use sprig_buffered::run;

// #[cfg(not(all(target_arch = "arm", target_os = "none")))]
//     pub use crate::pc::run;
// }

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
mod pc;
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
pub use pc::{run, init_heap};

#[cfg(feature = "runty8")]
pub mod runty8;
