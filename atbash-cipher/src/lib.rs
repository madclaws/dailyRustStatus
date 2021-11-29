/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut encoded_text: String = String::from("");
    let mut consecutive_count = 0;
    for letter in plain.chars() {
        if !letter.is_alphanumeric() {
            continue;
        }
        if consecutive_count == 5 {
            encoded_text.push(' ');
            consecutive_count = 0;
        }
        if letter.is_numeric() {
            encoded_text.push(letter);
            consecutive_count += 1;
            continue;    
        }
        let index = get_index_from_letter(letter);
        let encoded_index = 25 - index;
        encoded_text.push(get_letter_from_index(encoded_index));
        consecutive_count += 1;
    }
    encoded_text
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let mut decoded_text: String = String::from("");
    for letter in cipher.chars() {
        if !letter.is_alphanumeric() {
            continue;
        }
        if letter.is_numeric() {
            decoded_text.push(letter);
            continue;    
        }
        let index = get_index_from_letter(letter);
        let decoded_index = 25 - index;
        decoded_text.push(get_letter_from_index(decoded_index));
    }
    decoded_text
}

pub fn get_index_from_letter(letter: char) -> i32 {
    let ascii_val = letter.to_lowercase().next().unwrap() as i32;
    ascii_val - 97
}

pub fn get_letter_from_index(index: i32) -> char {
    if (97 + index as i32) < 97  {
        (122 - (96 - (97 + index)) as u8) as char
    } else {
        ((97 + index) as u8) as char
    }
}