#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    score: u16,
    current_frame: u16,
    sub_frames: Vec<u16>,
    is_game_completed: bool
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame{
            score: 0, current_frame: 0, sub_frames: Vec::new(),
            is_game_completed: false
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        // open frame 
        if pins < 10 {
            // score should be stored for current subframe
            self.sub_frames.push(pins);
            // handle for existing spare
            // handle for existing streak
        }
        // spare
        // streak
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.is_game_completed {
            None
        } else {
            Some(1)
        }
    }
}
