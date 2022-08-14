#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    current_frame: u16,
    frames: Vec<Frame>,
    is_game_completed: bool,
    spare_frame: Option<u16>
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

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            spare_frame: None,
            current_frame: 0,
            frames: vec![Frame::new(); 10],
            is_game_completed: false,
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
                } else {
                    current_frame.current_subframe += 1
                }
            }
            
            // This means the roll was for first sub_frame
            if frame_index == self.current_frame {
                 // // Check if a spare exists, if then update spareframe total_score
                if let Some(frame) = self.spare_frame {
                    println!("A spare exists at frame {}", frame);
                    self.frames[frame as usize].total += pins;
                    self.spare_frame = None;
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
