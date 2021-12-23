fn main() {
    println!("Max product! {}", adjacent_elements_product(&[-23, 4, -5, 99, -27, 329, -2, 7, -921]));
}

fn adjacent_elements_product(list: &[i32]) -> i32 {
    let mut max: Option<i32> = None;
    for adjacent_list_iter in list.windows(2) {
        let mut product = 1;
        for val in adjacent_list_iter {
            product *= val
        }
        
        if max.is_none() || product > max.unwrap() {
            max = Some(product)
        }
    }
    max.unwrap()
}

#[test]
fn positive_numbers() {
    assert_eq!(adjacent_elements_product(&[5, 8]), 40);
    assert_eq!(adjacent_elements_product(&[1, 2, 3]), 6);
    assert_eq!(adjacent_elements_product(&[1, 5, 10, 9]), 90);
    assert_eq!(adjacent_elements_product(&[4, 12, 3, 1, 5]), 48);
    assert_eq!(adjacent_elements_product(&[5, 1, 2, 3, 1, 4]), 6);
}

#[test]
fn mixed_values() {
    assert_eq!(adjacent_elements_product(&[3, 6, -2, -5, 7, 3]), 21);
    assert_eq!(adjacent_elements_product(&[9, 5, 10, 2, 24, -1, -48]), 50);
    assert_eq!(adjacent_elements_product(&[5, 6, -4, 2, 3, 2, -23]), 30);
    assert_eq!(
        adjacent_elements_product(&[-23, 4, -5, 99, -27, 329, -2, 7, -921]),
        -14
    );
    assert_eq!(adjacent_elements_product(&[5, 1, 2, 3, 1, 4]), 6);
}

#[test]
fn containing_zeroes() {
    assert_eq!(adjacent_elements_product(&[1, 0, 1, 0, 1000]), 0);
    assert_eq!(adjacent_elements_product(&[1, 2, 3, 0]), 6);
}