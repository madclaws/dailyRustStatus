use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    calculate_modular_exponentiation(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    calculate_modular_exponentiation(b_pub, a, p)
}

fn calculate_modular_exponentiation(x: u64, y: u64, modulus: u64) -> u64{
    let mut result: u64 = 1;
    let mut a: u64 = x;
    let mut b: u64 = y;

    a %= modulus;
    if a == 0 {
        return 0
    }
    loop{
        if b == 0 {
            break;
        }
        if b % 2 != 0 {
            result = (result * a) % modulus;
        }
        b /= 2;
        a = (a * a) % modulus;
    }
    result
}
