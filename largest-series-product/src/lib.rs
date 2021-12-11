
#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1)
    }
    if string_digits.len() < span {
        return Err(Error::SpanTooLong)
    }
    let char_digits: Vec<char> = string_digits.chars().collect();
    let mut largest_product: u64 = 0;
    for slice in char_digits.windows(span) {
       let mut window_product: u64 = 1;
       for index in 0..slice.len() {
            if slice[index].is_numeric() {
                window_product *= slice[index].to_digit(10).unwrap() as u64;
            } else {
                return Err(Error::InvalidDigit(slice[index]))
            }
       }
       if window_product > largest_product {
           largest_product = window_product;
       }

    }
    println!("largest product {}", largest_product);
    Ok(largest_product)
}
