type ParsedMovement<'a> = (&'a str, &'a str, &'a str);

pub enum Movement {
    Up(u64),
    Down(u64),
    Forward(u64),
    Stationary,
}

impl<'a> Into<Movement> for &'a ParsedMovement<'a> {
    fn into(self) -> Movement {
        match self.0 {
            "up" => Movement::Up(self.2.parse::<u64>().unwrap_or(0)),
            "down" => Movement::Down(self.2.parse::<u64>().unwrap_or(0)),
            "forward" => Movement::Forward(self.2.parse::<u64>().unwrap_or(0)),
            _ => Movement::Stationary,
        }
    }
}

pub struct Submarine {
    movements: Vec<Movement>,
    horizontal_pos: u64,
    depth_pos: u64,
    aim: u64,
}

impl Default for Submarine {
    fn default() -> Self {
        Self {
            movements: vec![],
            horizontal_pos: 0,
            depth_pos: 0,
            aim: 0,
        }
    }
}

impl Submarine {
    pub fn add_movement(&mut self, movement: Movement) {
        self.movements.push(movement);
    }

    pub fn start_trip_part1(&mut self) {
        for movement in &self.movements {
            match movement {
                Movement::Up(y) => self.depth_pos -= y,
                Movement::Down(y) => self.depth_pos += y,
                Movement::Forward(x) => self.horizontal_pos += x,
                Movement::Stationary => {}
            }
        }
    }

    pub fn start_trip_part2(&mut self) {
        for movement in &self.movements {
            match movement {
                Movement::Down(y) => {
                    self.aim += y;
                }
                Movement::Up(y) => {
                    self.aim -= y;
                }
                Movement::Forward(x) => {
                    self.horizontal_pos += x;
                    self.depth_pos += self.aim * x;
                }
                Movement::Stationary => {}
            }
        }
    }

    pub fn get_coordinate(self) -> u64 {
        self.horizontal_pos * self.depth_pos
    }
}
