fn main() {
    println!("Earthquake Epicenters");
    calculate_trilateration(8.382353226, -58.003720814, 12.860754193, 
        -13.590571819, 73.976069096, 22.847488548, 77.291172370, 7.508764456, 5.767809783);
}


fn calculate_trilateration(x1: f64, y1: f64, t1: f64, x2: f64, y2: f64, t2: f64, x3: f64, y3: f64, t3: f64) 
    -> (f64, f64){
    let r1 = t1 * 6.0;
    let r2 = t2 * 6.0;
    let r3 = t3 * 6.0;

    let a = -2.0 * x1 + 2.0 * x2;
    let b = -2.0 * y1 + 2.0 * y2;
    let c = r1.powf(2.0) - r2.powf(2.0) - x1.powf(2.0) + x2.powf(2.0) - y1.powf(2.0) + y2.powf(2.0);
    let f = r2.powf(2.0) - r3.powf(2.0) - x2.powf(2.0) + x3.powf(2.0) - y2.powf(2.0) + y3.powf(2.0);
    let d = -2.0 * x2 + 2.0 * x3;
    let e = -2.0 * y2 + 2.0 * y3;

    let x = (c * e - f * b) / (e * a - b * d);
    let y = (c * d - a * f) / (b * d - a * e);
    println!("x => {}, y => {}", x, y);
    (x, y)
}