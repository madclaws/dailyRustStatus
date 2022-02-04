// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.

use std::collections::HashMap;
use itertools::Itertools;

#[allow(clippy::new_without_default)]
pub struct School {
    students: HashMap<String, u32>
}

impl School {
    pub fn new() -> School {
        School{students: HashMap::new()}
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students.insert(String::from(student), grade);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut students_vec: Vec<_> = self.students.iter().map(|x| *x.1).unique().collect();
        println!("GRADES => {students_vec:?}");
        students_vec.sort_by(|a, b| a.cmp(b));
        students_vec   
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut students_vec: Vec<_> = self.students.iter().filter(|x| *x.1 == grade).collect();
        students_vec.sort_by(|(a, _a_val), (b, _b_val)| {
            a.cmp(b)
        });
        println!("Students list {students_vec:?}");
        let mut grade_vec: Vec<String> = Vec::new();
        for (student, _grade) in students_vec {
            grade_vec.push(student.clone())
        }
        grade_vec
    }
}
