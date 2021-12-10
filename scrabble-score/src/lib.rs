use std::collections::HashMap;

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let mut letter_count: HashMap<char, u64> = HashMap::new();
    let mut scrabble_score: u64 = 0;
    for letter in word.to_lowercase().chars(){
        letter_count.entry(letter)
        .and_modify(|e| *e += 1)
        .or_insert(1);
    }
    for (k, v) in letter_count {
        scrabble_score += get_value(k) * v;
    }
    scrabble_score
}

fn get_value(letter: char) -> u64 {
    if matches!(letter, 'a' | 'e'| 'i' | 'o' | 'u' | 'l' | 'n' | 'r'| 's'| 't') {
        1
    } else if matches!(letter, 'd' | 'g') {
        2
    } else if matches!(letter, 'b' | 'c' | 'm' | 'p') {
        3
    } else if matches!(letter, 'f' | 'h' | 'v' | 'w' | 'y') {
        4
    } else if matches!(letter, 'k') {
        5
    } else if matches!(letter, 'j' | 'x') {
        8
    } else if matches!(letter, 'q' | 'z'){
        10
    } else {
        0
    }
}
