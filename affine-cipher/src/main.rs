use affine_cipher::*;

fn main() {
    println!("{}", get_index_from_letter('z'));
    println!("{}", get_letter_from_index(3));
    println!("{:?}", encode("test", 5, 7));
    println!("{:?}", calculate_mmi(5, 26));
    println!("{:?}", decode("tytgn fjr", 3, 7));
}