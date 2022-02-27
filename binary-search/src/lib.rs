use std::cmp::{PartialEq, PartialOrd};

pub fn find<T: PartialEq + PartialOrd>(array: &[T], key: T) -> Option<usize> {
    if array.is_empty() {
        return None
    }
    let result = binary_search(&array, 0, (array.len() - 1) as i32, key);
    if result == -1 {
        None
    } else {
        Some(result as usize)
    }
}

fn binary_search<T: PartialEq + PartialOrd>(array: &[T], left: i32, right: i32, key: T) -> i32{
    if right >= left {
        let middle = left + (right - left) / 2;
        if array[middle as usize] == key {
            return middle
        }

        if array[middle as usize] > key {
            return binary_search(&array, left, middle - 1, key)
        }

        return binary_search(&array, middle + 1, right, key)
    }
    -1
}