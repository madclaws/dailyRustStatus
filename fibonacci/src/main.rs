fn main() {
    println!("fibonacci number at {} is {}",3, fibonacci(2));
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n
    }

    fibonacci(n-1) + fibonacci(n -2)
}

#[test]
fn nth_fibonacci() {
    assert_eq!(fibonacci(3), 2);
    assert_eq!(fibonacci(10), 55);
    assert_eq!(fibonacci(14), 377);
}