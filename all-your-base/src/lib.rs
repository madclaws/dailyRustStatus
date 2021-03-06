#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase)
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase)
    }
    match convert_to_decimal(number, from_base) {
        Ok(decimal_number) => Ok(convert_from_decimal(decimal_number, to_base)),
        Err(error) => Err(error)
    }
}

pub fn convert_to_decimal(number: &[u32], from_base: u32) -> Result<u32, Error> {
    let mut decimal_number = 0;
    let mut power = (number.len()) as u32;
    for elem in number {
        if elem >= &from_base {
            return Err(Error::InvalidDigit(*elem))
        }
        power -= 1;
        decimal_number += elem * u32::pow(from_base, power);
    }
    Ok(decimal_number)
}

pub fn convert_from_decimal(decimal_number: u32, to_base: u32) -> Vec<u32> {
    let mut base_vec: Vec<u32> = Vec::new();
    let mut decimal_number_mut: u32 = decimal_number;

    while decimal_number_mut >= to_base {
        base_vec.push(decimal_number_mut % to_base);
        decimal_number_mut /=  to_base;
    }
    base_vec.push(decimal_number_mut);
    base_vec.reverse();
    base_vec
}