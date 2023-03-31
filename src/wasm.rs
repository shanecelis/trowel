use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::{Rgb565, Rgb888, RgbColor}
};

use embedded_graphics_framebuf::{FrameBuf, backends::FrameBufferBackend};
extern crate console_error_panic_hook;

const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 128;

use crate::{App, Buttons};

/*
 * 1. What is going on here?
 * Create a static mutable byte buffer.
 * We will use for putting the output of our graphics,
 * to pass the output to js.
 * NOTE: global `static mut` means we will have "unsafe" code
 * but for passing memory between js and wasm should be fine.
 *
 * 2. Why is the size SCREEN_WIDTH * SCREEN_WIDTH * 4?
 * We want to have 20 pixels by 20 pixels. And 4 colors per pixel (r,g,b,a)
 * Which, the Canvas API Supports.
 */
const OUTPUT_BUFFER_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT * 4;

pub struct WasmBuffer([u8; OUTPUT_BUFFER_SIZE]);

impl FrameBufferBackend for WasmBuffer {
    type Color = Rgb565;
    fn set(&mut self, index: usize, color: Self::Color) {
        let i = index * 4;
        let c = Rgb888::from(color);
        self.0[i + 0] = c.r();
        self.0[i + 1] = c.g();
        self.0[i + 2] = c.b();
        self.0[i + 3] = 255;
    }

    fn get(&self, index: usize) -> Self::Color {
        let i = index * 4;
        let c = {
            let r = self.0[i + 0];
            let g = self.0[i + 1];
            let b = self.0[i + 2];

            Rgb888::new(r, g, b)
        };
        Rgb565::from(c)
    }

    fn nr_elements(&self) -> usize {
        SCREEN_HEIGHT * SCREEN_WIDTH
    }
}


use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

use embedded_fps::StdClock;
use crate::FpsApp;
use try_default::TryDefault;

impl TryDefault<FpsApp<StdClock>> for FpsApp<StdClock> {
    fn try_default() -> Option<Self> {
        None
    }
}

pub fn run_with<F,A>(app_maker: F)
        where F : FnOnce() -> A, A : App {

    // if Some("1") == option_env!("SHOW_FPS") {
    //     _run_with(move || app_maker().join(FpsApp::default()));
    // } else {
        _run_with(app_maker);
    // }
}

fn _run_with<F,A>(app_maker: F)
        where F : FnOnce() -> A, A : App {

    let mut app = app_maker();

    app.init().expect("error initializing");
    let mut buttons = Buttons::empty();
    setup(app).expect("error setting up app");

    // // 'running: loop {
    // loop {
    //     window.update(&display);
    //     // BUG: This seems to hang on macOS if window.events() is not called.
    //     for event in window.events() {
    //         match event {
    //             SimulatorEvent::KeyDown { keycode, .. } => match keycode {
    //                 Keycode::W => buttons |= Buttons::W,
    //                 Keycode::A => buttons |= Buttons::A,
    //                 Keycode::S => buttons |= Buttons::S,
    //                 Keycode::D => buttons |= Buttons::D,
    //                 Keycode::I => buttons |= Buttons::I,
    //                 Keycode::J => buttons |= Buttons::J,
    //                 Keycode::K => buttons |= Buttons::K,
    //                 Keycode::L => buttons |= Buttons::L,
    //                 _ => {}
    //             },

    //             SimulatorEvent::KeyUp { keycode, .. } => match keycode {
    //                 Keycode::W => buttons &= !Buttons::W,
    //                 Keycode::A => buttons &= !Buttons::A,
    //                 Keycode::S => buttons &= !Buttons::S,
    //                 Keycode::D => buttons &= !Buttons::D,
    //                 Keycode::I => buttons &= !Buttons::I,
    //                 Keycode::J => buttons &= !Buttons::J,
    //                 Keycode::K => buttons &= !Buttons::K,
    //                 Keycode::L => buttons &= !Buttons::L,
    //                 _ => {}
    //             },
    //             SimulatorEvent::Quit => panic!("quit"), //break 'running,
    //             _ => {}
    //         }
    //     }

    //     app.update(buttons).expect("error updating");
    //     app.draw(&mut display).expect("error drawing");
    //     window.update(&display);
    // }
}

// Main is called when the wasm module is instantiated.
// #[wasm_bindgen(start)]
fn setup<A>(app : A) -> Result<(), JsValue> where A : App{
    console_error_panic_hook::set_once();

    let mut data = WasmBuffer([0; OUTPUT_BUFFER_SIZE]);
    let mut display = FrameBuf::new(data, 160, 128);

    display
        .clear(Rgb565::BLACK)
        .expect("error clearing display");

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append.
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    body.append_child(&canvas)?;
    canvas.set_width(160);
    canvas.set_height(128);
    canvas.style().set_property("border", "solid")?;
    canvas.style().set_property("image-rendering", "pixelated")?;
    canvas.style().set_property("image-rendering", "crisp-edges")?;
    canvas.style().set_property("width", "100%")?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    let context = Rc::new(context);
    let body = Rc::new(body);
    let document = Rc::new(document);
    let pressed = Rc::new(Cell::new(false));
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            context.begin_path();
            context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            pressed.set(true);
        });
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            if pressed.get() {
                context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                context.stroke();
                context.begin_path();
                context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            }
        });
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            pressed.set(false);
            context.line_to(event.offset_x() as f64, event.offset_y() as f64);
            context.stroke();
        });
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let body = body.clone();
        let documentC = document.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |e: web_sys::KeyboardEvent| {
            let c = char::from(e.key_code() as u8).to_ascii_lowercase();
            match c {
                'w' => {
            let p = documentC.create_element("p").unwrap();
            p.set_inner_html("w");
            body.append_child(&p).unwrap();
                }
                _ => {
            let p = documentC.create_element("p").unwrap();
            p.set_inner_html("wtf!");
            body.append_child(&p).unwrap();
                }

            };
            });
        document.add_event_listener_with_callback("keypress", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
