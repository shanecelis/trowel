
use embedded_graphics::{
    draw_target::DrawTarget,
    image::{Image, ImageRaw, ImageRawBE},
    pixelcolor::{Rgb565, Rgb888},
    prelude::*,
};
use crate::{App, AppResult, Buttons};
use embedded_graphics_framebuf::{FrameBuf, backends::FrameBufferBackend};

pub struct Buffer([Rgb565; 20480]);

impl FrameBufferBackend for Buffer {
    type Color = Rgb565;
    fn set(&mut self, index: usize, color: Self::Color) {
        self.0[index] = color;
    }

    fn get(&self, index: usize) -> Self::Color {
        self.0[index]
    }

    fn nr_elements(&self) -> usize {
        20480
    }
}

pub struct BufferedApp<A>
    where A : App<FrameBuf<Rgb565, Buffer>,
              core::convert::Infallible>
{
    frameBuf : FrameBuf<Rgb565, Buffer>,
    // buffer : Buffer,
    app : A
}


impl<A> BufferedApp<A>
    where A : App<FrameBuf<Rgb565, Buffer>,
              core::convert::Infallible>
{
    pub fn new(app: A) -> Self {
        let data = Buffer([Rgb565::BLACK; 20480]);
        BufferedApp {
            // buffer: data,
            frameBuf: FrameBuf::new(data, 160, 128),
            app
        }
    }
}

impl<T, E, A> App<T, E> for BufferedApp<A>
where
    T : DrawTarget<Color = Rgb565, Error = E>,
    A : App<FrameBuf<Rgb565, Buffer>, core::convert::Infallible>
{
    fn init(&mut self) -> AppResult<E> {
        self.app.init();
        Ok(())
    }

    fn update(&mut self, buttons: Buttons) -> AppResult<E> {
        self.app.update(buttons);
        Ok(())
    }

    fn draw(&mut self, display: &mut T) -> AppResult<E> {
        self.app.draw(&mut self.frameBuf);

        display.draw_iter(self.frameBuf.into_iter());
        Ok(())
    }
}

pub fn run<TheirApp,T,E,MyApp>(app : TheirApp) -> !
where T: DrawTarget<Color = Rgb565, Error = E>,
      TheirApp: App<FrameBuf<Rgb565, Buffer>,
              core::convert::Infallible>,
      MyApp: App<T,E>
{
    let mut bufferedApp: BufferedApp<TheirApp> = BufferedApp::new(app);
    super::run(&mut bufferedApp);
}
