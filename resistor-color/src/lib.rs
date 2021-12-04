use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[repr(u8)]
#[derive(Debug, PartialEq, IntoEnumIterator, Copy, Clone, IntEnum)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(color: ResistorColor) -> usize {
    match color {
        ResistorColor::Black => 0,
        ResistorColor::Brown => 1,
        ResistorColor::Red => 2,
        ResistorColor::Orange => 3,
        ResistorColor::Yellow => 4,
        ResistorColor::Green => 5,
        ResistorColor::Blue => 6,
        ResistorColor::Violet => 7,
        ResistorColor::Grey => 8,
        ResistorColor::White => 9
    }
}

pub fn value_to_color_string(value: usize) -> String {
    println!("{:?}", ResistorColor::into_enum_iter().nth(value));
    match ResistorColor::into_enum_iter().nth(value) {
        Some(ResistorColor::Black) => String::from("Black"),
        Some(ResistorColor::Brown) => String::from("Brown"),
        Some(ResistorColor::Red) => String::from("Red"),
        Some(ResistorColor::Orange) => String::from("Orange"),
        Some(ResistorColor::Yellow) => String::from("Yellow"),
        Some(ResistorColor::Green) => String::from("Green"),
        Some(ResistorColor::Blue) => String::from("Blue"),
        Some(ResistorColor::Violet) => String::from("Violet"),
        Some(ResistorColor::Grey) => String::from("Grey"),
        Some(ResistorColor::White) => String::from("White"),
        _ => String::from("value out of range")
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut resistor_list: Vec<ResistorColor> = Vec::new();
    for val in 0..10 {
        resistor_list.push(ResistorColor::from_int(val).unwrap()); 
    }
    resistor_list
}
