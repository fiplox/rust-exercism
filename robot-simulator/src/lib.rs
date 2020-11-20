// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, d }
    }

    pub fn turn_right(self) -> Self {
        match self.d {
            Direction::North => Robot {
                x: self.x,
                y: self.y,
                d: Direction::East,
            },
            Direction::East => Robot {
                x: self.x,
                y: self.y,
                d: Direction::South,
            },
            Direction::South => Robot {
                x: self.x,
                y: self.y,
                d: Direction::West,
            },
            Direction::West => Robot {
                x: self.x,
                y: self.y,
                d: Direction::North,
            },
        }
    }

    pub fn turn_left(self) -> Self {
        match self.d {
            Direction::North => Robot {
                x: self.x,
                y: self.y,
                d: Direction::West,
            },
            Direction::East => Robot {
                x: self.x,
                y: self.y,
                d: Direction::North,
            },
            Direction::South => Robot {
                x: self.x,
                y: self.y,
                d: Direction::East,
            },
            Direction::West => Robot {
                x: self.x,
                y: self.y,
                d: Direction::South,
            },
        }
    }

    pub fn advance(self) -> Self {
        match self.d {
            Direction::North => Robot {
                x: self.x,
                y: self.y + 1,
                d: self.d,
            },
            Direction::East => Robot {
                x: self.x + 1,
                y: self.y,
                d: self.d,
            },
            Direction::South => Robot {
                x: self.x,
                y: self.y - 1,
                d: self.d,
            },
            Direction::West => Robot {
                x: self.x - 1,
                y: self.y,
                d: self.d,
            },
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut r = Robot::new(self.x, self.y, self.d);
        for c in instructions.chars() {
            match c {
                'L' => r = r.turn_left(),
                'R' => r = r.turn_right(),
                'A' => r = r.advance(),
                _ => continue,
            }
        }
        r
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
