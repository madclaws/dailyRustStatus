fn main() {
    println!("Exponential growth using differential equation");
    find_solution(5.0, 1.0, 0.6, 3);
    find_solution(1.0, 2.5, 0.1, 5);
    
    // No growth or decay
    find_solution(8.515985510991687, 0.0, 0.2517793744979713, 13);

    // Slow growth, small K
    find_solution(1.8842832432558776, 0.09370855540922123, 0.15558962674676496, 43);

    //  Fast grwoth
    find_solution(-3.092767959657019, 4.875798380359455, 0.12622160405021277, 20);

    find_solution(1.125816437756065, -2.5, 0.7, 26);

}

fn find_solution(x_0: f32, k: f32, dt: f32, n: u32) -> Vec<f32> {
    let mut current_x_val = x_0;
    let mut solutions: Vec<f32> = Vec::new();
    solutions.push(current_x_val);
    for _index in 1..=n {
        let nth_solution = current_x_val + (k * current_x_val * dt);
        current_x_val = nth_solution;
        solutions.push(nth_solution);
    }
    println!("{solutions:?}");
    solutions
}
