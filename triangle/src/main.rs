use triangle::*;
fn main() {
    let sides = [3, 4, 4];
    let triangle = Triangle::build(sides).unwrap();
    println!("{}", triangle.is_equilateral())
}
