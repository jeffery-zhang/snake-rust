use rand::{thread_rng, Rng};

pub struct Food {
    window_width: u32,
    window_height: u32,
    pub width: f64,
    pub height: f64,
    pub x: f64,
    pub y: f64,
}

impl Food {
    pub fn new(window_width: u32, window_height: u32) -> Self {
        let x = thread_rng().gen_range(10..(window_width - 10));
        let y = thread_rng().gen_range(10..(window_height - 10));

        Food {
            window_width,
            window_height,
            width: 10f64,
            height: 10f64,
            x: x as f64,
            y: y as f64,
        }
    }
    pub fn set_position(&mut self, exclude: Vec<(u32, u32)>) {
        let (x, y) = self.randomize(exclude);
        self.x = x as f64;
        self.y = y as f64;
    }

    fn randomize(&mut self, exclude: Vec<(u32, u32)>) -> (u32, u32) {
        let x = thread_rng().gen_range(10..(self.window_width - 10));
        let y = thread_rng().gen_range(10..(self.window_height - 10));
        for point in &exclude {
            if (x, y) == *point {
                return self.randomize(exclude);
            }
        }

        (x, y)
    }
}
