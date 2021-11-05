/// Determine whether a sentence is a pangram.
use std::collections::HashMap;
pub fn is_pangram(sentence: &str) -> bool {
    let mut pangram_hash: HashMap<char, usize> = HashMap::new();
    for (_i, val) in sentence.chars().enumerate() {
        if !val.is_alphabetic() || !val.is_ascii() {
            continue;
        }
        pangram_hash.entry(val.to_lowercase().next().unwrap())
            .and_modify(|e| { *e += 1 })
            .or_insert(1);
    }
    if pangram_hash.keys().len() == 26 {
        return true
    }
    false
}
