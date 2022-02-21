fn main() {
    println!("Chaos!");
    println!("Population growth\n {:?}", logistic_map(2.81, 51));
    println!("Steady State\n {:?}", logistic_map(2.0, 51));
    println!("Chaotic behavior\n {:?}", logistic_map(3.88, 51));
    println!("Death\n {:?}", logistic_map(1.0, 51));
    println!("Quick stable\n {:?}", logistic_map(2.0, 51));
    println!("Fluctuate stable\n {:?}", logistic_map(3.0, 51));
    println!("Divergence\n {:?}", logistic_map(3.6, 51));

}

fn logistic_map(r: f32, n: usize) -> Vec<f32> {
    let mut l_map: Vec<f32> = Vec::new();
    l_map.push(0.5);
    for nth in 1..=n {
        let xth = r * l_map[nth - 1] * (1.0 - l_map[nth - 1]);
        l_map.push(xth);
    }
    l_map
}
