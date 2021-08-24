pub fn factors(n: u64) -> Vec<u64> {
    let mut dividend = n;
    let mut divisor = 2;
    let mut prime_factors: Vec<u64> = Vec::new();

    while dividend > 1 {
        let quotient = dividend / divisor;
        let remainder = dividend % divisor;
        if remainder == 0 {
            prime_factors.push(divisor);
            dividend = quotient;
        } else {
            divisor += 1;
        }
    }
    prime_factors
}
