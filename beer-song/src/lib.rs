pub fn verse(n: u32) -> String {
    format!("{}{}", get_first_sentence(n), get_second_sentence(n))
}

pub fn sing(start: u32, end: u32) -> String {
    let mut ultimate_verse: String = "".to_owned();
    for n in end..=start {
        if n != end {
            ultimate_verse = format!("{}\n{}", verse(n), ultimate_verse); 
        } else {
            ultimate_verse = format!("{}{}", verse(n), ultimate_verse); 
        }
    }
    ultimate_verse
}

fn get_first_sentence(n: u32) -> String {
    if n > 1 {
        format!("{} bottles of beer on the wall, {} bottles of beer.\n", n, n)
    } else if n == 1 {
        format!("1 bottle of beer on the wall, 1 bottle of beer.\n")
    } else {
        format!("No more bottles of beer on the wall, no more bottles of beer.\n")
    }
}

fn get_second_sentence(n: u32) -> String{
    if n > 2 {
        format!("Take one down and pass it around, {} bottles of beer on the wall.\n", n - 1)
    } else if n == 2 {
        format!("Take one down and pass it around, {} bottle of beer on the wall.\n", n - 1)
    } else if n == 1 {
        format!("Take it down and pass it around, no more bottles of beer on the wall.\n")
    } else {
        format!("Go to the store and buy some more, 99 bottles of beer on the wall.\n")
    }
}
