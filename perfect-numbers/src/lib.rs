use std::cmp::Ordering; 

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num < 1 {
        return None
    }
    match num.cmp(&get_aliquot_sum(num)) {
        Ordering::Equal => Some(Classification::Perfect),
        Ordering::Less => Some(Classification::Abundant),
        Ordering::Greater => Some(Classification::Deficient),
    }
}

fn get_aliquot_sum(num: u64) -> u64 {
    let mut aliquot_sum: u64 = 0;
    for elem in 1..num {
        if num % elem == 0 {
            aliquot_sum += elem
        }
    }
    aliquot_sum
}
