pub fn number(user_number: &str) -> Option<String> {
    let mut filtered_number: String = user_number
        .split(|c: char| !c.is_numeric())
        .collect::<Vec<&str>>()
        .join("");
    if filtered_number.len() < 10 {
        return None;
    }

    if filtered_number.starts_with('1') && filtered_number.len() == 11 {
        filtered_number = filtered_number.trim_start_matches('1').to_string();
    }

    if filtered_number
        .chars()
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap()
        > 1
        && filtered_number
            .chars()
            .nth(3)
            .unwrap()
            .to_digit(10)
            .unwrap()
            > 1
        && filtered_number.len() == 10
    {
        return Some(filtered_number);
    }

    None
}
