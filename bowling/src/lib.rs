#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    score: u16,
    current_frame: u16,
    frame_list: Vec<u16>
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame{score: 0, current_frame: 0, frame_list: Vec::new()}
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        unimplemented!("Record that {} pins have been scored", pins);
    }

    pub fn score(&self) -> Option<u16> {
        unimplemented!("Return the score if the game is complete, or None if not.");
    }
}
