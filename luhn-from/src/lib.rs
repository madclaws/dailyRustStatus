use std::fmt::*;

#[derive(Debug)]
pub struct Luhn<T>{
    code: T
}

impl <T: Display>Luhn<T> {
    pub fn is_valid(&self) -> bool {
        let mut checksum: u32 = 0;
        let code_list: Vec<_> = self.code.to_string()
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
                   checksum += caluclate_new_val(index, val);
                }
                _ => {
                    return false;
                }
            }
        }
        checksum % 10 == 0
    }
}

fn caluclate_new_val(index: usize, current_val: u32) -> u32 {
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
/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> From<T> for Luhn<T> {
    fn from(input: T) -> Self {
        Luhn{code: input}
    }
}
