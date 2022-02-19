pub fn encode(key: &str, s: &str) -> Option<String> {
    let mut encoded_string: String = String::new();
    let mut key_itr = key.chars();
    let mut s_itr = s.chars();
    if key.is_empty() {
        return None
    }
    for _index in 0..s.len() {
        let sub_value = get_substitution_value(key_itr.next().unwrap()); 
        if sub_value < 0 {
            return None
        }
        let rotated_code = rotate_character(s_itr.next().unwrap() as i32,
        sub_value as i8) as u8;
        encoded_string.push(rotated_code as char);
    }
    println!("ENCODED STRING = {encoded_string}");
    Some(encoded_string)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    unimplemented!("Use {} to decode {} using shift cipher", key, s)
}

pub fn encode_random(s: &str) -> (String, String) {
    unimplemented!(
        "Generate random key with only a-z chars and encode {}. Return tuple (key, encoded s)",
        s
    )
}

fn get_substitution_value(key: char) -> i32 {
    let code = key as i32;
    println!("{code}");
    code - 97
}

fn rotate_character(char_code: i32, key: i8) -> i32 {
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