// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb565, prelude::*, mock_display::MockDisplay};
use crate::{App, Buttons};

/// The `run` function configures the RP2040 peripherals, then runs the app.
pub fn run(
    app: &mut impl App<MockDisplay<Rgb565>, core::convert::Infallible>,
) -> ! {
    let mut disp : MockDisplay<Rgb565> = MockDisplay::new();

    // disp.set_orientation(&Orientation::Landscape).unwrap();
    disp.clear(Rgb565::BLACK).expect("error clearing display");
    disp.set_allow_overdraw(true);
    disp.set_allow_out_of_bounds_drawing(true);

    app.init().expect("error initializing");

    let mut buttons;
    loop {
        buttons = Buttons::empty();

        // if w.is_low().unwrap() {
        //     buttons |= Buttons::W;
        // }
        // if a.is_low().unwrap() {
        //     buttons |= Buttons::A;
        // }
        // if s.is_low().unwrap() {
        //     buttons |= Buttons::S;
        // }
        // if d.is_low().unwrap() {
        //     buttons |= Buttons::D;
        // }
        // if i.is_low().unwrap() {
        //     buttons |= Buttons::I;
        // }
        // if j.is_low().unwrap() {
        //     buttons |= Buttons::J;
        // }
        // if k.is_low().unwrap() {
        //     buttons |= Buttons::K;
        // }
        // if l.is_low().unwrap() {
        //     buttons |= Buttons::L;
        // }

        app.update(buttons).expect("error updating");
        app.draw(&mut disp).expect("error drawing");
    }
}
