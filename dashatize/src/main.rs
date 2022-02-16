fn main() {
    println!("Dashatize it! {}", dashatize(6815));
}

fn dashatize(n: i64) -> String{
    let mut dashatized: String = String::from("");
    let mut is_last_odd: bool = false;
    let n_str = i64::abs(n).to_string();
    for str_char in n_str.chars() {
        if let Some(num) = str_char.to_digit(10) {
            if num % 2 != 0 {
                if !is_last_odd {
                    dashatized.push('-');
                }
                dashatized.push(str_char);
                dashatized.push('-');
                is_last_odd = true;
            } else {
                dashatized.push(str_char);
                is_last_odd = false;
            }
        }
    }
    dashatized.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(dashatize(274), "2-7-4");
        assert_eq!(dashatize(5311), "5-3-1-1");
        assert_eq!(dashatize(86320), "86-3-20");
        assert_eq!(dashatize(974302), "9-7-4-3-02");
    }
    
    #[test]
    fn weird() {
        assert_eq!(dashatize(0), "0");
        assert_eq!(dashatize(-1), "1");
        assert_eq!(dashatize(-28369), "28-3-6-9");                
    }
}
