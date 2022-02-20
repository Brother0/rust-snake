extern crate piston_window;
extern crate rand;

mod constants;
mod draw;
mod game;
mod snake;

use draw::to_coord_u32;
use piston_window::*;

use crate::constants::BACK_COLOR;
use crate::constants::HEIGHT;
use crate::constants::WIDTH;
use game::Game;

fn main() {
    let (width, height) = (WIDTH, HEIGHT);

    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| game.update(arg.dt));
    }
}
