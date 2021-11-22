#[derive(Debug)]
pub struct Triangle{
    a: u64,
    b: u64,
    c: u64
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides.to_vec().into_iter().any(|x| x < 1) {
            return None
        }
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
