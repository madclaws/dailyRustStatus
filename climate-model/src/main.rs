fn main() {
    println!("Rockstar climate model");
    println!("{:?}", calculate_planet_temperature(1361.0, 0.306, 0.612)); 

}

fn calculate_planet_temperature(S: f32, a: f32, e: f32) -> f32 {
    let sigma = 5.670374419 * f32::powf(10.0, -8.0);
    let numerator = (1.0 - a) * S;
    let denominator = 4.0 * e *  sigma;
    let t = (numerator / denominator).powf(1.0 / 4.0);
    f32::floor((t - 273.15) * f32::powf(10.0, 2.0))  / f32::powf(10.0, 2.0)
}

#[cfg(test)]

#[test]
fn test_temperature_of_earth() {
    assert_eq!(calculate_planet_temperature(1361.0, 0.306, 0.612), 14.05);
}

