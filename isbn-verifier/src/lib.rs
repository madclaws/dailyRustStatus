/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let processed_isbn = isbn.split('-').collect::<Vec<&str>>().join("");
    if processed_isbn.len() != 10 {
        return false
    }
    let mut multiplier = 10;
    let mut isbn_sum = 0;
    for chr in processed_isbn.chars() {
        if chr == 'X' && multiplier == 1 {
            isbn_sum += 10 * multiplier;
        } else if !chr.is_numeric() {
            return false
        } else {
            isbn_sum += chr.to_digit(10).unwrap() * multiplier;
        }
        multiplier -= 1;
    }
    isbn_sum % 11 == 0
}
