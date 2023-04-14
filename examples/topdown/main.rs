#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]

use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::{Rgb565, Rgb888},
    primitives::Rectangle,
    prelude::*,
};
use tinybmp::Bmp;
use trowel::{App, AppResult, Buttons, Error, buffered::BufferedApp};
use trowel::flipped::{DrawTargetExt2, Axes};

const BMP_DATA: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/topdown/sprites/player.bmp"));

const SPRITE_COUNT: usize = 51;

#[derive(Clone, Copy)]
struct SpriteData {
    name: &'static str,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
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
    bmp: Option<Bmp<'static, Rgb565>>,
    current_animation: Animation,
    current_frame_index: usize,
    position: Point, 
}

impl App for TopDown {
    fn init(&mut self) -> AppResult {
        self.bmp = Some(Bmp::from_slice(BMP_DATA).map_err(|e| Error::BmpErr(e))?);
        self.current_animation = IDLE;
        self.current_frame_index = 0;
        self.position = Point::new(0, 0);
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
        let speed = 3;
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
        display.clear(Rgb565::BLACK)
                .map_err(|_| Error::DisplayErr)?;


        let sprite_index = self.current_animation.frame_indices[self.current_frame_index % self.current_animation.frame_count()];

        let sprite = sprite_data_new(sprite_index);
        let position = self.position;
        let at = position;
        let size = Size::new(sprite.width, sprite.height);
    
        if self.current_animation == LEFT_IDLE || self.current_animation == LEFT_WALK || self.current_animation == LEFT_ATTACK {
            self.bmp
            .expect("no bmp set")                
            .draw_sub_image(&mut target.cropped(&Rectangle::new(at, size)).flipped(Axes::X),
            &Rectangle::new(Point::new(sprite.x, sprite.y), size)).map_err(|_| Error::DisplayErr)?;
        }
        else {
        self.bmp
            .expect("no bmp set")                
            .draw_sub_image(&mut target.cropped(&Rectangle::new(at, size)).flipped(Axes::empty()),
            &Rectangle::new(Point::new(sprite.x, sprite.y), size)).map_err(|_| Error::DisplayErr)?;
        }
    
        Ok(())
    }      
}

#[trowel::entry]
fn main() {
    let app = TopDown {frame: -1,bmp:None,current_animation:IDLE,current_frame_index:0, position: Point::new(0, 0) };
    let mut app = BufferedApp::new(app);
    app.frame_buf.data.transparent = Some(Rgb565::from(Rgb888::new(0xee, 0x00, 0xff)));
    trowel::run(app);
}
