use crypto_square::*;

fn main() {
    encrypt("If man was meant to stay on the ground, god would have given us roots.");   
    println!("dimensions {:?}", find_rec_dimensions(54)); 
}