use std::cmp::*;
use std::clone::*;
use std::ops::*;

#[derive(Debug)]
pub struct Triangle<T>{
    a: T,
    b: T,
    c: T
}

impl <T: Clone + PartialOrd + PartialEq + Add<Output = T> + Copy>Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let a =  sides[0];
        let b =  sides[1];
        let c =  sides[2];
        
        if (a + b) <= c || (a + c) <= b || (c + b) <= a {
            return None
        }
        Some(Triangle{a, b, c})
    }

    pub fn is_equilateral(&self) -> bool {
        if self.a == self.b && self.b == self.c {
            true
        } else {
            false
        }
    }

    pub fn is_scalene(&self) -> bool {
        if !self.is_equilateral() && !self.is_isosceles() {
            true
        } else {
            false
        }
    }

    pub fn is_isosceles(&self) -> bool {
        if self.a == self.b || self.a == self.c || self.b == self.c {
            true
        } else {
            false
        }
    }
}
