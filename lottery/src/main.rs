fn main() {
    println!("Lottery Ticket!");
    println!("{}", bingo(&[("HGTYRE", 74), ("BE", 66), ("JKTY", 74)], 3));
}

fn bingo(ticket: &[(&str, u8)], win: usize) -> &'static str {
    let mut total_mini_wins = 0;
    for (mini_string, mini_number) in ticket {
        for mini_char in mini_string.chars() {
            if mini_char as i32 == *mini_number as i32 {
                total_mini_wins += 1;
                break;
            }
        }
    }
    if total_mini_wins >= win {
        return "Winner!"
    } 
    "Loser!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(bingo(&[("ABC", 65), ("HGR", 74), ("BYHT", 74)], 2), "Loser!");
        assert_eq!(bingo(&[("ABC", 65), ("HGR", 74), ("BYHT", 74)], 1), "Winner!");
        assert_eq!(bingo(&[("HGTYRE", 74), ("BE", 66), ("JKTY", 74)], 3), "Loser!");
    }
}