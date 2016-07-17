#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;

pub struct Robot {
    x : isize,
    y : isize,
    d : Direction,
}

impl Robot {

    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot { x : x, y : y, d : d }
    }

    pub fn turn_right(self) -> Self {
        Robot { d : match self.d { North => East, East => South,
                                   South => West, West => North},
                .. self }
    }

    pub fn turn_left(self) -> Self {
        Robot { d : match self.d { North => West, East => North,
                                   South => East, West => South},
                .. self }
    }

    pub fn advance(self) -> Self {
        Robot { x : match self.d { North | South => self.x,
                                   East  => self.x + 1, West  => self.x - 1},
                y : match self.d { East | West => self.y,
                                   North => self.y + 1, South => self.y - 1},
                .. self }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |t, inst| {
            match inst {
                'A' => t.advance(),
                'L' => t.turn_left(),
                'R' => t.turn_right(),
                _ => t,
            }
        })
    }

    pub fn position(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
