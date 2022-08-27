#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    current_frame: u16,
    frames: Vec<Frame>,
    is_game_completed: bool,
    strike_frames: Vec<StrikeFrame>,
}

#[derive(Clone)]
pub struct Frame {
    sub_frames: Vec<u16>,
    total: u16,
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            sub_frames: Vec::new(),
            total: 0,
        }
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self::new()
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

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}
impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            current_frame: 0,
            frames: vec![Frame::new(); 10],
            is_game_completed: false,
            strike_frames: Vec::new(),
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.is_game_completed {
            let mut total_score: u16 = 0;
            for (frame_no, frame) in self.frames.iter().enumerate() {
                total_score += frame.total;
                println!("Frame no:{} => {}", frame_no, frame.total);
            }
            Some(total_score)
        } else {
            None
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        // We get the valid frame, subframe to enter the points.
        // It will be an Option
        if let Some(current_frame) = self.get_valid_frame(pins) {
            self.current_frame = current_frame;
            if self.is_pin_valid(pins) {
                self.frames[current_frame as usize].sub_frames.push(pins);
                println!(
                    "pins {} pushed on frame {}, subframe {}",
                    pins,
                    self.current_frame,
                    self.frames[current_frame as usize].sub_frames.len()
                );
                if self.current_frame < 9
                    && self.frames[current_frame as usize].sub_frames.len() == 2
                {
                    // calculate the total score if frame is finished
                    self.frames[current_frame as usize].total =
                        self.get_subframes_sum(current_frame);
                    self.handle_active_strikes(pins);
                } else if self.current_frame == 9
                    && self.frames[current_frame as usize].sub_frames.len() >= 2
                {
                    self.frames[current_frame as usize].total =
                        self.get_subframes_sum(current_frame);
                    self.handle_active_strikes(pins);
                } else {
                    // if sub_frame is 0, then check if previous frame is spare, if yes, then add current pin
                    // to its total score.
                    if self.current_frame > 0 && self.is_spare(self.current_frame - 1) {
                        println!("SPARE FRAME {}", self.current_frame);
                        self.frames[(self.current_frame - 1) as usize].total += pins
                    }
                    self.handle_active_strikes(pins);
                    // strike on first sub_frame
                    if pins == 10 && self.current_frame < 9 {
                        self.strike_frames
                            .push(StrikeFrame::new(self.current_frame))
                    }
                }
            } else {
                return Err(Error::NotEnoughPinsLeft);
            }
            if self.current_frame == 9
                && self.frames[current_frame as usize].sub_frames.len() == 2
                && self.get_subframes_sum(9) < 10
            {
                self.is_game_completed = true;
            } else if self.current_frame == 9
                && self.frames[current_frame as usize].sub_frames.len() == 3
            {
                self.is_game_completed = true;
            }
            Ok(())
        } else {
            Err(Error::GameComplete)
        }
    }

    /// Returns the frame and subframe for the points to be added, change for femo downtime
    fn get_valid_frame(&self, _pins: u16) -> Option<u16> {
        if self.current_frame < 9 {
            // println!("subframes of current frame {} are {}", self.current_frame, self.frames[self.current_frame as usize].sub_frames.len());
            if self.frames[self.current_frame as usize].sub_frames.len() < 2 {
                if self.frames[self.current_frame as usize].sub_frames.len() == 1
                    && self.get_subframes_sum(self.current_frame) == 10
                {
                    Some(self.current_frame + 1)
                } else {
                    Some(self.current_frame)
                }
            } else {
                Some(self.current_frame + 1)
            }
        } else if self.frames[self.current_frame as usize].sub_frames.len() == 2
            && self.get_subframes_sum(self.current_frame) >= 10
        {
            Some(self.current_frame)
        } else if self.frames[self.current_frame as usize].sub_frames.len() < 2 {
            Some(self.current_frame)
        } else {
            None
        }
    }

    fn is_pin_valid(&self, pins: u16) -> bool {
        if self.current_frame < 9 {
            pins <= (10 - self.get_subframes_sum(self.current_frame))
        } else if self.is_strike(self.current_frame)
            && self.get_subframes_len(9) == 2
            && self.frames[self.current_frame as usize].sub_frames[1] < 10
        {
            pins <= (10 - self.frames[self.current_frame as usize].sub_frames[1])
        } else if self.frames[self.current_frame as usize].sub_frames.len() <= 2
            && self.get_subframes_sum(self.current_frame) >= 10
        {
            pins <= 10
        } else {
            pins <= (10 - self.get_subframes_sum(self.current_frame))
        }
    }

    fn get_subframes_len(&self, frame_index: u16) -> u16 {
        self.frames[frame_index as usize].sub_frames.len() as u16
    }

    fn is_strike(&self, frame_index: u16) -> bool {
        self.frames[frame_index as usize].sub_frames.len() >= 1
            && self.frames[frame_index as usize].sub_frames[0] == 10
    }

    fn is_spare(&self, frame_index: u16) -> bool {
        self.get_subframes_sum(frame_index) == 10
            && self.frames[frame_index as usize].sub_frames.len() == 2
    }

    fn get_strike_frames(&self) -> Vec<usize> {
        self.strike_frames
        .iter()
        .enumerate()
        .filter(|(_i, frame)| frame.active)
        .map(|(i, _strikeframe)| i)
        .collect::<Vec<usize>>()
    }

    fn get_subframes_sum(&self, frame_index: u16) -> u16 {
        self.frames[frame_index as usize]
            .sub_frames
            .iter()
            .sum::<u16>()
    }

    fn handle_active_strikes(&mut self, pins: u16) {
        for index in self.get_strike_frames() {
            self.strike_frames[index].next_rolls.push(pins);
            let frame_index = self.strike_frames[index].frame_index;
            println!("pushed pins {} to next roll of strike {}", pins, frame_index);
            // println!("current frame index {} , next roll pins {}", self.current_frame, pins);
            if self.strike_frames[index].next_rolls.len() == 2 {
                self.strike_frames[index].active = false;
                self.frames[frame_index as usize].total = self.strike_frames[index]
                    .next_rolls
                    .iter()
                    .sum::<u16>()
                    + 10;
            }
        }
    }
}
