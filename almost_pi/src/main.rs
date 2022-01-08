fn main() {
    println!("Almost PI");
    println!("{}",  almost_pi(25));
}


fn almost_pi(n: u32) -> f64 {
    let pi_value: f64 = (0..n).into_iter()
        .fold(0.0, |mut sum, k| {sum += (f64::powf(-1.0, k as f64)) / (2.0 * k as f64 + 1.0); sum});
    pi_value * 4.0
}

#[cfg(test)]

#[test]
fn nametesting_almost_pi() {
    assert_eq!(almost_pi(25), 3.1815766854350325);
    assert_eq!(almost_pi(1000), 3.140592653839794);
}