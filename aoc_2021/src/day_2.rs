use std::fs;

#[derive(Debug)]
pub struct Position{
    x: i32,
    depth: i32,
    aim: i32
}

impl Position{
    pub fn new() -> Self {
        Position{x: 0, depth: 0, aim: 0}
    }

    pub fn move_forward(&mut self, units: i32) {
        self.x += units;
    }

    pub fn change_aim(&mut self, units: i32) {
        self.aim += units;
    }

    pub fn change_depth(&mut self, units: i32) {
        self.depth += self.aim * units
    }
}

pub fn run() {
    println!("\nDay 2 => Dive");
    
    let input: String = fs::read_to_string("/home/madclaws/labs/aoc_2021/inputs/day_2.txt").expect("Unable to load the file");
    let instruction_list: Vec<&str> = input.split('\n').collect();
    let mut position = Position::new();
    for instruction in instruction_list {
        let splitted_instruction: Vec<&str> = instruction.split(" ").collect();
        let units: i32 = splitted_instruction[1].parse().unwrap();
        match splitted_instruction[0]{
            "forward" => {
                position.move_forward(units);
                position.change_depth(units);
            },
            "down" => {
                position.change_aim(units)
            },
            _ => {
                position.change_aim(-units)
            }
        }
    }
    println!("Multiply your final horizontal position by your final depth with aim => {:?}", position.x * position.depth);
}