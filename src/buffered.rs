
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::Rgb565,
    primitives::Rectangle,
    prelude::*,
};
use crate::{App, AppResult, Buttons, Error};
use embedded_graphics_framebuf::{FrameBuf, backends::FrameBufferBackend};

pub struct Buffer(pub [Rgb565; 20480]);

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
    app : A,
    pub interlace : Option<u8>,
    frame : u32,
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
            app,
            interlace: None,
            frame: 0,
        }
    }
}

impl<A> App for BufferedApp<A>
    where A : App {

    fn init(&mut self) -> AppResult {
        self.app.init()
    }

    fn update(&mut self, buttons: Buttons) -> AppResult {
        self.app.update(buttons)?;
        self.frame += 1;
        Ok(())
    }

    fn draw<T,E>(&mut self, display: &mut T) -> AppResult
        where T : DrawTarget<Color = Rgb565, Error = E> {
        self.app.draw(&mut self.frame_buf)?;

        match self.interlace {
            None => display.draw_iter(self.frame_buf.into_iter())
                           .map_err(|_| Error::DisplayErr),
            Some(k) => {
               let mut buf : [Rgb565; 160] = [Rgb565::BLACK; 160];
               for (jj, row) in self.frame_buf.data.0.chunks(160).enumerate() {
                   let j = jj as u8;
               // for j in 0..128 {
                   if (j % k) as u32 == self.frame % k as u32 {
                       buf.copy_from_slice(row);
                       display.fill_contiguous(&Rectangle { top_left: Point { x:0, y:j as i32 },
                                                            size: Size { width: 160, height: 1 } },
                                               // self.frame_buf.data.0[start..end])?;
                                               buf)
                           .map_err(|_| Error::DisplayErr)?;
                   }
               }
               Ok(())
            }
        }
    }
}

pub fn run<TheirApp,T,E,MyApp>(app : TheirApp) -> !
where T: DrawTarget<Color = Rgb565, Error = E>,
      TheirApp: App,
      MyApp: App,
{
    let buffered_app: BufferedApp<TheirApp> = BufferedApp::new(app);
    super::run(buffered_app);
}
