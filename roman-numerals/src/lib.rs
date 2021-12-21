use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    value: String
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)        
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let num_string = num.to_string();
        let mut current_digit_place: i32 = (num_string.len() - 1) as i32;
        let mut roman_string: String = String::from("");
        for digit_char in num_string.chars() {
            let digit = digit_char.to_digit(10).unwrap();
            if digit != 0 {
                roman_string = format!("{}{}", roman_string, get_roman_numerals(digit, &current_digit_place))  
            } 
            current_digit_place -= 1;
        }
        Roman{value: roman_string}
    }
}

fn get_roman_numerals(digit: u32, current_digit_place: &i32) -> String {
    match current_digit_place {
        0 => String::from(get_roman_script(digit)),
        1 => get_tens_place(digit),
        2 => get_hundreds_place(digit),
        3 => get_roman_script(1000).repeat(digit as usize),
        _ => String::from("")
    }
}

fn get_tens_place(digit: u32) -> String {
    match digit {
        4 => format!("{}{}", get_roman_script(10), get_roman_script(50)),
        9 => format!("{}{}", get_roman_script(10), get_roman_script(100)),
        5 => get_roman_script(50).to_string(),
        digit if digit < 5 => get_roman_script(10).repeat(digit as usize),
        _ => format!("{}{}", get_roman_script(50), get_roman_script(10).repeat((digit - 5) as usize))
    }
}

fn get_hundreds_place(digit: u32) -> String {
    match digit {
        4 => format!("{}{}", get_roman_script(100), get_roman_script(500)),
        9 => format!("{}{}", get_roman_script(100), get_roman_script(1000)),
        5 => get_roman_script(500).to_string(),
        digit if digit < 5 => get_roman_script(100).repeat(digit as usize),
        _ => format!("{}{}", get_roman_script(500), get_roman_script(100).repeat((digit - 5) as usize))
    }
}

fn get_roman_script<'a>(num: u32) -> &'a str {
    match num {
        1 => "I",
        2 => "II",
        3 => "III",
        4 => "IV",
        5 => "V",
        6 => "VI",
        7 => "VII",
        8 => "VIII",
        9 => "IX",
        10 => "X",
        50 => "L",
        100 => "C",
        500 => "D",
        1000 => "M",
        _ => ""
    }
}
