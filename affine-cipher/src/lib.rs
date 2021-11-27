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
        Err(val) => return Err(val),
        _ => {
            let mut encoded_text: String = String::from("");
            for letter in plaintext.chars() {
                if !letter.is_alphanumeric() {
                    continue;
                }
                let index = get_index_from_letter(letter);
                let encoded_index = ((a * index) + b) % 26;
                encoded_text.push(get_letter_from_index(encoded_index))
            }
            Ok(encoded_text)
        }
    }
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    unimplemented!("Decode {} with the key ({}, {})", ciphertext, a, b);
}

pub fn get_index_from_letter(letter: char) -> i32 {
    let ascii_val = letter.to_lowercase().next().unwrap() as i32;
    ascii_val - 97
}

pub fn get_letter_from_index(index: i32) -> char {
    let ascii_val = (97 + index) as u8;
    ascii_val as char
}

fn is_gcd_compatible(a: i32, b: i32) -> Result<bool, AffineCipherError> {
    let a_u = a as u32;
    let b_u = a as u32;
    if a_u.gcd(26 as u32) != 1 {
        Err(AffineCipherError::NotCoprime(a))
    } else if b_u.gcd(26 as u32) != 1 {
        Err(AffineCipherError::NotCoprime(b))
    } else {
        Ok(true)
    }
}

