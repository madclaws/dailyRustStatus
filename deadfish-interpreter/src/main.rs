fn main() {
    println!("Deadfish interpreter!");
    parse("iiisdosodddddiso");
}

fn parse(code: &str) -> Vec<i32> {
    let mut current_value: i32 = 0;
    let mut output_array: Vec<i32> = Vec::new();
    for symbol in code.chars() {
        match symbol {
            'i' => current_value += 1,
            'd' => current_value -= 1,
            's' => current_value *= current_value,
            'o' => output_array.push(current_value),
            _ => continue
        }
    }
    println!("The output array => {output_array:?}");
    output_array
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_tests() {
        assert_eq!(parse("iiisdoso"),
            vec![8, 64]);
        assert_eq!(parse("iiisdosodddddiso"),
            vec![8, 64, 3600]);
    }
}