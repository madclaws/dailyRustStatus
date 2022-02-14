fn main() {
    println!("Note G, Bernoulli number");
    calculate_nth_bernoulli(8, 8-1);
}


fn calculate_nth_bernoulli(n: u32, k: u32) -> f32{
    if k == 0 {
        return 1.0
    }

    if k == 1 {
        return -0.5
    }

    calculate_binomial_coefficient(n, k - 1) * calculate_nth_bernoulli(n, k - 2) / (n + 1  - k) as f32

}

fn calculate_binomial_coefficient(n: u32, k: u32) -> f32 {
    factorial(n) / (factorial(k) * factorial(n - k)) as f32
}

fn factorial(n: u32) -> f32 {
    if n == 0 {
        return 1.0
    }
    n as f32 * factorial(n - 1)
}

