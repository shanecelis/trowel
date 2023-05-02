use crate::{App, AppResult, Buttons, Error};
use embedded_graphics::{
    draw_target::DrawTarget, pixelcolor::Rgb565, prelude::*, primitives::Rectangle,
};
use embedded_graphics_framebuf::{backends::FrameBufferBackend, FrameBuf};

pub struct Buffer {
    pub data: [Rgb565; 20480],
    pub transparent: Option<Rgb565>,
}

impl FrameBufferBackend for Buffer {
    type Color = Rgb565;
    fn set(&mut self, index: usize, color: Self::Color) {
        if let Some(trans) = self.transparent {
            if trans == color {
                // Don't set color.
                return;
            }
        }
        self.data[index] = color;
    }

    fn get(&self, index: usize) -> Self::Color {
        self.data[index]
    }

    fn nr_elements(&self) -> usize {
        20480
    }
}

pub struct BufferedApp<A, B = Buffer, C = Rgb565>
where
    A: App,
    B: FrameBufferBackend<Color = C>,
    C: PixelColor,
{
    frame_buf: FrameBuf<C, B>,
    // buffer : Buffer,
    app: A,
    pub interlace: Option<u8>,
    frame: u32,
    last_buttons: Buttons,
    pub increase_button: Option<Buttons>,
    pub decrease_button: Option<Buttons>,
}

impl<A> BufferedApp<A>
where
    A: App,
{
    pub fn new(app: A) -> Self {
        let data = Buffer {
            data: [Rgb565::BLACK; 20480],
            transparent: None,
        };
        BufferedApp {
            frame_buf: FrameBuf::new(data, 160, 128),
            app,
            interlace: None,
            frame: 0,
            last_buttons: Buttons::empty(),
            increase_button: None,
            decrease_button: None,
        }
    }

    fn increase(&mut self) {
        self.interlace = match self.interlace {
            Some(n) => Some(n + 1),
            None => Some(2),
        }
    }

    fn decrease(&mut self) {
        self.interlace = match self.interlace {
            Some(2) => None,
            Some(n) => Some(n - 1),
            None => None,
        }
    }
}

impl<A> App for BufferedApp<A>
where
    A: App,
{
    fn init(&mut self) -> AppResult {
        self.app.init()
    }

    fn update(&mut self, buttons: Buttons) -> AppResult {
        self.app.update(buttons)?;
        self.frame += 1;
        if let Some(inc_b) = self.increase_button {
            if buttons.contains(inc_b) && !self.last_buttons.contains(inc_b) {
                self.increase()
            }
        }

        if let Some(dec_b) = self.decrease_button {
            if buttons.contains(dec_b) && !self.last_buttons.contains(dec_b) {
                self.decrease()
            }
        }
        self.last_buttons = buttons;
        Ok(())
    }

    fn draw<T, E>(&mut self, display: &mut T) -> AppResult
    where
        T: DrawTarget<Color = Rgb565, Error = E>,
    {
        self.app.draw(&mut self.frame_buf)?;

        match self.interlace {
            None => display
                .draw_iter(self.frame_buf.into_iter())
                .map_err(|_| Error::DisplayErr),
            Some(k) => {
                let mut buf: [Rgb565; 160] = [Rgb565::BLACK; 160];
                for (jj, row) in self.frame_buf.data.data.chunks(160).enumerate() {
                    let j = jj as u8;
                    // for j in 0..128 {
                    if (j % k) as u32 == self.frame % k as u32 {
                        buf.copy_from_slice(row);
                        display
                            .fill_contiguous(
                                &Rectangle {
                                    top_left: Point { x: 0, y: j as i32 },
                                    size: Size {
                                        width: 160,
                                        height: 1,
                                    },
                                },
                                // self.frame_buf.data.0[start..end])?;
                                buf,
                            )
                            .map_err(|_| Error::DisplayErr)?;
                    }
                }
                Ok(())
            }
        }
    }
}
