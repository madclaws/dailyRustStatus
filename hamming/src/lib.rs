/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        None
    } else {
        let mut s1_iter = s1.chars();
        let mut s2_iter = s2.chars();
        let mut hamming_dist: usize = 0;
        for _i in 0..s1.len() {
            if s1_iter.next().unwrap() != s2_iter.next().unwrap() {
                hamming_dist += 1;
            }
        }
        Some(hamming_dist)
    }

}
