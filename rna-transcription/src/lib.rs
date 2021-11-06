use std::collections::HashMap;
#[derive(Debug, PartialEq)]
pub struct Dna{
    strand: String
}

#[derive(Debug, PartialEq)]
pub struct Rna{
    strand: String
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let invalid_index: i32;
        invalid_index = validate_dna_nucleotide(dna);
        if invalid_index >= 0 {
            return Err(invalid_index as usize)
        }
        Ok(Dna{strand: String::from(dna)})
    }

    pub fn into_rna(self) -> Rna {
        let mut rna_strand: String = String::from("");
        let mut dna_rna_map: HashMap<char, char> = HashMap::new();
        dna_rna_map.insert('G', 'C');
        dna_rna_map.insert('C', 'G');
        dna_rna_map.insert('T', 'A');
        dna_rna_map.insert('A', 'U');

        for (_i, val) in self.strand.chars().enumerate() {
            rna_strand.push(*dna_rna_map.get(&val).unwrap())
        }
        Rna{strand: rna_strand}
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let invalid_index: i32;
        invalid_index = validate_rna_nucleotide(rna);
        if invalid_index >= 0 {
            return Err(invalid_index as usize)
        }
        Ok(Rna{strand: String::from(rna)})
    }
}

fn validate_dna_nucleotide(strand: &str) -> i32 {
    for (i, val) in strand.chars().enumerate() {
        if !matches!(val, 'A' | 'T' | 'G'| 'C') {
            return i as i32
        }
    }
    -1
}

fn validate_rna_nucleotide(strand: &str) -> i32 {
    for (i, val) in strand.chars().enumerate() {
        if !matches!(val, 'A' | 'U' | 'G'| 'C') {
            return i as i32
        }
    }
    -1
}


