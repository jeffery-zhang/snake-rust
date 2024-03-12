use crate::food::Food;
use crate::piston_window::*;
use crate::snake::*;
use std::{thread::sleep, time::Duration};

pub struct Game {
    window_width: u32,
    window_height: u32,
    pub unit: u32,
    pub state: GameState,
    pub snake: Snake,
    pub food: Food,
    pub curr_level: u32,
    pub all_levels: Vec<u32>,
    pub difficulty: Difficulty,
}

#[derive(PartialEq)]
pub enum GameState {
    Menu,
    Playing(Difficulty),
    GameOver,
}

#[derive(PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Game {
    pub fn new(window_width: u32, window_height: u32, unit: u32) -> Self {
        let speed = match Difficulty::Easy {
            Difficulty::Easy => 3,
            Difficulty::Medium => 6,
            Difficulty::Hard => 9,
        };

        Game {
            window_width,
            window_height,
            unit,
            state: GameState::Playing(Difficulty::Medium),
            snake: Snake::new(window_width, window_height, unit, speed),
            food: Food::new(window_width, window_height, unit),
            curr_level: 1,
            all_levels: vec![1, 2, 3, 4, 5],
            difficulty: Difficulty::Medium,
        }
    }

    pub fn start(&mut self, difficulty: Difficulty) {
        self.state = GameState::Playing(difficulty);
    }

    pub fn over<F: FnMut()>(&mut self, mut callback: F) {
        self.snake.body_color = [1.0, 0.0, 0.0, 1.0];
        // sleep(Duration::from_secs(2));
        self.state = GameState::GameOver;
        callback();
    }

    pub fn eating_check(&mut self) -> bool {
        let snake_head = self.snake.body[0];
        if snake_head.1 == self.food.x && snake_head.2 == self.food.y {
            self.snake.grow();
            self.food.set_position(vec![]);
            return true;
        }

        false
    }

    pub fn impact_check(&mut self) -> bool {
        let snake_head = self.snake.body[0];
        for i in &mut self.snake.body[1..] {
            if i.1 == snake_head.1 && i.2 == snake_head.2 {
                return true;
            }
        }
        return false;
    }

    pub fn render(&mut self, event: &Event, window: &mut PistonWindow) {
        let food = &mut self.food;
        let snake = &mut self.snake;
        window.draw_2d(event, |c, g, _| {
            clear([0.0; 4], g);
            snake.moving();
            rectangle(
                food.color,
                [
                    food.x as f64,
                    food.y as f64,
                    food.unit as f64,
                    food.unit as f64,
                ],
                c.transform,
                g,
            );
            for (idx, s) in snake.body.iter().enumerate() {
                let color = if idx == 0 {
                    snake.head_color
                } else {
                    snake.body_color
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

    pub fn add_control(&mut self, event: Event) {
        let snake = &mut self.snake;
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up => snake.turn(Direction::Up),
                Key::Down => snake.turn(Direction::Down),
                Key::Left => snake.turn(Direction::Left),
                Key::Right => snake.turn(Direction::Right),
                Key::Space => snake.speed_up(),
                _ => {}
            }
        }
        if let Some(Button::Keyboard(key)) = event.release_args() {
            match key {
                Key::Space => snake.slow_down(),
                _ => {}
            }
        }
    }
}
