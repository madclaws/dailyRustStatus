pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples_list: Vec<u32> = vec![0;0];
    for n in 1..limit {
        for (_i, x) in factors.iter().enumerate() {
            if x < &1 {
                break;
            }
            if n % x == 0 {
                multiples_list.push(n);
                break;
            }
        }
    }
    multiples_list.iter().sum()
}
