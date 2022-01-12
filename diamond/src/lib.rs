pub fn get_diamond(c: char) -> Vec<String> {
    let mut diamond: Vec<String> = Vec::new();
    let diamond_width = get_diamond_width(c);
    let mut start_pos: u32 = diamond_width / 2;
    for letter in 'A'..=c {
        diamond.push(create_row_for_letter(letter, start_pos, diamond_width));
        start_pos = start_pos.saturating_sub(1);
    }
    if c != 'A' {
        for row in diamond.clone().into_iter().rev().skip(1) {
            diamond.push(row);
        }
    }
    render_diamond(&diamond); 
    diamond
}

fn render_diamond(diamond: &[String]) {
    for row in diamond {
        println!("{}", row);
    }
}

fn get_letter_value(letter: char) -> u32 {
    letter as u32 - 65
}

fn get_diamond_width(letter: char) -> u32 {
    get_letter_value(letter) * 2 + 1
}

fn create_row_for_letter(letter: char, start_pos: u32, width: u32) -> String {
    let mut row: String = String::with_capacity(width as usize);
    let mut end_pos = start_pos;
    if letter != 'A' {
        end_pos = start_pos + (1 + (get_letter_value(letter) - 1) * 2) + 1;
    }

    for index in 0..width {
        if index == start_pos || index == end_pos {
            row.push(letter);
        } else {
            row.push(' ');
        }
    }
    row
}
