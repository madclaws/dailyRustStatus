pub fn reply(message: &str) -> &str {
    let message = message.trim().to_owned();
    
    if message.is_empty() {
        "Fine. Be that way!"
    } else if !is_all_letter_numeric(&message) && message == message.to_uppercase() && message.ends_with('?'){ 
        "Calm down, I know what I'm doing!"
    } else if !is_all_letter_numeric(&message) && message == message.to_uppercase() {
        "Whoa, chill out!"
    } else if message.ends_with('?') {
        "Sure."
    } else {
        "Whatever."
    }
}

fn is_all_letter_numeric(message: &str) -> bool {
    for (_, val) in message.chars().enumerate() {
        if val.is_alphabetic() {
            return false
        }
    }
    true
}