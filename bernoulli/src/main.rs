fn main() {
    println!("Note G, Bernoulli number {}", calculate_bernoulli(2));
}

fn calculate_bernoulli(n: u32) -> f32{
    let mut summation: f32 = 0.0;
    let mut summation_history: f32 = 0.0;
    if n == 0 {
        return 1.0
    }
    if n == 1 {
        return -0.5
    }
    for k in 0..n {
        let nth_bernoulli: f32;
        if k == 0 {
            nth_bernoulli = 1.0;
        } else if k == 1 {
            nth_bernoulli = -0.5;
        } else {
            nth_bernoulli = summation_history;
        }
        println!("{}th bernoulli => {}", k, nth_bernoulli);
        summation += binomial_coefficient(n, k) * (nth_bernoulli / (n + 1 - k) as f32); 
        summation_history = -summation;
        println!("summation history {}", summation_history);
    }
    - summation
}


fn binomial_coefficient(n: u32, k: u32) -> f32 {
    factorial(n) / (factorial(k) * factorial(n - k)) as f32
}

fn factorial(n: u32) -> f32 {
    if n == 0 {
        return 1.0
    }
    n as f32 * factorial(n - 1)
}

#[cfg(test)]

#[test]
fn test_calculate_bernouli() {
    assert_eq!(calculate_bernoulli(0), 1.0);
    assert_eq!(calculate_bernoulli(1), -0.5);
    assert_eq!(calculate_bernoulli(2), 0.16666666);
}