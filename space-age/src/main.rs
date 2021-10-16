use space_age::*;
fn main() {
    println!("Space age");
    println!("{:?}", Duration::from(1_000));
    println!("{:?}", Earth::years_during(&Duration::from(2_134_835_688)))   

}