use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut bracket_map: HashMap<char, char> = HashMap::new();
    let mut bracket_stack: Vec<char> = Vec::new();
    create_bracket_map(&mut bracket_map);
    for (_i, val) in string.chars().enumerate() {
        if bracket_stack.is_empty() && is_valid_character(&val){
            println!("bracket_stack empty, pushing {}", val);
            bracket_stack.push(val);
            continue;
        }
        match bracket_map.get(&val) {
            Some(&val_key) => {
              if bracket_stack.last() == Some(&val_key) {
                println!("bracket closed {} with {}", val, val_key);
                bracket_stack.pop();
              } else {
                println!("bracket_stack pushing {}", val);
                bracket_stack.push(val);
              }
            },
            _ => {
                if is_valid_character(&val) {
                    bracket_stack.push(val);
                    println!("pushing {}", val);
                } else {
                    println!("Invalid character {}", val);
                }
            }
        }
    }
    println!("{:?} bracket stack", bracket_stack);
    bracket_stack.is_empty()
}

fn create_bracket_map(bracker_map: &mut HashMap<char, char>) {
    bracker_map.insert(']', '[');
    bracker_map.insert(')', '(');
    bracker_map.insert('}', '{');
}

fn is_valid_character(chr: &char) -> bool {
    return matches!(chr, '[' | ']' | '{' | '}' | ')' | '(');
}
