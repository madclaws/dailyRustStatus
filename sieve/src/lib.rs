pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut prime_list: Vec<u64> = Vec::new();
    let mut initial_list: Vec<bool> = (0..=upper_bound).map(|_prime_status| true).collect();
    
    println!("{:?}", initial_list);
    
    for p in 2..=upper_bound {
        if initial_list[p as usize] {
            let mut j = p * p;
            while j < upper_bound {
                initial_list[j as usize] = false;
                j += p;
            }
        }    
    }

    for p in 2..upper_bound {
        if initial_list[p as usize] {
            prime_list.push(p)
        }
    }

    println!("Primes are {:?}", prime_list);
    prime_list

}
