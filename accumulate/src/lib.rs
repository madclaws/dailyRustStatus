// use std::fmt::Debug;

pub fn map<T, O, F: FnMut(T) -> O>(input: Vec<T>, mut function: F) -> Vec<O> {
    let mut output: Vec<O> = Vec::new();
    for val in input {
        output.push(function(val)); 
    }
    output
}
