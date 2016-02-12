extern crate piston_window;
extern crate find_folder;

use piston_window::*;

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

    let mut x = 0.0;
    let mut y = 0.0;

    for e in window {
        e.draw_2d(|c, g| {
            clear(black, g);

            let transform = c.transform.trans(
                x * font_size,
                (y + 1.0) * font_size
            );

            text::Text::new_color(white, 14).draw(
                "@",
                &mut glyphs,
                &c.draw_state,
                transform,
                g
            );
        });

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::H => x -= 1.0,
                Key::J => y += 1.0,
                Key::K => y -= 1.0,
                Key::L => x += 1.0,
                _ => ()
            }
        }
    }
}
