pub fn translate(input: &str) -> String {
    let mut piggy_output: String = String::from("");
    let words: Vec<&str> = input.trim().split_whitespace().collect();
    for word in words.iter(){
        // Rule 4
        if word.len() == 2 && word.chars().nth(1) == Some('y') {
            let reverse_word: String = word.chars().rev().collect();
            piggy_output = format!("{}{}{}", piggy_output, reverse_word, "ay");
        } else if is_char_vowel_sound(word.chars().next().unwrap())
            || word.starts_with("xr") || word.starts_with("yt") {
            // Rule 2
            piggy_output = format!("{}{}{}", piggy_output, word, "ay");
        } else {
            // Rule 3
            let qu_split_list: Vec<&str> = word.split("qu").collect();
            if qu_split_list.len() == 2 {
                    if !qu_split_list[0].is_empty() &&
                        !is_char_vowel_sound(qu_split_list[0].chars().nth(qu_split_list[0].len() - 1).unwrap()){
                        piggy_output = format!("{}{}{}{}{}", piggy_output, qu_split_list[1], qu_split_list[0], "qu", "ay");
                    } else {
                        piggy_output = format!("{}{}{}{}", piggy_output, qu_split_list[1], "qu", "ay");
                    }
            } else {
                let consonant_cluster = get_consonant_cluster(word);
                if !consonant_cluster.is_empty() {
                    // Rule 2
                    let new_slice = &word[consonant_cluster.len()..word.len()];
                    piggy_output = format!("{}{}{}{}", piggy_output, new_slice, &consonant_cluster, "ay");
                }
            }
        }
        piggy_output.push(' ');
    }
    piggy_output.trim().to_owned()
}

fn is_char_vowel_sound(letter: char) -> bool {
    matches!(letter, 'a' | 'e' | 'i' | 'o' | 'u')
}

fn get_consonant_cluster(word: &str) -> String {
    let mut consonant_cluster: String = String::from("");
    for letter in word.chars() {
        if is_char_vowel_sound(letter) || (letter == 'y' && !word.starts_with('y')){
            break;
        }
        consonant_cluster.push(letter);
    }
    consonant_cluster
}

