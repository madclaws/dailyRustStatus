fn main() {
    println!("English Beggars!");
    beggars(&[1, 2, 3, 4, 5], 1);
}

fn beggars(values: &[u32], n: usize) -> Vec<u32> {
    let mut beggar_count = 1;
    let mut beggar_list: Vec<u32> = Vec::new();
    let mut beggar_index = 0;

    while beggar_count <= n {
        let mut beggar_sum = 0;
        let mut beg_index = beggar_index;
        for index in 0..values.len() {
            if index == beg_index {
                beggar_sum += values[index];
                beg_index += n;
            }
        }
        beggar_list.push(beggar_sum);
        beggar_index += 1;
        beggar_count += 1;
    }
    beggar_list
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(beggars(&[1, 2, 3, 4, 5], 1), [15]);
        assert_eq!(beggars(&[1, 2, 3, 4, 5], 2), [9, 6]);
        assert_eq!(beggars(&[1, 2, 3, 4, 5], 3), [5, 7, 3]);
        assert_eq!(beggars(&[1, 2, 3, 4, 5], 6), [1, 2, 3, 4, 5, 0]);
    }
    
    #[test]
    fn test_zero_beggars() {
        assert_eq!(beggars(&[1, 2, 3, 4, 5], 0), []);
    }
}