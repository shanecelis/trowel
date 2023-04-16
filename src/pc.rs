/* Original code[1] by Shane Celis[2] Copyright (c) 2023 Hack Club[3]
   Licensed under the MIT License[4]

   [1]: https://github.com/shanecelis/trowel
   [2]: https://mastodon.gamedev.place/@shanecelis
   [3]: https://hackclub.com
   [4]: https://opensource.org/licenses/MIT
*/

use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{
    sdl2::Keycode, BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent,
    Window,
};

use self::fs::PCFS;

use super::{AppExt, FpsApp};
use embedded_fps::StdClock;

use crate::{App, Buttons, OptionalFS};

mod fs;

impl Default for FpsApp<StdClock> {
    fn default() -> Self {
        FpsApp::new(StdClock::default())
    }
}

pub fn run_with<F, A>(app_maker: F) -> !
where
    F: FnOnce() -> A,
    A: App + 'static,
{
    if Some("1") == option_env!("SHOW_FPS") {
        _run_with(move || app_maker().join(FpsApp::default()));
    } else {
        _run_with(app_maker);
    }
}

fn _run_with<F, A>(app_maker: F) -> !
where
    F: FnOnce() -> A,
    A: App + 'static,
{
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(160, 128));

    display
        .clear(Rgb565::BLACK)
        .expect("error clearing display");

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::Default)
        .build();
    let mut window = Window::new("Sprig Simulator", &output_settings);
    let mut app = app_maker();
    let mut fs = fs::PCFS::new();
    let mut fs: OptionalFS<PCFS> = Some(&mut fs).into();

    // if Some("1") == option_env!("SHOW_FPS") {
    //     app = app.join(FpsApp::default());
    // }
    app.init(&mut fs).expect("error initializing");

    let mut buttons = Buttons::empty();
    // 'running: loop {
    loop {
        window.update(&display);
        // BUG: This seems to hang on macOS if window.events() is not called.
        for event in window.events() {
            match event {
                SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                    Keycode::W => buttons |= Buttons::W,
                    Keycode::A => buttons |= Buttons::A,
                    Keycode::S => buttons |= Buttons::S,
                    Keycode::D => buttons |= Buttons::D,
                    Keycode::I => buttons |= Buttons::I,
                    Keycode::J => buttons |= Buttons::J,
                    Keycode::K => buttons |= Buttons::K,
                    Keycode::L => buttons |= Buttons::L,
                    _ => {}
                },

                SimulatorEvent::KeyUp { keycode, .. } => match keycode {
                    Keycode::W => buttons &= !Buttons::W,
                    Keycode::A => buttons &= !Buttons::A,
                    Keycode::S => buttons &= !Buttons::S,
                    Keycode::D => buttons &= !Buttons::D,
                    Keycode::I => buttons &= !Buttons::I,
                    Keycode::J => buttons &= !Buttons::J,
                    Keycode::K => buttons &= !Buttons::K,
                    Keycode::L => buttons &= !Buttons::L,
                    _ => {}
                },
                SimulatorEvent::Quit => panic!("quit"), //break 'running,
                _ => {}
            }
        }

        app.update(buttons, &mut fs).expect("error updating");
        app.draw(&mut display).expect("error drawing");
        window.update(&display);
    }
}
