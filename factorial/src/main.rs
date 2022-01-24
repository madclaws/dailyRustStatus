fn main() {
    println!("Calculate factorial!");
    let input = 10;
    println!("Factorial of {input} => {}", factorial(input));
}

fn factorial(n: u64) -> u64 {
    if n == 0 {
        return 1
    }
    n * factorial(n - 1)
}

#[cfg(test)]

#[test]
fn testing_factorial_calculation() {
    assert_eq!(factorial(10), 3628800);
    assert_eq!(factorial(5), 120);
    assert_eq!(factorial(7), 5040);
}