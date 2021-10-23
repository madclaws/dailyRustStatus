pub fn nth(n: u32) -> u32 {
    let mut prime_vector: Vec<u32> = vec![0; 0];
    for num in 2..{
        if prime_vector.len() as u32 == n +1 {
            break;
        }
        if is_prime(num) {
            prime_vector.push(num)
        }
    }
    *prime_vector.last().unwrap()
}

pub fn is_prime(num: u32) ->  bool {
    for n in 2..=(num/2) {
        if num % n == 0 {
            return false
        }
    }
    true
}
