/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
use gcd::Gcd;

#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    match is_gcd_compatible(a, b) {
        Err(val) => Err(val),
        _ => {
            let mut encoded_text: String = String::from("");
            let mut consecutive_count = 0;
            for letter in plaintext.chars() {
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
                let encoded_index = ((a * index) + b) % 26;
                encoded_text.push(get_letter_from_index(encoded_index));
                consecutive_count += 1;
            }
            Ok(encoded_text)
        }
    }
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    match calculate_mmi(a, 26) {
        Err(val) => Err(val),
        Ok(mmi) => {
            let mut decoded_text: String = String::from("");
            for letter in ciphertext.chars() {
                if !letter.is_alphanumeric() {
                    continue;
                }
                if letter.is_numeric() {
                    decoded_text.push(letter);
                    continue;    
                }
                let index = get_index_from_letter(letter);
                let decoded_index = (mmi * (index - b)) % 26;
                decoded_text.push(get_letter_from_index(decoded_index));
            }
            Ok(decoded_text)
        }
    }
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

fn is_gcd_compatible(a: i32, b: i32) -> Result<bool, AffineCipherError> {
    let a_u = a as u32;
    let b_u = a as u32;
    if a_u.gcd(26_u32) != 1 {
        Err(AffineCipherError::NotCoprime(a))
    } else if b_u.gcd(26_u32) != 1 {
        Err(AffineCipherError::NotCoprime(b))
    } else {
        Ok(true)
    }
}

pub fn calculate_mmi(a: i32, m: i32) -> Result<i32, AffineCipherError> {
    match is_gcd_compatible(a, m) {
        Err(reason) => Err(reason),
        _ => {
            for n in 1..m {
                if (a * n) % m == 1 {
                    return Ok(n);
                }
            }
            Err(AffineCipherError::NotCoprime(a))
        }
    }
}

