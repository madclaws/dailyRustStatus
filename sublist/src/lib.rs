use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match first_list.len().cmp(&second_list.len()) {
        Ordering::Greater=> {
            let result = compare(first_list, second_list, false);
            if result == Comparison::Sublist {
                return Comparison::Superlist
            }
            result
        },
        Ordering::Less => {
            compare(second_list, first_list, false)
        },
        Ordering::Equal => {
            compare(second_list, first_list, true)
        }
    }
}

fn compare<T: PartialEq>(large_list: &[T], small_list: &[T], is_equal: bool) -> Comparison {
    let mut s_index: usize = 0;
    let mut l_index: usize = 0;
    let mut l_start_index: usize = 0;
    let mut cc: usize = 0;
    loop {
        if s_index == small_list.len() {
            break;
        }
        if l_index == large_list.len() {
            break;
        }
        if small_list[s_index] == large_list[l_index] {
            cc += 1; s_index += 1; l_index += 1;
        } else {
            cc = 0; s_index = 0; l_start_index += 1; l_index = l_start_index
        }
    }

    if cc == small_list.len() && !is_equal{
        Comparison::Sublist
    } else if cc == small_list.len() && is_equal{
        Comparison::Equal
    } else {
        Comparison::Unequal
    }
}
