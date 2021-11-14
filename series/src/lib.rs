pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut start_index: u32 = 0;
    let mut current_index: u32 = 0;
    let mut consec_count: u32 = 0;
    let mut series_vec: Vec<String> = Vec::new();
    let mut tmp_string: String = String::from("");

    loop {
        if consec_count == len as u32 && current_index > 0 {
            consec_count = 0;
            series_vec.push(String::from(&tmp_string));
            start_index += 1;
            current_index = start_index;
            tmp_string = String::from("");
        }
        if let Some(val) = digits.chars().nth(current_index as usize) {
            current_index += 1;
            if len > 0 {
                tmp_string.push(val);
                consec_count += 1;
            } else {
                tmp_string.push_str("");
            }
        } else {
            if len == 0 {
                series_vec.push(String::from(""));
            }
            return series_vec
        }
    }
}
