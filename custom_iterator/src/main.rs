// use custom_iterator::*;
mod lib;
use lib::*;
fn main() {
    println!("Custom Iterators!");
    // println!("{:?}", create_tree(5, 10).traverse());
    create_tree('a', 'b').traverse();
}
