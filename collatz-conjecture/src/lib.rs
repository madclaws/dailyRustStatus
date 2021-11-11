pub fn collatz(n: u64) -> Option<u64> {
    let mut steps: u64 = 0;
    let mut num: u64 = n;
    if num < 1 {
        return None
    }
    loop {
        if num == 1 {
            break;
        }
        if num % 2 == 0 {
            num /= 2;
        } else {
            match num.checked_mul(3) {
            Some(val) => {
                match val.checked_add(1) {
                    Some(val) => num = val,
                    None => return None
                }
            },
            None => return None    
            };
        }
        match steps.checked_add(1) {
            Some(val) => steps = val,
            None => return None
        }
    }
    Some(steps)
}
