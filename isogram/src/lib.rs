use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    let mut candidate_count: HashMap<char, i32> = HashMap::new();
    
    for (_i, val) in candidate.chars().enumerate() {
        if val != ' ' && val != '-' {
            candidate_count.entry(val.to_lowercase().next().unwrap())
            .and_modify(|e| {*e += 1})
            .or_insert(1);
        }
    }
    for (_i, val) in candidate_count.iter() {
        if val > &1 {
            return false
        }
    } 
    true
}
