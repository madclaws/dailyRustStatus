use rand::{Rng};
use std::{collections::HashSet, sync::Mutex};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref COLLISON_CHECK: Mutex<HashSet<String>>  = Mutex::new(HashSet::new());
}

#[derive(Debug)]
pub struct Robot{
    name: String
}

impl Robot {
    pub fn new() -> Self {
        Robot{name: Robot::generate_rand_name()}
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        COLLISON_CHECK.lock().unwrap().remove(&self.name);
        self.name = Robot::generate_rand_name();
    }

    fn generate_rand_name() -> String {
        let mut name: String = String::from("");
        let mut collision_set = COLLISON_CHECK.lock().unwrap();
        loop {
            let a = rand::thread_rng().gen_range(65..91) as u8;
            let b = rand::thread_rng().gen_range(65..91) as u8;
            name.push(a as char);
            name.push(b as char);
    
            name.push(rand::thread_rng().gen_range(0..10).to_string().chars().nth(0).unwrap());
            name.push(rand::thread_rng().gen_range(0..10).to_string().chars().nth(0).unwrap());
            name.push(rand::thread_rng().gen_range(0..10).to_string().chars().nth(0).unwrap());
            if collision_set.insert(name.clone()) {
                return name
            }
        }
    }
}
