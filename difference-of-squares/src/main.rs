fn main() {
    println!("{:?}", (1..=10).fold(0, |sum, x| sum + (i32::pow(x, 2))));}