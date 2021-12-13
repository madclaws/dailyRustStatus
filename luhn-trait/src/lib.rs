use std::fmt::*;

pub trait Luhn {
    fn valid_luhn(&self) -> bool;

    fn caluclate_new_val(&self, index: usize, current_val: u32) -> u32 {
        if index % 2 != 0 {
            let mut double_val = current_val * 2;
            if double_val > 9 {
                double_val -= 9;
            }
            double_val
        } else {
            current_val
        }
    }
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: Display> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let mut checksum: u32 = 0;
        let code_list: Vec<_> = self.to_string()
        .chars()
        .filter(|c| *c != ' ')
        .map(|c| c.to_digit(10))
        .collect();
    
        if code_list.len() <= 1 {
            return false
        }
        for (index, wrapped_val) in code_list.into_iter().rev().enumerate() {
            match wrapped_val {
                Some(val) => {
                   checksum += self.caluclate_new_val(index, val);
                }
                _ => {
                    return false;
                }
            }
        }
        checksum % 10 == 0
       
    }
}
