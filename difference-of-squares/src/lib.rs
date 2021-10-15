pub fn square_of_sum(n: u32) -> u32 {
    let sum = (1..=n).fold(0, |sum, x| sum +x);
    u32::pow(sum, 2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).fold(0, |square_sum, x| square_sum + (u32::pow(x, 2)))
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
