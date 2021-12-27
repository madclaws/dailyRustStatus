fn main() {
    println!("Sum consecutives!");
    sum_consecutives(&vec![-5, -5, 7, 7, 12, 0]);
}

/// Sums the numbers that are the same and consecutive.
fn sum_consecutives(numbers: &[i32]) -> Vec<i32> {
    let mut sum_list: Vec<i32> = Vec::new();
    let mut consecutive_sum: i32 = numbers[0];
    for index in 0..numbers.len() {
        if index > 0 {
            if numbers[index - 1] == numbers[index] {
                consecutive_sum += numbers[index]
            } else {
                sum_list.push(consecutive_sum);
                consecutive_sum = numbers[index];
            }
        }
        if index == numbers.len() - 1 {
            sum_list.push(consecutive_sum);
        }
    }
    sum_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        println!("Sample Tests");

        let input = vec![1, 4, 4, 4, 0, 4, 3, 3, 1];
        let expected = vec![1, 12, 0, 4, 6, 1];
        println!("Input: {:?}", input);
        assert_eq!(sum_consecutives(&input), expected);

        let input = vec![-5, -5, 7, 7, 12, 0];
        let expected = vec![-10, 14, 12, 0];
        println!("Input: {:?}", input);
        assert_eq!(sum_consecutives(&input), expected);
    }
}
