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
    strike_frames: Vec<StrikeFrame>,
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
            sub_frames: Vec::new(),
            total: 0,
            current_subframe: 0,
        }
    }
}

#[derive(Clone)]
pub struct StrikeFrame {
    frame_index: u16,
    next_rolls: Vec<u16>,
    active: bool,
}

impl StrikeFrame {
    pub fn new(frame_index: u16) -> Self {
        StrikeFrame {
            frame_index,
            next_rolls: Vec::new(),
            active: true,
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
            strike_frames: Vec::new(),
        }
    }

    /// TODO
    /// - Handle open frame - Done
    /// - Handle spare - DONE
    /// - Handle streak - DONE
    /// - Handle 10th frame edge case
    pub fn roll_old(&mut self, pins: u16) -> Result<(), Error> {
        // open frame
        if self.current_frame < 10 {
            let frame_index = self.current_frame;
            let mut is_current_frame_strike = false;
            {
                let current_frame = &mut self.frames[self.current_frame as usize];
                if pins != 10 {
                    current_frame.sub_frames[current_frame.current_subframe as usize] = pins;
                }
                if current_frame.current_subframe == 1 {
                    current_frame.total = current_frame.sub_frames.iter().sum();
                    if current_frame.total == 10 {
                        println!("Spare on frame {}", self.current_frame);
                        self.spare_frame = Some(self.current_frame)
                    }
                    self.current_frame += 1;

                    for strike_frame in &mut self.strike_frames {
                        if strike_frame.active {
                            println!("Strike frame exists at {}", strike_frame.frame_index);
                            strike_frame.next_rolls.push(pins);
                            if strike_frame.next_rolls.len() == 2 {
                                strike_frame.active = false;
                                let total_frame_score: u16 = strike_frame.next_rolls.iter().sum();
                                self.frames[strike_frame.frame_index as usize].total =
                                    total_frame_score + 10;
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
                        // updating streak score on first sub-frame
                        // self.frames[self.current_frame as usize].total = pins;
                        self.current_frame += 1;
                    } else {
                        current_frame.current_subframe += 1
                    }
                }
            }

            // This means the roll was for first sub_frame
            if frame_index == self.current_frame || is_current_frame_strike {
                // // Check if a spare exists, if then update spareframe total_score
                if let Some(frame) = self.spare_frame {
                    println!("A spare exists at frame {}", frame);
                    self.frames[frame as usize].total += pins;
                    self.spare_frame = None;
                }
                // find the active strike frames and update them
                for strike_frame in &mut self.strike_frames {
                    if strike_frame.active && !is_current_frame_strike {
                        println!("Strike frame exists at {}", strike_frame.frame_index);
                        strike_frame.next_rolls.push(pins);
                        println!("Pins added to strike frame {}", pins);
                        if strike_frame.next_rolls.len() == 2 {
                            strike_frame.active = false;
                            let total_frame_score: u16 = strike_frame.next_rolls.iter().sum();
                            self.frames[strike_frame.frame_index as usize].total =
                                total_frame_score + 10;
                        }
                    }
                }
            }

            if self.current_frame == 10 {
                self.is_game_completed = true;
            }
            Ok(())
        } else {
            return Err(Error::GameComplete);
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.is_game_completed {
            let mut total_score: u16 = 0;
            let mut frame_no: u16 = 0;
            for frame in &self.frames {
                let frame_total_score = frame.sub_frames.iter().sum::<u16>();
                total_score += frame_total_score;
                println!("Frame no:{} => {}", frame_no, frame_total_score);
                frame_no += 1;
            }
            Some(total_score)
        } else {
            None
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        // We get the valid frame, subframe to enter the points.
        // It will be an Option
        if let Some(current_frame) = self.get_valid_frame() {
            self.current_frame = current_frame;
            if self.is_pin_valid(pins) {
                self.frames[current_frame as usize].sub_frames.push(pins);
            } else {
                return Err(Error::NotEnoughPinsLeft);
            }
            if self.current_frame == 9 && self.frames[current_frame as usize].sub_frames.len() >= 2
            {
                self.is_game_completed = true;
            }
            return Ok(());
        } else {
            return Err(Error::GameComplete);
        }
    }

    /// Returns the frame and subframe for the points to be added
    fn get_valid_frame(&self) -> Option<u16> {
        if self.current_frame < 9 {
            if self.frames[self.current_frame as usize].sub_frames.len() < 2 {
                Some(self.current_frame)
            } else {
                Some(self.current_frame + 1)
            }
        } else if self.current_frame == 9 {
            // Will take care of sub_frame 3 later
            if self.frames[self.current_frame as usize].sub_frames.len() < 2 {
                Some(self.current_frame)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn is_pin_valid(&self, pins: u16) -> bool {
        pins <= (10
            - self.frames[self.current_frame as usize]
                .sub_frames
                .iter()
                .sum::<u16>())
    }
}
