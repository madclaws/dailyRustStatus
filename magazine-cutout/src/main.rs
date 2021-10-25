use magazine_cutout::*;

fn main() {
    println!("Can construct note => {}", can_construct_note(&"two times three is not four".split_whitespace()
    .collect::<Vec<&str>>(), &"two times two is four".split_whitespace()
    .collect::<Vec<&str>>()));
}