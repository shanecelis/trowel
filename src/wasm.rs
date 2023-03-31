use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::{Rgb565, Rgb888, RgbColor}
};

use embedded_graphics_framebuf::{FrameBuf, backends::FrameBufferBackend};
extern crate console_error_panic_hook;

use std::cell::RefCell;
use std::rc::Rc;
// use std::borrow::Borrow;

const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 128;

use crate::{App, Buttons};
use web_sys::{Window, ImageData};

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


use wasm_bindgen::{
    prelude::*,
    Clamped};

use embedded_fps::StdClock;
use crate::FpsApp;
use try_default::TryDefault;

impl TryDefault<FpsApp<StdClock>> for FpsApp<StdClock> {
    fn try_default() -> Option<Self> {
        None
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

    //     app.update(buttons).expect("error updating");
    //     app.draw(&mut display).expect("error drawing");
    //     window.update(&display);
    // }
}

fn animation_frame(w: &Window, f: &Closure<dyn FnMut()>) {
    w.request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register 'requestAnimationFrame` OK");
}

// #[wasm_bindgen]
// Main is called when the wasm module is instantiated.
// #[wasm_bindgen(start)]
fn setup<A>(mut app : A) -> Result<(), JsValue> where A : App + 'static {
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
    let buttons = Rc::new(RefCell::new(Buttons::empty()));
    // let context = Rc::new(context);
    let body = Rc::new(body);
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
    // {
    // // Here we want to call `requestAnimationFrame` in a loop, but only a fixed
    // // number of times. After it's done we want all our resources cleaned up. To
    // // achieve this we're using an `Rc`. The `Rc` will eventually store the
    // // closure we want to execute on each frame, but to start out it contains
    // // `None`.
    // //
    // // After the `Rc` is made we'll actually create the closure, and the closure
    // // will reference one of the `Rc` instances. The other `Rc` reference is
    // // used to store the closure, request the first frame, and then is dropped
    // // by this function.
    // //
    // // Inside the closure we've got a persistent `Rc` reference, which we use
    // // for all future iterations of the loop
    // // let f : Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    // let f = Rc::new(RefCell::new(None));
    // let g = f.clone();
    // let buttons = buttons.clone();
    // let window = Rc::new(window);
    // let win = window.clone();

    // // let mut i = 0;
    // *g.borrow_mut() = Some(Closure::new(Box::new(move || {
    //     // if i > 300 {
    //     //     body().set_text_content(Some("All done!"));

    //     //     // Drop our handle to this closure so that it will get cleaned
    //     //     // up once we return.
    //     //     let _ = f.borrow_mut().take();
    //     //     return;
    //     // }
    //     // app.update((*buttons).borrow().clone());

    //     // Set the body's text content to how many times this
    //     // requestAnimationFrame callback has fired.
    //     // i += 1;


    //     // Schedule ourself for another requestAnimationFrame callback.
    //     animation_frame(&win, f.borrow().as_ref().unwrap());
    // }) as Box<dyn FnMut()>));

    // animation_frame(&window, g.borrow().as_ref().unwrap());
    // }


    let init : Option<Closure<dyn FnMut()>> = None;
    let f = Rc::new(RefCell::new(init));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {

        app.update(buttons.borrow().clone());
        app.draw(&mut display);
        let image = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut display.data.0), 160, 128).expect("Unable to make image data");
        context.put_image_data(&image, 0., 0.);


        // Schedule ourself for another requestAnimationFrame callback.
        // let b : RefCell<Option<Closure<dyn FnMut()>>>= *f;
        request_animation_frame(f.borrow().as_ref().unwrap());

    }) as Box<dyn FnMut()>));// <dyn FnMut()>);

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
