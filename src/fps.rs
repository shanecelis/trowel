use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb565, prelude::*};
use embedded_time::Clock;

use super::{App, AppResult, Buttons, Error};
use embedded_fps::FPS;
use embedded_graphics::{
    mono_font::{ascii::FONT_7X13, MonoTextStyle, MonoTextStyleBuilder},
    text::{Alignment, Text},
};

pub struct FpsApp<C>
where
    C: Clock,
{
    // #[cfg(all(target_arch = "arm", target_os = "none"))]
    //     fps_counter : FPS<100, MonotonicClock>,
    // #[cfg(not(all(target_arch = "arm", target_os = "none")))]
    //     fps_counter : FPS<100, embedded_fps::StdClock>,
    fps_counter: FPS<100, C>,
    style: MonoTextStyle<'static, Rgb565>,
}

impl<C> FpsApp<C>
where
    C: Clock,
{
    pub fn new(clock: C) -> Self {
        let style = MonoTextStyleBuilder::new()
            .font(&FONT_7X13)
            .text_color(Rgb565::WHITE)
            .background_color(Rgb565::BLACK)
            .build();
        let fps_counter = FPS::<100, _>::new(clock);
        // Ok(Self { fps_counter, style })
        Self { fps_counter, style }
    }
}

impl<C> App for FpsApp<C>
where
    C: Clock,
{
    fn init(&mut self) -> AppResult {
        Ok(())
    }

    fn update(&mut self, _buttons: Buttons) -> AppResult {
        Ok(())
    }

    fn draw<T, E>(&mut self, display: &mut T) -> AppResult
    where
        T: DrawTarget<Color = Rgb565, Error = E>,
    {
        // let character_style = MonoTextStyle::new(&FONT_7X13, Rgb565::WHITE);
        let fps_position = Point::new(155, 15);

        let fps = self.fps_counter.tick();
        Text::with_alignment(
            &format!("FPS: {fps}"),
            fps_position,
            self.style,
            Alignment::Right,
        )
        .draw(display)
        .map_err(|_| Error::DisplayErr)?;
        Ok(())
        // .expect("error on fps");
    }
}
