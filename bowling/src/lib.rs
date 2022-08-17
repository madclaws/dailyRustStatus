#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    current_frame: u16,
    frames: Vec<Frame>,
    is_game_completed: bool,
    spare_frame: Option<u16>,
    strike_frames: Vec<StrikeFrame>
}

#[derive(Clone)]
pub struct Frame {
    sub_frames: Vec<u16>,
    current_subframe: u16,
    total: u16,
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            sub_frames: vec![0; 3],
            total: 0,
            current_subframe: 0,
        }
    }
}

#[derive(Clone)]
pub struct StrikeFrame {
    frame_index: u16,
    next_rolls: Vec<u16>,
    active: bool
}

impl StrikeFrame {
    pub fn new(frame_index: u16) -> Self {
        StrikeFrame{frame_index, next_rolls: Vec::new(), active: true}
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            spare_frame: None,
            current_frame: 0,
            frames: vec![Frame::new(); 10],
            is_game_completed: false,
            strike_frames: Vec::new()
        }
    }

    /// TODO
    /// - Handle open frame - Done
    /// - Handle spare - DONE
    /// - Handle streak
    /// - Handle 10th frame edge case
    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        // open frame
        if self.current_frame < 10 {
            let frame_index = self.current_frame;
            let mut is_current_frame_strike = false;
            {
                let current_frame = &mut self.frames[self.current_frame as usize];
                current_frame.sub_frames[current_frame.current_subframe as usize] = pins;
                if current_frame.current_subframe == 1 {
                    current_frame.total = current_frame.sub_frames.iter().sum();
                    if current_frame.total == 10 {
                        println!("Spare on frame {}", self.current_frame);
                        self.spare_frame = Some(self.current_frame)
                    }
                    self.current_frame += 1;

                    for strike_frame in &mut self.strike_frames {
                        if strike_frame.active {
                            strike_frame.next_rolls.push(pins);
                            if strike_frame.next_rolls.len() == 2 {
                                strike_frame.active = false;
                                let total_frame_score: u16 = strike_frame.next_rolls.iter().sum();
                                self.frames[strike_frame.frame_index as usize].total = total_frame_score;
                            }
                        }
                    }
                } else {
                    if pins == 10 {
                        // A strike
                        println!("Strike on frame {}", self.current_frame);
                        let strike_frame = StrikeFrame::new(self.current_frame);
                        self.strike_frames.push(strike_frame);
                        is_current_frame_strike = true;
                        self.current_frame += 1;
                    } else {
                        current_frame.current_subframe += 1
                    }
                }
            }
            
            // This means the roll was for first sub_frame
            if frame_index == self.current_frame || is_current_frame_strike{
                 // // Check if a spare exists, if then update spareframe total_score
                if let Some(frame) = self.spare_frame {
                    println!("A spare exists at frame {}", frame);
                    self.frames[frame as usize].total += pins;
                    self.spare_frame = None;
                }
                // find the active strike frames and update them
                for strike_frame in &mut self.strike_frames {
                    if strike_frame.active {
                        strike_frame.next_rolls.push(pins);
                        if strike_frame.next_rolls.len() == 2 {
                            strike_frame.active = false;
                            let total_frame_score: u16 = strike_frame.next_rolls.iter().sum();
                            self.frames[strike_frame.frame_index as usize].total = total_frame_score;
                        }
                    }
                }

                
            }

            if self.current_frame == 10 {
                self.is_game_completed = true;    
            }
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.is_game_completed {
            let mut total_score: u16 = 0;
            for frame in &self.frames {
                total_score += frame.total;
            }
            Some(total_score)
        } else {
            None
        }
    }
}
