use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::{Rgb565, Rgb888, RgbColor}
};

use embedded_graphics_framebuf::{FrameBuf, backends::FrameBufferBackend};
extern crate console_error_panic_hook;

use std::cell::RefCell;
use std::rc::Rc;

use crate::{App, Buttons};
use web_sys::{Window, ImageData};
use wasm_bindgen::{
    prelude::*,
    Clamped};

use crate::FpsApp;
use try_default::TryDefault;
use embedded_time::{fraction::Fraction, Clock as EClock, clock::Error, Instant as EInstant};
use wasm_timer::Instant;

const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 128;

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
pub struct WasmClock(Instant);

// https://docs.rs/embedded-time/0.12.1/embedded_time/clock/trait.Clock.html
impl EClock for WasmClock {
    type T = u64;

    const SCALING_FACTOR: Fraction = Fraction::new(1, 1_000);

    fn try_now(&self) -> Result<EInstant<Self>, Error> {
        Ok(EInstant::<WasmClock>::new(self.0.elapsed().as_millis() as u64))
    }
}

impl TryDefault<FpsApp<WasmClock>> for FpsApp<WasmClock> {
    fn try_default() -> Option<Self> {
        Some(FpsApp::new(WasmClock(Instant::now())))
    }
}

pub fn run_with<F,A>(app_maker: F)
        where F : FnOnce() -> A, A : App + 'static {

    // if Some("1") == option_env!("SHOW_FPS") {
    //     _run_with(move || app_maker().join(FpsApp::default()));
    // } else {
        _run_with(app_maker);
    // }
}

fn _run_with<F,A>(app_maker: F)
        where F : FnOnce() -> A, A : App + 'static {

    let mut app = app_maker();
    app.init().expect("error initializing");
    setup::<A>(app).expect("error setting up app");
}

fn animation_frame(w: &Window, f: &Closure<dyn FnMut()>) {
    w.request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register 'requestAnimationFrame` OK");
}

fn setup<A>(mut app : A) -> Result<(), JsValue> where A : App + 'static {
    console_error_panic_hook::set_once();

    let data = WasmBuffer([0; OUTPUT_BUFFER_SIZE]);
    let mut display = FrameBuf::new(data, 160, 128);

    display
        .clear(Rgb565::BLACK)
        .expect("error clearing display");

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let canvas = document.get_element_by_id("trowel-canvas")
                         .map(|c| c.dyn_into::<web_sys::HtmlCanvasElement>().unwrap())
                         .or_else(|| {
        let canvas = document
            .create_element("canvas").ok()?
            .dyn_into::<web_sys::HtmlCanvasElement>().ok()?;
        canvas.set_id("trowel-canvas");
        // canvas.style().set_property("border", "solid")?;
        canvas.set_width(160);
        canvas.set_height(128);
        canvas.style().set_property("image-rendering", "pixelated").ok()?;
        canvas.style().set_property("image-rendering", "crisp-edges").ok()?;
        canvas.style().set_property("width", "100%").ok()?;
        canvas.style().set_property("height", "100%").ok()?;
        body.append_child(&canvas).ok()?;
        Some(canvas)
    }).expect("Cannot get or make canvas");

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    let buttons = Rc::new(RefCell::new(Buttons::empty()));
    {
        let buttons = buttons.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |e: web_sys::KeyboardEvent| {
            let c = char::from(e.key_code() as u8).to_ascii_uppercase();
            buttons.replace_with(move |buttons|
            match c {
                'W' => *buttons | Buttons::W,
                'A' => *buttons | Buttons::A,
                'S' => *buttons | Buttons::S,
                'D' => *buttons | Buttons::D,
                'I' => *buttons | Buttons::I,
                'J' => *buttons | Buttons::J,
                'K' => *buttons | Buttons::K,
                'L' => *buttons | Buttons::L,
                _ => *buttons
            });
        });
        document.add_event_listener_with_callback("keypress", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let buttons = buttons.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |e: web_sys::KeyboardEvent| {
            let c = char::from(e.key_code() as u8).to_ascii_uppercase();
            buttons.replace_with(move |buttons|
            match c {
                'W' => *buttons & !Buttons::W,
                'A' => *buttons & !Buttons::A,
                'S' => *buttons & !Buttons::S,
                'D' => *buttons & !Buttons::D,
                'I' => *buttons & !Buttons::I,
                'J' => *buttons & !Buttons::J,
                'K' => *buttons & !Buttons::K,
                'L' => *buttons & !Buttons::L,
                _ => *buttons
            });
        });
        document.add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    let init : Option<Closure<dyn FnMut()>> = None;
    let f = Rc::new(RefCell::new(init));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {

        app.update(buttons.borrow().clone()).expect("error updating");
        app.draw(&mut display).expect("error drawing");
        let image = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut display.data.0), 160, 128)
            .expect("Unable to make image data");
        context.put_image_data(&image, 0., 0.).expect("error putting image");

        // Schedule ourself for another requestAnimationFrame callback.
        // let b : RefCell<Option<Closure<dyn FnMut()>>>= *f;
        request_animation_frame(f.borrow().as_ref().unwrap());

    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
