use std::iter::StepBy;
use std::ops::Range;

use rand::seq::IteratorRandom;
use rand::thread_rng;

pub struct Food {
    x_range: StepBy<Range<u32>>,
    y_range: StepBy<Range<u32>>,
    pub unit: u32,
    pub x: i32,
    pub y: i32,
    pub color: [f32; 4],
}

impl Food {
    pub fn new(window_width: u32, window_height: u32, unit: u32) -> Self {
        let x_range = (unit..(window_width - unit)).step_by(unit as usize);
        let y_range = (unit..(window_height - unit)).step_by(unit as usize);

        Food {
            x_range,
            y_range,
            unit,
            x: 400,
            y: 400,
            color: [1.0, 1.0, 0.0, 1.0],
        }
    }
    pub fn set_position(&mut self, exclude: Vec<(u32, u32)>) {
        let (x, y) = self.randomize(exclude);
        self.x = x as i32;
        self.y = y as i32;
    }

    fn randomize(&mut self, exclude: Vec<(u32, u32)>) -> (u32, u32) {
        let x_range = &self.x_range;
        let y_range = &self.y_range;
        let x = x_range.clone().choose(&mut thread_rng()).unwrap();
        let y = y_range.clone().choose(&mut thread_rng()).unwrap();
        for point in &exclude {
            if (x, y) == *point {
                return self.randomize(exclude);
            }
        }

        (x, y)
    }
}
