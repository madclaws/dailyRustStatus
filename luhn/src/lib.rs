/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.trim().len() <= 1 {
        return false
    }

    let mut num_vec: Vec<u32> = vec![0; 0];
    let mut index = 0;
    for n in code.trim().chars().rev() {
        if n == ' ' {
            continue;
        }
        if !n.is_numeric() {
            return false
        }
        let int_char: u32 = n.to_digit(10).unwrap();
        if index % 2 != 0 {
            let mut double_val = int_char * 2;
            if double_val > 9 {
                double_val -= 9;
            }
            num_vec.push(double_val);
        } else {
            num_vec.push(int_char);
        }
        index += 1;
    }
    
    let sum: u32 = num_vec.iter().sum();

    sum % 10 == 0
}
