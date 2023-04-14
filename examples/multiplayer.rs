#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]

use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Line, PrimitiveStyle, Rectangle},
    text::Text,
};
use trowel::{buffered::BufferedApp, App, AppResult, Buttons, Error, stdout};
use genio::Write;

// Define the cube struct
struct Cube {
    x: i32,
    y: i32,
    player_name: &'static str,
}

impl Cube {
    fn move_left(&mut self, distance: i32) {
        self.x -= distance;
    }

    fn move_right(&mut self, distance: i32) {
        self.x += distance;
    }

    fn move_up(&mut self, distance: i32) {
        self.y -= distance;
    }

    fn move_down(&mut self, distance: i32) {
        self.y += distance;
    }
}

struct DrawCubes {
    frame: i32, // Frame count
    cube1: Cube,
    cube2: Cube,
}

impl App for DrawCubes {
    fn init(&mut self) -> AppResult {
        Ok(())
    }

    fn update(&mut self, buttons: Buttons) -> AppResult {
        if buttons.contains(Buttons::W) {
            self.cube1.move_up(5);
        }

        if buttons.contains(Buttons::S) {
            self.cube1.move_down(5);
        }

        if buttons.contains(Buttons::A) {
            self.cube1.move_left(5);
        }

        if buttons.contains(Buttons::D) {
            self.cube1.move_right(5);
        }

        if buttons.contains(Buttons::I) {
            self.cube2.move_up(5);
        }

        if buttons.contains(Buttons::K) {
            self.cube2.move_down(5);
        }

        if buttons.contains(Buttons::J) {
            self.cube2.move_left(5);
        }

        if buttons.contains(Buttons::L) {
            self.cube2.move_right(5);
        }

        self.frame += 1;
        Ok(())
    }

    fn draw<T, E>(&mut self, display: &mut T) -> AppResult
    where
        T: DrawTarget<Color = Rgb565, Error = E>,
    {
        display
            .clear(Rgb565::BLACK)
            .map_err(|_| Error::DisplayErr)?;

        // Create a new character style
        let style = MonoTextStyle::new(&ascii::FONT_7X13, Rgb565::WHITE);

       // Draw player name above the first cube
       Text::new(
        self.cube1.player_name, // Use player_name field from cube1
        Point::new(self.cube1.x - 5, self.cube1.y - 15), // Update the text position according to the cube position
        style,
    )
    .draw(display)
    .map_err(|_| Error::DisplayErr)?;

    // Draw player name above the second cube
    Text::new(
        self.cube2.player_name, // Use player_name field from cube2
        Point::new(self.cube2.x - 5, self.cube2.y - 15), // Update the text position according to the cube position
        style,
    )
    .draw(display)
    .map_err(|_| Error::DisplayErr)?;

        // Create a new primitive style
        let cube_style = PrimitiveStyle::with_stroke(Rgb565::GREEN, 1);

        // Draw first cube
        let cube1 = Rectangle::new(Point::new(self.cube1.x, self.cube1.y), Size::new(30, 30))
            .into_styled(cube_style);
        cube1.draw(display).map_err(|_| Error::DisplayErr)?;

        // Draw second cube
        let cube2 = Rectangle::new(Point::new(self.cube2.x, self.cube2.y), Size::new(30, 30))
            .into_styled(cube_style);
        cube2.draw(display).map_err(|_| Error::DisplayErr)?;

        // Create a new line style
        let line_style = PrimitiveStyle::with_stroke(Rgb565::GREEN, 1);

        // Draw lines for first cube
        let offset = 10;
        let lines1 = [
            Line::new(
                Point::new(self.cube1.x, self.cube1.y),
                Point::new(self.cube1.x - offset, self.cube1.y - offset),
            )
            .into_styled(line_style),
            Line::new(
                Point::new(self.cube1.x + 30, self.cube1.y),
                Point::new(self.cube1.x + 30 - offset, self.cube1.y - offset),
            )
            .into_styled(line_style),
            Line::new(
                Point::new(self.cube1.x - offset, self.cube1.y - offset),
                Point::new(self.cube1.x + 30 - offset, self.cube1.y - offset),
            )
            .into_styled(line_style),
            Line::new(
                Point::new(self.cube1.x - offset, self.cube1.y + 30 - offset),
                Point::new(self.cube1.x + 30 - offset, self.cube1.y + 30 - offset),
            )
            .into_styled(line_style),
            Line::new(
                Point::new(self.cube1.x, self.cube1.y + 30),
                Point::new(self.cube1.x - offset, self.cube1.y + 30 - offset),
            )
            .into_styled(line_style),
            Line::new(
                Point::new(self.cube1.x + 30, self.cube1.y + 30),
                Point::new(self.cube1.x + 30 - offset, self.cube1.y + 30 - offset),
            )
            .into_styled(line_style),
            Line::new(
                Point::new(self.cube1.x - offset, self.cube1.y - offset),
                Point::new(self.cube1.x - offset, self.cube1.y + 30 - offset),
            )
            .into_styled(line_style),
            Line::new(
                Point::new(self.cube1.x + 30 - offset, self.cube1.y - offset),
                Point::new(self.cube1.x + 30 - offset, self.cube1.y + 30 - offset),
            )
            .into_styled(line_style),
        ];

        for line in &lines1 {
            line.draw(display).map_err(|_| Error::DisplayErr)?;
        }

        // Draw lines for second cube
        let lines2 = [
            Line::new(
                Point::new(self.cube2.x, self.cube2.y),
                Point::new(self.cube2.x - offset, self.cube2.y - offset),
            )
            .into_styled(line_style),
            Line::new(
                Point::new(self.cube2.x + 30, self.cube2.y),
                Point::new(self.cube2.x + 30 - offset, self.cube2.y - offset),
            )
            .into_styled(line_style),
            Line::new(
                Point::new(self.cube2.x - offset, self.cube2.y - offset),
                Point::new(self.cube2.x + 30 - offset, self.cube2.y - offset),
            )
            .into_styled(line_style),
            Line::new(
                Point::new(self.cube2.x - offset, self.cube2.y + 30 - offset),
                Point::new(self.cube2.x + 30 - offset, self.cube2.y + 30 - offset),
            )
            .into_styled(line_style),
            Line::new(
                Point::new(self.cube2.x, self.cube2.y + 30),
                Point::new(self.cube2.x - offset, self.cube2.y + 30 - offset),
            )
            .into_styled(line_style),
            Line::new(
                Point::new(self.cube2.x + 30, self.cube2.y + 30),
                Point::new(self.cube2.x + 30 - offset, self.cube2.y + 30 - offset),
            )
            .into_styled(line_style),
            Line::new(
                Point::new(self.cube2.x - offset, self.cube2.y - offset),
                Point::new(self.cube2.x - offset, self.cube2.y + 30 - offset),
            )
            .into_styled(line_style),
            Line::new(
                Point::new(self.cube2.x + 30 - offset, self.cube2.y - offset),
                Point::new(self.cube2.x + 30 - offset, self.cube2.y + 30 - offset),
            )
            .into_styled(line_style),
        ];

        for line in &lines2 {
            line.draw(display).map_err(|_| Error::DisplayErr)?;
        }
        Ok(())
    }
}

let map = r##"
!"#$%&'()*
+,-./01234
56789:;<=>
?@ABCDEFGH
IJKLMNOPQR
STUVWXYZ[\
]^_`abcdef
ghijklmnop
qrstuvwxyz

{|}~
"##; // that's 10x8 with 4 printable characters to spare.


let map = r##"
~~~____bp_
~~~_r_B___
~~________
__......._
_.........
..........
....____..
..._______
__________
"##;

let map = r##"
!"#$%&'()*
+,-./01234
56789:;<=>
?@ABCDEFGH
IJKLMNOPQR
STUVWXYZ[\
]^_`abcdef
ghijklmnop
qrstuvwxyz
{|}~
"##;
fn main() {
    let draw_cubes = DrawCubes {
        frame: 0,
        cube1: Cube {
            x: 20,
            y: 50,
            player_name: "cube1",
        },
        cube2: Cube {
            x: 70,
            y: 50,
            player_name: "cube2",
        },
    };

    let buffered_app = BufferedApp::new(draw_cubes);

    trowel::run(buffered_app);
}

#[cfg_attr(all(target_arch = "arm", target_os = "none"), trowel::entry)]
fn entry() -> ! {
    main();
    loop {}
}
