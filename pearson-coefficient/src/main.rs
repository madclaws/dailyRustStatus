fn main() {
    println!("Pearson correlation coefficient");
    let pearson = correlation_coefficient(
        vec![
            5427.0, 5688.0, 6198.0, 6462.0, 6635.0, 7336.0, 7248.0, 7491.0, 8161.0, 8578.0, 9000.0,
        ],
        vec![
            18.079, 18.594, 19.753, 20.734, 20.831, 23.029, 23.597, 23.584, 22.525, 27.731, 29.449,
        ],
    );

    println!("Pearson correlation coefficient {pearson}")
}

fn correlation_coefficient(x_n: Vec<f32>, y_n: Vec<f32>) -> f32 {
    let mean_x = find_mean(&x_n);
    let mean_y = find_mean(&y_n);
    let cov_xy = find_covariance(&x_n, mean_x, &y_n, mean_y);
    let dev_x = find_standard_deviation(&x_n, mean_x);
    let dev_y = find_standard_deviation(&y_n, mean_y);

    cov_xy / (dev_x * dev_y)
}

fn find_mean(x_n: &[f32]) -> f32 {
    x_n.iter().sum::<f32>() / x_n.len() as f32
}

fn find_standard_deviation(x_n: &[f32], mean: f32) -> f32 {
    let mut deviation = 0.0;
    for i in x_n {
        deviation += (i - mean).powf(2.0);
    }
    (deviation / x_n.len() as f32).sqrt()
}

fn find_covariance(x_n: &[f32], mean_x: f32, y_n: &[f32], mean_y: f32) -> f32 {
    let mut cov_xy = 0.0;
    for i in 0..x_n.len() {
        cov_xy += (x_n[i] - mean_x) * (y_n[i] - mean_y)
    }
    cov_xy / x_n.len() as f32
}
