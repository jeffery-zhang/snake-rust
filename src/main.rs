extern crate piston_window;
extern crate rand;

use piston_window::*;
use std::time::Instant;
mod food;
use food::Food;

static WINDOW_WIDTH: u32 = 1280;
static WINDOW_HEIGHT: u32 = 720;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut food = Food::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut events = Events::new(EventSettings::new()).ups(1);
    let mut last_update = Instant::now();

    while let Some(e) = events.next(&mut window) {
        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g, _| {
                clear([0.0; 4], g);
                rectangle(
                    [1.0, 1.0, 0.0, 1.0],
                    [food.x, food.y, food.width, food.height],
                    c.transform,
                    g,
                );
            });
        }

        if last_update.elapsed().as_secs() > 3 {
            food.set_position(vec![]);
            last_update = Instant::now();
        }
    }
}
