pub fn raindrops(n: u32) -> String {
    let mut final_string: String = "".to_owned();
    let mut is_any_factor: bool = false;
    if n % 3 == 0 {
        final_string.push_str("Pling");
        is_any_factor = true;
    } 
    if n % 5 == 0 {
        final_string.push_str("Plang");
        is_any_factor = true;
    } 
    if n % 7 == 0 {
        final_string.push_str("Plong");
        is_any_factor = true;
    } 

    if !is_any_factor {
        final_string.push_str(&n.to_string())
    }
    return final_string
}
