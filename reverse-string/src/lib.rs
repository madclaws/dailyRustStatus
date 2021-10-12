use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut reverse_string: String = "".to_owned();

    for c in input.graphemes(true) {
        println!("{}", c);
        reverse_string = format!("{}{}", c, reverse_string);
    }
    reverse_string
}
