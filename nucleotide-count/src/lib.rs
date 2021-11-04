use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let n_count: usize;
    let invalid_letters: String;
    if dna.is_empty() {
        return Ok(0)
    }
    invalid_letters = dna.chars().filter(|x| !matches!(x, 'A' | 'T' | 'G'| 'C')).collect();
    if !invalid_letters.is_empty() {
        return Err(invalid_letters.chars().next().unwrap())
    }
    if !matches!(nucleotide, 'A' | 'T' |'G' | 'C') {
        return Err(nucleotide)
    }
    n_count = dna.chars().filter(|x| *x == nucleotide).count();
    Ok(n_count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
   let mut n_hash: HashMap<char, usize> = HashMap::new();
   let n_vec: Vec<char> = ['A', 'G', 'C', 'T'].to_vec(); 
   let invalid_letters: String;
   invalid_letters = dna.chars().filter(|x| !matches!(x, 'A' | 'T' | 'G'| 'C')).collect();
    if !invalid_letters.is_empty() {
        return Err(invalid_letters.chars().next().unwrap())
    }
   for (_i, val) in n_vec.iter().enumerate() {
        match count(*val, dna) {
            Ok(count_val) => {
                n_hash.insert(*val, count_val);
            },
            Err(x) => {
                return Err(x)
            }
        }
   }
   Ok(n_hash)

}
