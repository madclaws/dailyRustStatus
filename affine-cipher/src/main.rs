use affine_cipher::*;

fn main() {
    println!("{}", get_index_from_letter('z'));
    println!("{}", get_letter_from_index(3));
    println!("{:?}", encode("mindblowingly", 11, 15));
}