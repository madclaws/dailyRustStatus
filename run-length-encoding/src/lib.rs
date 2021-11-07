pub fn encode(source: &str) -> String {
    let mut encoded_string: String = String::from("");    
    let mut char_count: u32 = 0;
    let mut last_char: char = '\0';
    for (_i, val) in source.chars().enumerate() {
        if val != last_char && last_char != '\0' {
            if char_count > 1 {
                encoded_string.push_str(&char_count.to_string());
            }
            encoded_string.push(last_char);
            char_count = 1;
        } else {
            char_count += 1;
        }
        last_char = val;
    }

    if char_count > 1 {
        encoded_string.push_str(&char_count.to_string());
    }
    if last_char == '\0' {
        encoded_string.push_str("");
    } else {
        encoded_string.push(last_char);
    }

    encoded_string
}

pub fn decode(source: &str) -> String {
    let mut decode_string: String = String::from("");

    let mut pre_count: String = String::from("");

    for (_i, val) in source.chars().enumerate() {
        if char::is_numeric(val) {
            pre_count.push(val);
            
        } else {
            let count: i32 = match pre_count.parse() {
                Ok(value) => value,
                _ => 1
            };
            let char_str: String = char::to_string(&val); 
            decode_string.push_str(&char_str.repeat(count as usize));
            println!("precount {}", count);
            pre_count = "".to_owned();
        } 
    }

    decode_string
}
