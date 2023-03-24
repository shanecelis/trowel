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

use embedded_fps::StdClock;
use super::FpsApp;

use crate::{App, Buttons};

pub fn init_heap() { }

pub mod buffered {
    pub use crate::pc::run;
}

impl Default for FpsApp<StdClock> {
    fn default() -> Self {
        FpsApp::new(StdClock::default())
    }
}

/// The `run` function configures the RP2040 peripherals, then runs the app.
pub fn run(app: impl App) -> ! {
    run_with(move || app);
}

pub fn run_with<F,A>(app_maker: F) -> !
        where F : FnOnce() -> A, A : App
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

    app.init().expect("error initializing");

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

        app.update(buttons).expect("error updating");
        app.draw(&mut display).expect("error drawing");
        window.update(&display);
    }
}
