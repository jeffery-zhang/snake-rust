use crate::food::Food;
use crate::snake::Snake;

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

pub enum GameState {
    Menu,
    Playing(Difficulty),
    GameOver,
}

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Game {
    pub fn new(window_width: u32, window_height: u32, unit: u32) -> Self {
        Game {
            window_width,
            window_height,
            unit,
            state: GameState::Playing(Difficulty::Easy),
            snake: Snake::new(window_width, window_height, unit),
            food: Food::new(window_width, window_height, unit),
            curr_level: 1,
            all_levels: vec![1, 2, 3, 4, 5],
            difficulty: Difficulty::Easy,
        }
    }

    pub fn start(&mut self, difficulty: Difficulty) {
        self.state = GameState::Playing(difficulty);
    }
}
