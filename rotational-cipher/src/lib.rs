pub fn rotate(input: &str, key: i8) -> String {
    let mut rot_string: String = String::new();
    // 'a'.to
    for ch in input.chars() {
        let rotated_code = rotate_character(ch as i32, key) as u8;
        println!("{}", rotated_code);
        rot_string.push(rotated_code as char);
    }
    println!("{}", rot_string);
    rot_string
}

fn rotate_character(char_code: i32, key: i8) -> i32 {
    println!("{}", char_code);
    if char_code > 64 && char_code < 91 {
        if char_code + key as i32 > 90 {
            ((char_code + key as i32) - 90) + 64
        } else if (char_code + key as i32) < 64  {
            91 - (64 - (char_code + key as i32))
        } 
        else {
            char_code + key as i32
        }
    } else if char_code > 96 &&  char_code <= 122 {
        if char_code + key as i32 > 122 {
            ((char_code + key as i32) - 122) + 96
        } else if (char_code + key as i32) < 97  {
            122 - (96 - (char_code + key as i32))
        } else {
            char_code + key as i32
        }
    } else {
        char_code
    }
}