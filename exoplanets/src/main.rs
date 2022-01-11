fn main() {
    println!("Habitable exoplanets");
    println!("{}", find_habitable_status(1.5, 2.5)); 
}

fn find_habitable_status(luminosity: f32, distance: f32) -> String {
    let inner_radius = f32::sqrt(luminosity / 1.1);
    let outer_radius = f32::sqrt(luminosity / 0.54);
    if distance >= inner_radius && distance <= outer_radius {
        return "just right".to_owned();
    } else if distance < inner_radius {
        return "too hot".to_owned();
    } else {
        return "too cold".to_owned();
    }
}
