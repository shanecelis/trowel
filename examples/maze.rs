/* Original code[1] Copyright (c) 2023 Grant Handy[2]
   Modified code[3] by Shane Celis[4], Copyright (c) 2023 Hack Club[5]
   Licensed under the MIT License[6]

   [1]: https://github.com/grantshandy/wasm4-raycaster
   [2]: https://github.com/grantshandy
   [3]: https://github.com/shanecelis/trowel/blob/master/examples/maze.rs
   [4]: https://mastodon.gamedev.place/@shanecelis
   [5]: https://hackclub.com
   [6]: https://opensource.org/licenses/MIT
*/

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]

use core::f32::consts::{FRAC_PI_2, PI};
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::{raw::RawU24, Rgb565, Rgb888},
    prelude::*,
    primitives::{Line, PrimitiveStyle},
};
#[allow(unused_imports)]
use micromath::F32Ext;
use trowel::{buffered::BufferedApp, App, AppExt, AppResult, Buttons, Error, FpsApp, FS};
use try_default::TryDefault;

// The original platform had a 160x160 display. Sprig only has a 160x128
// display.
// const HEIGHT: i32 = 160;
const HEIGHT: i32 = 128;

// const STEP_SIZE: f32 = 0.045;
// Made the step size bigger since our fps was low.
const STEP_SIZE: f32 = 0.09;
// const FIVE_PI_SQUARED: f32 = 5.0 * (PI * PI);

const FOV: f32 = PI / 2.7; // The player's field of view.
const HALF_FOV: f32 = FOV * 0.5; // Half the player's field of view.
const ANGLE_STEP: f32 = FOV / 160.0; // The angle between each ray.

// WALL_HEIGHT was 100 for a 160 screen height, so we worked backwards.
const WALL_HEIGHT: f32 = HEIGHT as f32 - 60.0;

const PALETTE: [u32; 4] = [0xe0f8cf, 0x86c06c, 0x306850, 0x071821];

fn to_color(c: u32) -> Rgb565 {
    Rgb565::from(Rgb888::from(RawU24::new(c)))
}

const MAP: [u16; 8] = [
    0b1111111111111111,
    0b1000001010000101,
    0b1011100000110101,
    0b1000111010010001,
    0b1010001011110111,
    0b1011101001100001,
    0b1000100000001101,
    0b1111111111111111,
];

impl App for State {
    fn init(&mut self) -> AppResult {
        // This way the first frame is zero.
        self.frame = -1;
        Ok(())
    }

    fn update(&mut self, buttons: Buttons) -> AppResult {
        self.frame += 1;

        self.update(
            buttons.contains(Buttons::W),
            buttons.contains(Buttons::S),
            buttons.contains(Buttons::A),
            buttons.contains(Buttons::D),
        );
        Ok(())
    }

    fn draw<T, E>(&mut self, display: &mut T) -> AppResult
    where
        T: DrawTarget<Color = Rgb565, Error = E>,
    {
        display
            .clear(to_color(PALETTE[0]))
            .map_err(|_| Error::DisplayErr)?;
        // Go through each column on screen and draw walls in the center.
        for (x, wall) in self.get_view().iter().enumerate() {
            let (height, shadow) = wall;

            let color = to_color(PALETTE[if *shadow { 1 } else { 2 }]);
            let y = HEIGHT / 2 - (height / 2);
            let _ = Line::new(
                Point::new(x as i32, y),
                Point::new(x as i32, y + *height - 1),
            )
            .into_styled(PrimitiveStyle::with_stroke(color, 1))
            .draw(display)
            .map_err(|_| Error::DisplayErr)?;
        }
        Ok(())
    }
}

struct State {
    frame: i32,
    player_x: f32,
    player_y: f32,
    player_angle: f32,
}

impl State {
    /// Move the character
    pub fn update(&mut self, up: bool, down: bool, left: bool, right: bool) {
        // Store our current position in case we might need it later.
        let previous_position = (self.player_x, self.player_y);

        if up {
            self.player_x += cosf(self.player_angle) * STEP_SIZE;
            self.player_y += -sinf(self.player_angle) * STEP_SIZE;
        }

        if down {
            self.player_x -= cosf(self.player_angle) * STEP_SIZE;
            self.player_y -= -sinf(self.player_angle) * STEP_SIZE;
        }

        if right {
            self.player_angle -= STEP_SIZE;
        }

        if left {
            self.player_angle += STEP_SIZE;
        }

        // If moving us on this frame put us into a wall, just revert it.
        if point_in_wall(self.player_x, self.player_y) {
            (self.player_x, self.player_y) = previous_position;
        }
    }

    /// Returns 160 wall heights and their "color" from the player's perspective.
    pub fn get_view(&self) -> [(i32, bool); 160] {
        // The player's FOV is split in half by their viewing angle. In order to
        // get the ray's starting angle, we must add half the FOV to the
        // player's angle to get the edge of the player's FOV.
        let starting_angle = self.player_angle + HALF_FOV;

        let mut walls = [(0, false); 160];

        for (idx, wall) in walls.iter_mut().enumerate() {
            // The `idx` is what number ray we are, `wall` is a mutable
            // reference to a value in `walls`.
            let angle = starting_angle - idx as f32 * ANGLE_STEP;

            // Get both the closest horizontal and vertical wall intersections
            // for this angle.
            let h_dist = self.horizontal_intersection(angle);
            let v_dist = self.vertical_intersection(angle);

            let (min_dist, shadow) = if h_dist < v_dist {
                (h_dist, false)
            } else {
                (v_dist, true)
            };

            // Get the minimum of the two distances and "convert" it into a wall
            // height.
            *wall = (
                (WALL_HEIGHT / (min_dist * cosf(angle - self.player_angle))) as i32,
                shadow,
            );
        }

        walls
    }

    /// Returns the nearest wall the ray intersects with on a horizontal grid line.
    fn horizontal_intersection(&self, angle: f32) -> f32 {
        // This tells you if the angle is "facing up" regardless of how big the
        // angle is.
        let up = fabsf(floorf(angle / PI) % 2.0) != 0.0;

        // The first_y and first_x are the first grid intersections that the ray
        // intersects with.
        let first_y = if up {
            ceilf(self.player_y) - self.player_y
        } else {
            floorf(self.player_y) - self.player_y
        };
        let first_x = -first_y / tanf(angle);

        // The dy and dx are the "ray extension" values mentioned earlier.
        let dy = if up { 1.0 } else { -1.0 };
        let dx = -dy / tanf(angle);

        // The next_x and next_y are mutable values which will keep track of how
        // far away the ray is from the player.
        let mut next_x = first_x;
        let mut next_y = first_y;

        // This is the loop where the ray is extended until it hits the wall.
        // It's not an infinite loop as implied in the explanation, instead it
        // only goes from 0 to 256.
        //
        // This was chosen because if something goes wrong and the ray never
        // hits a wall (which should never happen) the loop will eventually
        // break and the game will keep on running.
        for _ in 0..256 {
            // current_x and current_y are where the ray is currently
            // on the map, while next_x and next_y are relative
            // coordinates, current_x and current_y are absolute
            // points.
            let current_x = next_x + self.player_x;
            let current_y = if up {
                next_y + self.player_y
            } else {
                next_y + self.player_y - 1.0
            };

            // Tell the loop to quit if we've just hit a wall.
            if point_in_wall(current_x, current_y) {
                break;
            }

            // If we didn't hit a wall on this extension, add dx and dy to our
            // current position and keep going.
            next_x += dx;
            next_y += dy;
        }

        // Return the distance from next_x and next_y to the player.
        distance(next_x, next_y)
    }

    /// Returns the nearest wall the ray intersects with on a vertical grid line.
    fn vertical_intersection(&self, angle: f32) -> f32 {
        // This tells you if the angle is "facing up" regardless of how big the
        // angle is.
        let right = fabsf(floorf((angle - FRAC_PI_2) / PI) % 2.0) != 0.0;

        // The first_y and first_x are the first grid intersections that the ray
        // intersects with.
        let first_x = if right {
            ceilf(self.player_x) - self.player_x
        } else {
            floorf(self.player_x) - self.player_x
        };
        let first_y = -tanf(angle) * first_x;

        // The dy and dx are the "ray extension" values mentioned earlier.
        let dx = if right { 1.0 } else { -1.0 };
        let dy = dx * -tanf(angle);

        // The next_x and next_y are mutable values which will keep track of how
        // far away the ray is from the player.
        let mut next_x = first_x;
        let mut next_y = first_y;

        // This is the loop where the ray is extended until it hits the wall.
        // It's not an infinite loop as implied in the explanation, instead it
        // only goes from 0 to 256.
        //
        // This was chosen because if something goes wrong and the ray never
        // hits a wall (which should never happen) the loop will eventually quit
        // and the game will keep on running.
        for _ in 0..256 {
            // current_x and current_y are where the ray is currently
            // on the map, while next_x and next_y are relative
            // coordinates, current_x and current_y are absolute
            // points.
            let current_x = if right {
                next_x + self.player_x
            } else {
                next_x + self.player_x - 1.0
            };
            let current_y = next_y + self.player_y;

            // Tell the loop to quit if we've just hit a wall.
            if point_in_wall(current_x, current_y) {
                break;
            }

            // If we didn't hit a wall on this extension, add dx and dy to our
            // current position and keep going.
            next_x += dx;
            next_y += dy;
        }

        // Return the distance from next_x and next_y to the player.
        distance(next_x, next_y)
    }
}

fn point_in_wall(x: f32, y: f32) -> bool {
    match MAP.get(y as usize) {
        Some(line) => line & (0b1_u16 << core::cmp::min(x.floor() as u16, 15)) != 0,
        None => true,
    }
}

fn distance(a: f32, b: f32) -> f32 {
    sqrtf((a * a) + (b * b))
}

fn sinf(x: f32) -> f32 {
    x.sin()
    // let y = x / TAU;
    // let z = y - floorf(y);
    // x = z * TAU;

    // let sinf_imp = |x: f32| -> f32 {
    //     // these magic numbers were discovered 1400 years ago!
    //     (16.0 * x * (PI - x)) / (FIVE_PI_SQUARED - (4.0 * x * (PI - x)))
    // };

    // if x > PI {
    //     -sinf_imp(x - PI)
    // } else {
    //     sinf_imp(x)
    // }
}

fn cosf(x: f32) -> f32 {
    x.cos()
    // sinf(x + FRAC_PI_2)
}

fn tanf(x: f32) -> f32 {
    x.tan()
    // sinf(x) / cosf(x)
}

fn sqrtf(x: f32) -> f32 {
    // unsafe { core::intrinsics::sqrtf32(x) }
    x.sqrt()
}

fn floorf(x: f32) -> f32 {
    x.floor()
    // unsafe { core::intrinsics::floorf32(x) }
}

fn ceilf(x: f32) -> f32 {
    x.ceil()
    // unsafe { core::intrinsics::ceilf32(x) }
}

fn fabsf(x: f32) -> f32 {
    // unsafe { core::intrinsics::fabsf32(x) }
    x.abs()
}

fn main() {
    trowel::run_with(|| {
        let state: State = State {
            frame: 0,
            player_x: 1.5,
            player_y: 1.5,
            player_angle: -PI / 2.0,
        };
        let fps_app = FpsApp::try_default().expect("Could not create fps app");
        let mut app = BufferedApp::new(state.join(fps_app));
        // let mut app = BufferedApp::new(state);
        app.interlace = None;
        // These are just demo/debug buttons.
        app.increase_button = Some(Buttons::I);
        app.decrease_button = Some(Buttons::K);
        app
    })
}

#[cfg_attr(all(target_arch = "arm", target_os = "none"), cortex_m_rt::entry)]
fn entry() -> ! {
    main();
    loop {}
}
