use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let  mut word_count_map: HashMap<String, u32> = HashMap::new();
    let word_vec: Vec<&str> = words.trim().split(&['-', ' ', ':', '@', '\n', '.',
    '?', '!', '\"', ',', '&', '@', '$', '%', '^', '&'][..]).filter(|e| !e.is_empty()).collect();
    for word in word_vec.iter() {
        word_count_map.entry(word.trim_matches('\'').to_lowercase())
        .and_modify(|e| *e += 1)
        .or_insert(1);
    }
    println!("{:?}", word_count_map);
    word_count_map
}
