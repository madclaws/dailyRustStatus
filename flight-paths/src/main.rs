fn main() {
    println!("Flight Paths!");
    println!("{:?}", haversine_distance(46.283, 86.667, -48.877, 123.393));
}

fn haversine_distance(lat1: f64, long1: f64, lat2: f64, long2: f64) -> f64 {
    let delta_latitude = to_radian(lat2) - to_radian(lat1);
    let delta_longitude = to_radian(long2) - to_radian(long1);

    let half_chord_distance = (delta_latitude / 2.0).sin().powf(2.0) + to_radian(lat1).cos()
    * to_radian(lat2).cos() * (delta_longitude / 2.0).sin().powf(2.0);

    let angular_distance = 2.0 * f64::atan2(half_chord_distance.sqrt(), (1.0 - half_chord_distance).sqrt());
    6372.1 * angular_distance
}

fn to_radian(degree: f64) -> f64 {
    (std::f64::consts::PI / 180.0) * degree
}