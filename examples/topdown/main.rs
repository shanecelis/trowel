#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]

use embedded_graphics::{
    draw_target::DrawTarget,
    framebuffer,
    framebuffer::{Framebuffer, buffer_size},
    pixelcolor::{Rgb565, Rgb888, raw::{LittleEndian, RawU16}},
    primitives::Rectangle,
    image::{SubImage, GetPixel},
    prelude::*,
};
use tinybmp::Bmp;
use trowel::{App, AppResult, Buttons, Error, buffered::BufferedApp};
use trowel::flipped::{DrawTargetExt2, Axes};
use heapless::FnvIndexSet as Set;

const BMP_DATA: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/topdown/sprites/player.bmp"));

const SPRITE_COUNT: usize = 51;
const SPRITE_WIDTH_MAX : usize = 23;
const SPRITE_HEIGHT_MAX : usize = 33;
const SET_SIZE : usize = 512;
type Framebuf = Framebuffer::<Rgb565, RawU16, LittleEndian, 160, 128, {buffer_size::<Rgb565>(160, 128)}>;

// Ought to be the maximum size of a sprite.
type Spritebuf = Framebuffer::<Rgb565, RawU16, LittleEndian, SPRITE_WIDTH_MAX, SPRITE_HEIGHT_MAX,
                               {buffer_size::<Rgb565>(SPRITE_WIDTH_MAX, SPRITE_HEIGHT_MAX)}>;

#[derive(Clone, Copy)]
struct SpriteData {
    name: &'static str,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl SpriteData {
    fn as_image<'a, T>(&self, atlas: &'a T) -> SubImage<'a, T> where T : ImageDrawable {
        atlas.sub_image(&Rectangle::new(Point::new(self.x, self.y), Size::new(self.width, self.height)))
    }
}

const SPRITE_DATA: [SpriteData; SPRITE_COUNT] = [
    // Idle
    SpriteData { name: "sprite1", x: 52, y: 21, width: 13, height: 21 },
    SpriteData { name: "sprite2", x: 112, y: 23, width: 13, height: 21 },
    SpriteData { name: "sprite3", x: 38, y: 0, width: 13, height: 21 },
    SpriteData { name: "sprite4", x: 30, y: 71, width: 13, height: 20 },
    SpriteData { name: "sprite5", x: 20, y: 92, width: 13, height: 20 },
    SpriteData { name: "sprite6", x: 74, y: 113, width: 13, height: 20 },

    // right idle
    SpriteData { name: "sprite7", x: 126, y: 44, width: 15, height: 21 },
    SpriteData { name: "sprite8", x: 126, y: 22, width: 15, height: 21 },
    SpriteData { name: "sprite9", x: 52, y: 43, width: 15, height: 21 },
    SpriteData { name: "sprite10", x: 0, y: 71, width: 15, height: 20 },
    SpriteData { name: "sprite11", x: 52, y: 0, width: 15, height: 20 },
    SpriteData { name: "sprite12", x: 96, y: 92, width: 15, height: 20 },

    // up idle
    SpriteData { name: "sprite13", x: 112, y: 68, width: 13, height: 21 },
    SpriteData { name: "sprite14", x: 112, y: 90, width: 13, height: 21 },
    SpriteData { name: "sprite15", x: 82, y: 44, width: 13, height: 21 },
    SpriteData { name: "sprite16", x: 21, y: 19, width: 13, height: 20 },
    SpriteData { name: "sprite17", x: 51, y: 92, width: 13, height: 20 },
    SpriteData { name: "sprite18", x: 82, y: 92, width: 13, height: 20 },

    // down walking
    SpriteData { name: "sprite19", x: 82, y: 66, width: 13, height: 23 },
    SpriteData { name: "sprite20", x: 68, y: 22, width: 13, height: 23 },
    SpriteData { name: "sprite21", x: 112, y: 0, width: 13, height: 22 },
    SpriteData { name: "sprite22", x: 38, y: 22, width: 13, height: 22 },
    SpriteData { name: "sprite23", x: 96, y: 47, width: 13, height: 21 },
    SpriteData { name: "sprite24", x: 82, y: 22, width: 13, height: 21 },

    // right walking
    SpriteData { name: "sprite25", x: 96, y: 23, width: 15, height: 23 },
    SpriteData { name: "sprite26", x: 126, y: 89, width: 15, height: 23 },
    SpriteData { name: "sprite27", x: 126, y: 66, width: 15, height: 22 },
    SpriteData { name: "sprite28", x: 96, y: 0, width: 15, height: 22 },
    SpriteData { name: "sprite29", x: 126, y: 0, width: 15, height: 21 },
    SpriteData { name: "sprite30", x: 126, y: 113, width: 15, height: 21 },
    
    // up walking
    SpriteData { name: "sprite31", x: 112, y: 45, width: 13, height: 22 },
    SpriteData { name: "sprite32", x: 96, y: 69, width: 13, height: 22 },
    SpriteData { name: "sprite33", x: 68, y: 0, width: 13, height: 21 },
    SpriteData { name: "sprite34", x: 82, y: 0, width: 13, height: 21 },
    SpriteData { name: "sprite35", x: 16, y: 71, width: 13, height: 20 },
    SpriteData { name: "sprite36", x: 23, y: 49, width: 13, height: 20 },

    // down attack
    SpriteData { name: "sprite37", x: 0, y: 22, width: 20, height: 26 },
    SpriteData { name: "sprite38", x: 65, y: 92, width: 16, height: 20 },
    SpriteData { name: "sprite39", x: 0, y: 0, width: 19, height: 21 },
    SpriteData { name: "sprite40", x: 68, y: 46, width: 13, height: 19 },

    // right attack
    SpriteData { name: "sprite41", x: 0, y: 113, width: 34, height: 23 },
    SpriteData { name: "sprite42", x: 35, y: 113, width: 20, height: 21 },
    SpriteData { name: "sprite43", x: 34, y: 92, width: 16, height: 20 },
    SpriteData { name: "sprite44", x: 44, y: 71, width: 15, height: 19 },

    // up attack
    SpriteData { name: "sprite45", x: 0, y: 49, width: 22, height: 21 },
    SpriteData { name: "sprite46", x: 56, y: 113, width: 17, height: 20 },
    SpriteData { name: "sprite47", x: 0, y: 92, width: 19, height: 20 },
    SpriteData { name: "sprite48", x: 37, y: 49, width: 13, height: 19 },


    // die
    SpriteData { name: "sprite49", x: 21, y: 0, width: 16, height: 18 },
    SpriteData { name: "sprite50", x: 88, y: 113, width: 17, height: 16 },
    SpriteData { name: "sprite51", x: 60, y: 71, width: 21, height: 13 },
];

#[derive(PartialEq, Eq)]
struct Animation {
    name: &'static str,
    frame_indices: &'static [usize],
}

const IDLE: Animation = Animation {
    name: "idle",
    frame_indices: &[0, 1, 2, 3, 4, 5],
};

const RIGHT_IDLE: Animation = Animation {
    name: "IdleRight",
    frame_indices: &[6, 7, 8, 9, 10, 11],
};

const LEFT_IDLE: Animation = Animation {
    name: "IdleLeft",
    frame_indices: &[6, 7, 8, 9, 10, 11],
};

const UP_IDLE: Animation = Animation {
    name: "IdleUp",
    frame_indices: &[12, 13, 14, 15, 16, 17],
};

const DOWN_WALK: Animation = Animation {
    name: "DownWalk",
    frame_indices: &[18, 19, 20, 21, 22, 23],
};

const RIGHT_WALK: Animation = Animation {
    name: "RightWalk",
    frame_indices: &[24, 25, 26, 27, 28, 29],
};

const LEFT_WALK: Animation = Animation {
    name: "LeftWalk",
    frame_indices: &[24, 25, 26, 27, 28, 29],
};

const UP_WALK: Animation = Animation {
    name: "UpWalk",
    frame_indices: &[30, 31, 32, 33, 34, 35],
};

const DOWN_ATTACK: Animation = Animation {
    name: "DownAttack",
    frame_indices: &[36, 37, 38, 39],
};

const RIGHT_ATTACK: Animation = Animation {
    name: "RightAttack",
    frame_indices: &[40, 41, 42, 43],
};

const LEFT_ATTACK: Animation = Animation {
    name: "LeftAttack",
    frame_indices: &[40, 41, 42, 43],
};

const UP_ATTACK: Animation = Animation {
    name: "UpAttack",
    frame_indices: &[44, 45, 46, 47],
};

const DIE: Animation = Animation {
    name: "Die",
    frame_indices: &[48, 49, 50],
};

impl Animation {
    fn frame_count(&self) -> usize {
        self.frame_indices.len()
    }
}

fn sprite_data_new(i: usize) -> SpriteData {
    SPRITE_DATA[i % SPRITE_COUNT]
}

struct TopDown {
    frame: i32,
    bmp: Bmp<'static, Rgb565>,
    current_animation: Animation,
    current_frame_index: usize,
    position: Point,
    transparent: Rgb565,
    framebuf: Framebuf,
    dirty_points: Set<MyPoint, SET_SIZE>,
    old_dirty_points: Set<MyPoint, SET_SIZE>,
}

// We just Point to implement Hash32, but we can't add that to Point, so we do
// the next best thing. One nice thing is these one-element wrapper types in
// rust don't actually cost any extra memory.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]//, hash32_derive::Hash32)]
struct MyPoint(Point);

impl From<Point> for MyPoint {
    fn from(value: Point) -> Self {
        Self(value)
    }
}


impl hash32::Hash for MyPoint {
    fn hash<H: hash32::Hasher>(&self, h: &mut H) {
        self.0.x.hash(h);
        self.0.y.hash(h);
    }
}

impl TopDown {
    fn new() -> Result<Self, Error> {
        let bmp = Bmp::from_slice(BMP_DATA).map_err(|e| Error::BmpErr(e))?;
        Ok(Self { frame: -1,
                  bmp,
                  current_animation: IDLE,
                  current_frame_index: 0,
                  position: Point::new(0, 0),
                  transparent: Rgb565::from(Rgb888::new(0xee, 0x00, 0xff)),
                  framebuf: Framebuf::new(),
                  dirty_points: Set::new(),
                  old_dirty_points: Set::new()
        })
    }
}

impl App for TopDown {
    fn init(&mut self) -> AppResult {
        Ok(())
    }

    fn update(&mut self, buttons: Buttons) -> AppResult {
        self.frame += 1;
    
        let is_walking_animation = [UP_WALK, DOWN_WALK, RIGHT_WALK, LEFT_WALK].contains(&self.current_animation);
        if is_walking_animation && self.current_frame_index + 1 == self.current_animation.frame_count() {
            match self.current_animation {
                UP_WALK => self.current_animation = UP_IDLE,
                DOWN_WALK => self.current_animation = IDLE,
                RIGHT_WALK => self.current_animation = RIGHT_IDLE,
                LEFT_WALK => self.current_animation = LEFT_IDLE,
                _ => (),
            }
        }
    
        // MOVEMENT
        let speed = 15;
        if buttons.contains(Buttons::W) {
            if self.current_animation != UP_IDLE && self.current_animation != UP_WALK {
                self.current_animation = UP_IDLE;
                self.current_frame_index = 0;
            } else if !is_walking_animation {
                self.position.y -= speed;
                self.current_animation = UP_WALK;
                self.current_frame_index = 0;
            }
        } else if buttons.contains(Buttons::D) {
            if self.current_animation != RIGHT_IDLE && self.current_animation != RIGHT_WALK {
                self.current_animation = RIGHT_IDLE;
                self.current_frame_index = 0;
            } else if !is_walking_animation {
                self.position.x += speed;
                self.current_animation = RIGHT_WALK;
                self.current_frame_index = 0;
            }
        } else if buttons.contains(Buttons::S) {
            if self.current_animation != IDLE && self.current_animation != DOWN_WALK {
                self.current_animation = IDLE;
                self.current_frame_index = 0;
            } else if !is_walking_animation {
                self.position.y += speed;
                self.current_animation = DOWN_WALK;
                self.current_frame_index = 0;
            }
        } else if buttons.contains(Buttons::A) {
            if self.current_animation != LEFT_IDLE && self.current_animation != LEFT_WALK {
                self.current_animation = LEFT_IDLE;
                self.current_frame_index = 0;
            } else if !is_walking_animation {
                self.position.x -= speed;
                self.current_animation = LEFT_WALK;
                self.current_frame_index = 0;
            }
        }
    
        let is_attack_animation = [UP_ATTACK, DOWN_ATTACK, RIGHT_ATTACK, LEFT_ATTACK].contains(&self.current_animation);
        let is_die_animation = self.current_animation == DIE;
        if (is_attack_animation || is_die_animation) && self.current_frame_index + 1 == self.current_animation.frame_count() {
        match self.current_animation {
            UP_ATTACK => self.current_animation = UP_IDLE,
            DOWN_ATTACK => self.current_animation = IDLE,
            RIGHT_ATTACK => self.current_animation = RIGHT_IDLE,
            LEFT_ATTACK => self.current_animation = LEFT_IDLE,
            DIE => self.current_animation = IDLE,
            _ => (),
        }
        // Reset the current frame index
        self.current_frame_index = 0;
    }
        // ATTACK
        if buttons.contains(Buttons::I) {
            if self.current_animation == IDLE {
                self.current_animation = DOWN_ATTACK;
            } else if self.current_animation == RIGHT_IDLE {
                self.current_animation = RIGHT_ATTACK;
            } else if self.current_animation == LEFT_IDLE {
                self.current_animation = LEFT_ATTACK;
            } else if self.current_animation == UP_IDLE {
                self.current_animation = UP_ATTACK;
            }
        }

        // DIE
        if buttons.contains(Buttons::K) {
            self.current_animation = DIE;
        }
    
        if !is_walking_animation || (is_walking_animation && self.current_frame_index + 1 < self.current_animation.frame_count()) {
            // Update the current frame index
            self.current_frame_index = (self.current_frame_index + 1) % self.current_animation.frame_count();
        }
    
        Ok(())
    }

    fn draw<T, E>(&mut self, target: &mut T) -> AppResult
    where
        T: DrawTarget<Color = Rgb565, Error = E>,
    {
        // We buffered. We can clear all the time.
        if self.frame == 0 {
            let area = self.framebuf.bounding_box();
            self.framebuf.fill_contiguous(&area,
                                          (0..area.size.width*area.size.height).map(|i| RawU16::from(i as u16).into()))
                    .map_err(|_| Error::DisplayErr)?;

            
            self.framebuf.as_image().draw(target)
                .map_err(|_| Error::DisplayErr)?;
            // target.clear(Rgb565::RED)
            //         .map_err(|_| Error::DisplayErr)?;
        }


        let sprite_index = self.current_animation.frame_indices[self.current_frame_index % self.current_animation.frame_count()];

        let sprite = sprite_data_new(sprite_index);
        let sprite_image = sprite.as_image(&self.bmp);
        let position = self.position;
        let at = position;
        let size = Size::new(sprite.width, sprite.height);
        let area = Rectangle::new(at, size);

        let mut axes = Axes::empty();
        if self.current_animation == LEFT_IDLE || self.current_animation == LEFT_WALK || self.current_animation == LEFT_ATTACK {
            axes |= Axes::X;
        }
        let mut sprite_buf = Spritebuf::new();
        sprite_image.draw(&mut sprite_buf)
                     .map_err(|_| Error::DisplayErr)?;


        self.dirty_points.clear();
        // Take note of all pixels touched in dirty_points.
        target
            .cropped(&area).flipped(axes)
            .draw_iter(Rectangle::new(Point::new(0,0), size)
                         .points()
                         .filter_map(|p| sprite_buf.pixel(p) // Option<Rgb565>
                                                   .filter(|c| *c != self.transparent)
                                                   .map(|c| Pixel(p, c)))
                         .map(|Pixel(p,c)| {
                             let q = axes.flip(p, size) + at;
                             // Don't need to draw over the background points we're drawing this time.
                             self.old_dirty_points.remove(&MyPoint::from(q));
                             let _ = self.dirty_points.insert(MyPoint::from(q));
                             Pixel(p,c)
                         }))
            .map_err(|_| Error::DisplayErr)?;

        // Restore the background pixels from the last sprite draw.
        target
            .draw_iter(self.old_dirty_points.iter()
                         .filter_map(|p| self.framebuf.pixel(p.0).map(|c| Pixel(p.0, c))))
            .map_err(|_| Error::DisplayErr)?;

        core::mem::swap(&mut self.dirty_points, &mut self.old_dirty_points);

        Ok(())
    }      
}

#[trowel::entry]
fn main() {
    let app = TopDown::new().expect("Could not make TopDown app");
    trowel::run(app);
}
