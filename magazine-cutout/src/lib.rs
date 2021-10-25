// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    // Building magazine word count hashmap
    let mut magazine_word_count: HashMap<&str, i32>  = HashMap::new();
    for (index, value) in magazine.iter().enumerate(){
        magazine_word_count.entry(*value)
            .and_modify(|e| {*e += 1})
            .or_insert(1);
    }
  
    // Iterating through the notes and checking if each word can be used from the hashmap
    // As soon as a word is used from hashmap, its count is decremented until its 0 and removed 
    for (index, value) in note.iter().enumerate() {
        match magazine_word_count.get(value) {
            Some(&word_count) => {
                // check if word_count is 0, then delete the entry and return false. Else decrement 1
                if word_count == 0 {
                    magazine_word_count.remove(value);
                    return false
                } else {
                    magazine_word_count.insert(value, word_count - 1);
                }
            },
            _ =>  return false
        }
    }
    true
}
