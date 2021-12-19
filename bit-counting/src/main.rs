fn main() {
    count_bits(2);
}

fn count_bits(n: i64) -> u32 {
    let mut base_vec: Vec<u32> = Vec::new();
    let mut decimal_number_mut: u32 = n as u32;

    while decimal_number_mut >= 2 {
        base_vec.push(decimal_number_mut % 2);
        decimal_number_mut /=  2;
    }
    base_vec.push(decimal_number_mut);
    base_vec.reverse();
    let bit_count = base_vec.into_iter().fold(0, |mut count, bit| {
        if bit == 1 {
            count += 1;
        }
        count
    });
    bit_count
}

#[test]
fn returns_expected() {
    assert_eq!(count_bits(0), 0);
    assert_eq!(count_bits(4), 1);
    assert_eq!(count_bits(7), 3);
    assert_eq!(count_bits(9), 2);
    assert_eq!(count_bits(10), 2);
}