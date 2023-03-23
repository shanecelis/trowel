
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::Rgb565,
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
    where A : App
    // <FrameBuf<Rgb565, Buffer>,
    //           core::convert::Infallible>
{
    frame_buf : FrameBuf<Rgb565, Buffer>,
    // buffer : Buffer,
    app : A
}


impl<A> BufferedApp<A>
    where A : App
    // <FrameBuf<Rgb565, Buffer>,
    //           core::convert::Infallible>
{
    pub fn new(app: A) -> Self {
        let data = Buffer([Rgb565::BLACK; 20480]);
        BufferedApp {
            // buffer: data,
            frame_buf: FrameBuf::new(data, 160, 128),
            app
        }
    }
}

impl<A> App for BufferedApp<A>
where
    A : App
{
    fn init(&mut self) -> AppResult {
        self.app.init();
        Ok(())
    }

    fn update(&mut self, buttons: Buttons) -> AppResult {
        self.app.update(buttons);
        Ok(())
    }

    fn draw<T,E>(&mut self, display: &mut T) -> AppResult
        where T : DrawTarget<Color = Rgb565, Error = E> {
        self.app.draw(&mut self.frame_buf);

        display.draw_iter(self.frame_buf.into_iter());
        Ok(())
    }
}

pub fn run<TheirApp,T,E,MyApp>(app : TheirApp) -> !
where T: DrawTarget<Color = Rgb565, Error = E>,
      TheirApp: App,
      MyApp: App,
{
    let mut buffered_app: BufferedApp<TheirApp> = BufferedApp::new(app);
    super::run(&mut buffered_app);
}
