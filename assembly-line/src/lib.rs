// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let speed: u32 = speed as u32;
    let total_car_produced = (speed * 221) as f64;
    let production_rate = total_car_produced * get_success_rate(speed);
    println!("production_rate {}", production_rate);
    production_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    ((production_rate_per_hour(speed) / 60.0)) as u32

}

fn get_success_rate(speed: u32) -> f64 {
    if speed <= 0 {
        return 0.0
    } else if speed > 0 && speed <= 4 {
        return 1.0
    } else if speed > 4 && speed <= 8 {
        return 0.9
    } else {
        return 0.77
    }
}


