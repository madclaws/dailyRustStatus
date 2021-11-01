use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut vector_word: Vec<char>  = word.chars().map(|x| x.to_lowercase().next().unwrap()).collect();
    let mut anagram_set: HashSet<&'a str> = HashSet::new();
    vector_word.sort_unstable();

    for (_i, val) in possible_anagrams.iter().enumerate() {
        let mut input_vector: Vec<char>  = val.chars().map(|x| x.to_lowercase().next().unwrap()).collect();
        if String::from(*val).to_lowercase() == String::from(word).to_lowercase() {
            println!("Same word, no fun, continue");
            continue;
        }
        input_vector.sort_unstable();
        
        if vector_word == input_vector {
            anagram_set.insert(val);
        }
    }
    anagram_set
}
