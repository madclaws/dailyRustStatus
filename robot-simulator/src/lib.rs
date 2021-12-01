// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot{
    x: i32, 
    y: i32,
    d: Direction
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot{x, y, d}
    }

    pub fn turn_right(&mut self) -> &mut Self {
        self.d = Robot::get_next_direction(&self.d, "right");
        self        
    }

    pub fn turn_left(&mut self) -> &mut Self {
        self.d = Robot::get_next_direction(&self.d, "left");
        self        
    }

    pub fn advance(&mut self) -> &mut Self {
        match self.d {
            Direction::East => self.x += 1,
            Direction::North => self.y += 1,
            Direction::South => self.y -= 1,
            Direction::West => self.x -= 1
        }
        self
    }

    pub fn instructions(&mut self, instructions: &str) -> &mut Self {
        for code in instructions.chars() {
            if code == 'R' {
                self.turn_right();
            } else if code == 'L' {
                self.turn_left();
            } else {
                self.advance();
            }
        }
        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }

    pub fn get_next_direction(current_direction: &Direction, turn: &str) -> Direction {
        if turn == "right" {
            match current_direction {
                Direction::East => Direction::South,
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::West => Direction::North
            }
        } else {
            match current_direction {
                Direction::East => Direction::North,
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::West => Direction::South
            }
        }
    }
}
