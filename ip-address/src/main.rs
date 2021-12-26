fn main() {
    println!("count ip address! ");
    ips_between("170.1.0.10", "20.0.1.0");

}

fn ips_between(start: &str, end: &str) -> u32 {
    let end_list: Vec<u64> = end.split('.').map(|addr| addr.parse().unwrap()).collect();
    let start_list: Vec<u64> = start.split('.').map(|addr| addr.parse().unwrap()).collect();
    let mut can_start_count: bool = false;
    
    let mut ip_end_count: u64 = 0;
    let mut ip_start_count: u64 = 0;

    for index in 0..end_list.len() {
        if end_list[index] > start_list[index] && !can_start_count {
            can_start_count = true
        }
        if can_start_count {
            ip_end_count += end_list[index] * u64::pow(256, 3 - index as u32);
            ip_start_count += start_list[index] * u64::pow(256, 3 - index as u32);
        }
    }
    (ip_end_count - ip_start_count) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(ips_between("10.0.0.0", "10.0.0.50"), 50);
        assert_eq!(ips_between("20.0.0.10", "20.0.1.0"), 246);
    }
}

