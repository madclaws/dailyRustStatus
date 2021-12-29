fn main() {
    println!("Calculate string rotation!");
    shifted_diff("Esham", "Esham");
}

fn shifted_diff(first: &str, second: &str) -> Option<usize> {
    let mut first_list: Vec<char> = first.chars().collect();
    let mut diff: Option<usize> = Some(0);
    if first == second {
        return diff
    }
    for _index in 0..first_list.len() {
        first_list.rotate_right(1); 
        let first_rotated: String = first_list.to_owned().into_iter().collect();
        diff = Some(diff.unwrap() + 1);
        if first_rotated == second {
            return diff
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(shifted_diff("eecoff", "coffee"), Some(4));
        assert_eq!(shifted_diff("Moose", "moose"), None);
        assert_eq!(shifted_diff("isn't", "'tisn"), Some(2));
        assert_eq!(shifted_diff("Esham", "Esham"), Some(0));
        assert_eq!(shifted_diff(" ", " "), Some(0));
        assert_eq!(shifted_diff("hoop", "pooh"), None);
        assert_eq!(shifted_diff("  ", " "), None);
    }
}

