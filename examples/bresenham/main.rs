#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]

use runty8::{App, Button, Pico8, Resources};
use runty8_core as runty8;

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[macro_use]
extern crate alloc;
#[cfg(all(target_arch = "arm", target_os = "none"))]
use alloc::vec::Vec;
#[cfg(all(target_arch = "arm", target_os = "none"))]
use micromath::F32Ext;

fn main() {
    let resources = Resources::default(); //runty8::load_assets!("examples/bresenham").unwrap();

    trowel::runty8::run::<MyThing>(resources);
}

#[cfg_attr(all(target_arch = "arm", target_os = "none"), cortex_m_rt::entry)]
fn entry() -> ! {
    main();
    loop {}
}

struct MyThing {
    center_x: i32,
    center_y: i32,
    radius: i32,
}

impl App for MyThing {
    fn init(_: &mut Pico8) -> Self {
        Self {
            center_x: 64,
            center_y: 64,
            radius: 3,
        }
    }

    fn update(&mut self, state: &mut Pico8) {
        if state.btn(Button::Down) {
            self.center_y += 1;
        }
        if state.btn(Button::Up) {
            self.center_y -= 1;
        }
        if state.btn(Button::Right) {
            self.center_x += 1;
        }
        if state.btn(Button::Left) {
            self.center_x -= 1;
        }

        if state.btn(Button::X) {
            self.radius -= 1;
            self.radius = self.radius.max(0);
        }
        if state.btn(Button::C) {
            self.radius += 1;
        }
    }

    fn draw(&mut self, draw: &mut Pico8) {
        draw.cls(0);
        draw.pset(self.center_x, self.center_y, 7);
        for x_sign in [-1, 1] {
            for y_sign in [-1, 1] {
                draw.pset(
                    self.center_x + x_sign * self.radius,
                    self.center_y + y_sign * self.radius,
                    12,
                )
            }
        }
        draw.print(
            &format!("C = ({}, {})", self.center_x, self.center_y),
            4,
            4,
            7,
        );
        draw.print(&format!("R = {}", self.radius), 4, 12, 7);

        for (x, y) in midpoint(self.center_x, self.center_y, self.radius) {
            draw.pset(x, y, 14);
        }
    }
}

fn midpoint(cx: i32, cy: i32, r: i32) -> Vec<(i32, i32)> {
    let mut points = vec![];

    let mut x = r as f32;
    let mut y = 0 as f32;
    while y < x {
        points.push((cx + x as i32, cy + y as i32));
        points.push((cx - x as i32, cy + y as i32));
        points.push((cx + x as i32, cy - y as i32));
        points.push((cx - x as i32, cy - y as i32));
        points.push((cx + y as i32, cy + x as i32));
        points.push((cx - y as i32, cy + x as i32));
        points.push((cx + y as i32, cy - x as i32));
        points.push((cx - y as i32, cy - x as i32));

        x = (x.powi(2) - 2.0 * y - 1.0).sqrt();
        y += 1.0;
    }
    points
}
