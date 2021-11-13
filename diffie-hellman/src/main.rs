use diffie_hellman::*;
fn main() {
    let p: u64 = 4_294_967_299;

    let g: u64 = 8;

    let private_key: u64 = 4_294_967_296;


    println!("{}", public_key(p, g, private_key));
}