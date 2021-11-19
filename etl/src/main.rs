use etl::*;
use std::collections::BTreeMap;

fn main() {
    // let input = input_from(&[(1, vec!['A'])]);
    let input = input_from(&[(1, vec!['A', 'E', 'I', 'O', 'U'])]);
    transform(&input);
}

fn input_from(v: &[(i32, Vec<char>)]) -> BTreeMap<i32, Vec<char>> {
    v.iter().cloned().collect()
}