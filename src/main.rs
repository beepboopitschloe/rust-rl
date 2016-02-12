extern crate piston_window;
extern crate find_folder;

use piston_window::*;
use std::slice::from_raw_parts;

#[derive(Copy, Clone)]
struct Tile {
    ch : char,
    color : types::Color
}

#[derive(Copy, Clone)]
struct Screen<'a> {
    width : u32,
    height : u32,
    tiles : &'a[Tile]
}

fn main() {
    let window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();

    let ref font = assets.join("curses.ttf");
    let factory = window.factory.borrow().clone();
    let mut glyphs = Glyphs::new(font, factory).unwrap();

    let white = [1.0; 4];
    let black = [0.0, 0.0, 0.0, 1.0];

    let font_size = 14.0;
    let w : u32 = 32;
    let h : u32 = 32;
    let screen_size = (w * h) as usize;
    let default_tile = Tile{
        ch: '.',
        color: [0.0, 0.7, 0.0, 1.0]
    };
    let mut tiles : &[Tile] = &[default_tile];

    unsafe {
        tiles = from_raw_parts(&default_tile, screen_size);
    }

    let mut screen = Screen{
        width: w,
        height: h,
        tiles: tiles
    };

    for e in window {
        e.draw_2d(|c, g| {
            clear(black, g);

            let t = text::Text::new_color(white, 14);

            for y in 0 .. screen.height {
                let range_start = y * screen.width;
                let range_end = range_start + screen.width;

                for x in range_start .. range_end {
                    let transform = c.transform.trans(
                        (x as f64) * font_size,
                        ((y as f64) + 1.0) * font_size
                    );

                    let idx = (y * screen.width + x) as usize;

                    let tile = screen.tiles[idx];
                    let s = ".";

                    println!("about to render {} to {}, {}", s, x, y);
                    t.draw(s, &mut glyphs, &c.draw_state, transform, g);
                    println!("finished render to {}, {}", x, y);
                }
            }
        });

        /*
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::H => x -= 1.0,
                Key::J => y += 1.0,
                Key::K => y -= 1.0,
                Key::L => x += 1.0,
                _ => ()
            }
        }
        */
    }
}
