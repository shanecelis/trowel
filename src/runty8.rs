//! Run a standalone Runty8 game natively or in wasm.

use runty8_core as runty;
use runty::{Input, Pico8, Resources};//, KeyboardEvent, InputEvent, Key, KeyState};

use embedded_graphics::{
    draw_target::DrawTarget,
    image::{Image, ImageRaw, ImageRawBE},
    pixelcolor::{Rgb565, Rgb888},
    prelude::*,
};
use crate::{App, AppResult, Buttons, Error};

struct RuntyApp<G>
    where G : runty::App + 'static
{
    pico8: Pico8,
    game: G,
    input: Input,
    last_buttons: Buttons
}

impl<G> RuntyApp<G>
    where G : runty::App + 'static {
    fn new(resources: Resources) -> Self{
        let mut pico8 = Pico8::new(resources);
        let game = G::init(&mut pico8);
        RuntyApp {
            pico8: pico8,
            game: game,
            input: Input::new(),
            last_buttons: Buttons::empty()
        }
    }

    // fn handle_event(&mut self, button: Buttons, buttons: Buttons, key : Key) {
    //     let down = buttons.contains(button);
    //     if down { //^ self.last_buttons.contains(button) {
    //         self.input.on_event(InputEvent::Keyboard(KeyboardEvent { key,
    //                                                   state: if down { KeyState::Down } else { KeyState::Up } }));
    //     }
    // }
}

impl<G> App for RuntyApp<G>
where
    G: runty::App + 'static,
{
    fn init(&mut self) -> AppResult {
        Ok(())
    }

    fn update(&mut self, buttons: Buttons) -> AppResult {

        // self.handle_event(Buttons::W, buttons, Key::W);
        // self.handle_event(Buttons::A, buttons, Key::A);
        // self.handle_event(Buttons::S, buttons, Key::S);
        // self.handle_event(Buttons::D, buttons, Key::D);
        // self.handle_event(Buttons::I, buttons, Key::I);
        // self.handle_event(Buttons::J, buttons, Key::J);
        // self.handle_event(Buttons::K, buttons, Key::K);
        // self.handle_event(Buttons::L, buttons, Key::L);
        self.input.up = Some(buttons.contains(Buttons::W));
        self.input.left = Some(buttons.contains(Buttons::A));
        self.input.down = Some(buttons.contains(Buttons::S));
        self.input.right = Some(buttons.contains(Buttons::D));
        self.input.x = Some(buttons.contains(Buttons::L));
        self.input.c = Some(buttons.contains(Buttons::K));
        self.pico8.state.update_input(&self.input);
        self.game.update(&mut self.pico8);
        self.last_buttons = buttons;
        Ok(())
    }

    fn draw<T,E>(&mut self, display: &mut T) -> AppResult
        where T: DrawTarget<Color = Rgb565, Error = E>
    {
        self.game.draw(&mut self.pico8);
        let mut rgb565s: [u8 ; 128 * 128 * 2 ] = [ 0; 128 * 128 * 2 ];
        let rgb888s = self.pico8.draw_data.buffer();
        for (i, rgb) in rgb888s.chunks(3).map(|rgb| Rgb565::from(Rgb888::new(rgb[0], rgb[1], rgb[2]))).enumerate() {
            let j = i * 2;
            let x : u16 = rgb.into_storage();
            rgb565s[j] = (x >> 8) as u8;
            rgb565s[j + 1] = (x & 0xff) as u8;
        }
        let raw: ImageRawBE<Rgb565> = ImageRaw::new(&rgb565s, 128);

        let image: Image<_> = Image::new(&raw, Point::new(16, 0));
        image.draw(display).map_err(|_| Error::DisplayErr)?;
        // raw.draw(display)?;
        Ok(())
    }
}

pub fn run<Game: runty::App + 'static>(resources: Resources) -> ! {
    let mut game: RuntyApp<Game> = RuntyApp::new(resources);
    super::run(&mut game);
}

