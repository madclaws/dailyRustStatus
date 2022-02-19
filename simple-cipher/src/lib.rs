use rand::Rng;


pub fn encode(key: &str, s: &str) -> Option<String> {
    let mut encoded_string: String = String::new();
    let mut key_itr = key.chars();
    let mut s_itr = s.chars();
    if key.is_empty() {
        return None
    }
    for _index in 0..s.len() {
        let mut sub_value = 0;
        if let Some(next_key) = key_itr.next() {
            sub_value = get_substitution_value(next_key);
        }
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
    let mut decode_string: String = String::new();
    let mut key_itr = key.chars();
    let mut s_itr = s.chars();
    if key.is_empty() {
        return None
    }
    for _index in 0..s.len() {
        let mut sub_value = 0;
        if let Some(next_key) = key_itr.next() {
            if next_key.is_uppercase() || next_key.is_numeric(){
                return None
            }
            sub_value = get_substitution_value_decode(next_key);
        }
        let rotated_code = rotate_character(s_itr.next().unwrap() as i32,
        sub_value as i8) as u8;
        decode_string.push(rotated_code as char);
    }
    Some(decode_string)
}

pub fn encode_random(s: &str) -> (String, String) {
    let key = generate_random_key();
    let key_clone = key.clone();
    (key, encode(&key_clone, s).unwrap())
}

fn get_substitution_value(key: char) -> i32 {
    let code = key as i32;
    println!("{code}");
    code - 97
}


fn get_substitution_value_decode(key: char) -> i32 {
    let code = key as i32;
    println!("{code}");
    -(code - 97)
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

fn generate_random_key() -> String {
    let mut rand_key: String = String::new();
    for _index in 0..100 {
        rand_key.push(((rand::thread_rng().gen_range(97..122)) as u8) as char);
    }
    rand_key
}