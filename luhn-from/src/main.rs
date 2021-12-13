use luhn_from::*;

fn main() {
    // let luhn = Luhn::from(456);
    // println!("{:?}", luhn.is_valid());

    let luhn1 = Luhn::from("046 454 287");
    println!("{:?}", luhn1.is_valid());

}