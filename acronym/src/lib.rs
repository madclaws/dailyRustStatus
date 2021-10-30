pub fn abbreviate(phrase: &str) -> String {
    let mut abbrevation: String = String::from("");
    for (_i, word) in phrase.trim().split(|c: char| c == ' ' || c == '-').enumerate() {
        let mutable_word = word.trim_start_matches('_').trim_end_matches('_');
        let mut letter_gap: i32 = 0;
        for (i, chr) in mutable_word.chars().enumerate() {
            if i == 0 {
                abbrevation.push(chr.to_uppercase().next().unwrap());
            } else if chr.is_uppercase() && letter_gap > 0{
                abbrevation.push(chr);
                letter_gap = 0;
            } else if chr.is_uppercase(){
                letter_gap = 0;
            } else {
                letter_gap += 1;
            }
        } 
    }
    abbrevation
}
