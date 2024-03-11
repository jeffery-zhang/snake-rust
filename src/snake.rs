pub struct Snake {
    window_width: u32,
    window_height: u32,
    pub unit: u32,
    pub forward_direction: Direction,
    pub speed: u32,
    pub body: Vec<(Direction, u32, u32)>,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Snake {
    pub fn new(window_width: u32, window_height: u32, unit: u32) -> Self {
        let head_point = (Direction::Right, 500, 500);
        Snake {
            window_width,
            window_height,
            unit,
            forward_direction: Direction::Right,
            speed: 10,
            body: [0; 5]
                .iter()
                .enumerate()
                .map(|(idx, _)| {
                    (
                        head_point.0,
                        head_point.1 - (idx as u32 * unit),
                        head_point.2,
                    )
                })
                .collect::<Vec<(Direction, u32, u32)>>(),
        }
    }

    pub fn turn(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                if self.forward_direction != Direction::Down {
                    self.forward_direction = direction;
                }
            }
            Direction::Down => {
                if self.forward_direction != Direction::Up {
                    self.forward_direction = direction;
                }
            }
            Direction::Left => {
                if self.forward_direction != Direction::Right {
                    self.forward_direction = direction;
                }
            }
            Direction::Right => {
                if self.forward_direction != Direction::Left {
                    self.forward_direction = direction;
                }
            }
        };
    }

    pub fn fast_forward(&mut self) {
        self.speed *= 2;
    }

    pub fn slow_down(&mut self) {
        self.speed = 1;
    }

    pub fn one_frame_move(&mut self) {
        let dir = self.forward_direction;
        let new_head: (Direction, u32, u32) = match dir {
            Direction::Up => {
                let mut y = self.body[0].2 - self.speed;
                if y < self.unit {
                    y = self.window_height - self.unit;
                }
                (self.forward_direction, self.body[0].1, y)
            }
            Direction::Down => {
                let mut y = self.body[0].2 + self.speed;
                if y >= self.window_height {
                    y = 0;
                }
                (self.forward_direction, self.body[0].1, y)
            }
            Direction::Left => {
                let mut x = self.body[0].1 - self.speed;
                if x < self.unit {
                    x = self.window_width - self.unit;
                }
                (self.forward_direction, x, self.body[0].2)
            }
            Direction::Right => {
                let mut x = self.body[0].1 + self.speed;
                if x >= self.window_width {
                    x = 0;
                }
                (self.forward_direction, x, self.body[0].2)
            }
        };
        self.body.insert(0, new_head);
        self.body.pop();
    }
}
