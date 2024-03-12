pub struct Snake {
    window_width: u32,
    window_height: u32,
    count: u32,
    default_speed: u32,
    pub unit: u32,
    pub forward_direction: Direction,
    pub speed: u32,
    pub body: Vec<(Direction, i32, i32)>,
    pub body_color: [f32; 4],
    pub head_color: [f32; 4],
}

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Snake {
    pub fn new(window_width: u32, window_height: u32, unit: u32, speed: u32) -> Self {
        let head_point = (Direction::Right, 500, 500);
        Snake {
            window_width,
            window_height,
            count: 0,
            unit,
            default_speed: speed,
            forward_direction: Direction::Right,
            speed,
            body: [0; 5]
                .iter()
                .enumerate()
                .map(|(idx, _)| {
                    (
                        head_point.0,
                        head_point.1 - (idx as i32 * (unit as i32)),
                        head_point.2,
                    )
                })
                .collect::<Vec<(Direction, i32, i32)>>(),
            head_color: [1.0, 0.0, 0.0, 1.0],
            body_color: [0.0, 0.0, 1.0, 1.0],
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

    pub fn speed_up(&mut self) {
        self.speed = self.unit;
    }

    pub fn slow_down(&mut self) {
        self.speed = self.default_speed;
    }

    pub fn moving(&mut self) {
        self.count += self.speed;
        if self.count < self.unit {
            return;
        }
        let dir = self.forward_direction;
        let new_head: (Direction, i32, i32) = match dir {
            Direction::Up => {
                let mut y = self.body[0].2 - (self.unit as i32);
                if y < 0 {
                    y = (self.window_height - self.unit) as i32;
                }
                (self.forward_direction, self.body[0].1, y)
            }
            Direction::Down => {
                let mut y = self.body[0].2 + (self.unit as i32);
                if y >= (self.window_height as i32) {
                    y = 0;
                }
                (self.forward_direction, self.body[0].1, y)
            }
            Direction::Left => {
                let mut x = self.body[0].1 - (self.unit as i32);
                if x < 0 {
                    x = (self.window_width - self.unit) as i32;
                }
                (self.forward_direction, x, self.body[0].2)
            }
            Direction::Right => {
                let mut x = self.body[0].1 + (self.unit as i32);
                if x >= (self.window_width as i32) {
                    x = 0;
                }
                (self.forward_direction, x, self.body[0].2)
            }
        };
        self.body.insert(0, new_head);
        self.body.pop();
        self.count = 0;
    }

    pub fn grow(&mut self) {
        let dir = self.body.last().unwrap().0;
        let new_tail: (Direction, i32, i32) = match dir {
            Direction::Up => (
                dir,
                self.body.last().unwrap().1,
                self.body.last().unwrap().2 + (self.unit as i32),
            ),
            Direction::Down => (
                dir,
                self.body.last().unwrap().1,
                self.body.last().unwrap().2 - (self.unit as i32),
            ),
            Direction::Left => (
                dir,
                self.body.last().unwrap().1 + (self.unit as i32),
                self.body.last().unwrap().2,
            ),
            Direction::Right => (
                dir,
                self.body.last().unwrap().1 - (self.unit as i32),
                self.body.last().unwrap().2,
            ),
        };
        self.body.push(new_tail);
    }
}
