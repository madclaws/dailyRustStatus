use anagram::*;

fn main() {
    println!("{:?}", anagrams_for("Listen", &["enlists", "google", "inlets", "banana", "listen"]));
}