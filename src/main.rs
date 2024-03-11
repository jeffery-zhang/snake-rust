extern crate piston_window;
extern crate rand;

use piston_window::*;
use std::time::Instant;

mod game;
use game::Game;
mod gui;
use gui::create_menu_buttons;
mod food;
mod snake;

static WINDOW_WIDTH: u32 = 1280;
static WINDOW_HEIGHT: u32 = 720;
static POINT_UNIT: u32 = 10;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(WINDOW_WIDTH, WINDOW_HEIGHT, POINT_UNIT);
    let mut events = Events::new(EventSettings::new()).ups(1);
    let mut last_update = Instant::now();

    while let Some(e) = events.next(&mut window) {
        let food = &mut game.food;
        let snake = &mut game.snake;
        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g, _| {
                clear([0.0; 4], g);
                snake.one_frame_move();
                rectangle(
                    [1.0, 1.0, 0.0, 1.0],
                    [food.x, food.y, food.unit as f64, food.unit as f64],
                    c.transform,
                    g,
                );
                for (idx, s) in snake.body.iter().enumerate() {
                    let color = if idx == 0 {
                        [1.0, 0.0, 0.0, 1.0]
                    } else {
                        [0.0, 1.0, 0.0, 1.0]
                    };
                    rectangle(
                        color,
                        [s.1 as f64, s.2 as f64, snake.unit as f64, snake.unit as f64],
                        c.transform,
                        g,
                    );
                }
            });
        }

        if last_update.elapsed().as_secs() > 3 {
            (*food).set_position(vec![]);
            last_update = Instant::now();
        }
    }
}
