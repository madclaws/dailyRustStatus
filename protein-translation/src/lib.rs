use std::collections::HashMap;
pub struct CodonsInfo<'a> {
    pub codonpairs: HashMap<&'a str, &'a str>
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        match self.codonpairs.get(codon) {
            Some(&val) => Some(val),
            _ => None
        }
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut list_of_protein: Vec<&'a str> = Vec::new();
        for chunk_list in rna.chars().collect::<Vec<char>>().chunks(3) {
            let codon = chunk_list.iter().collect::<String>();
            match self.name_for(&codon) {
                Some("stop codon") => break,
                Some(protein) => {
                    list_of_protein.push(protein);
                },
                _ => return None
            }
        }
        Some(list_of_protein)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let mut codon_pairs: HashMap<&'a str, &'a str> = HashMap::new();
    for (_i, &(codon, protein)) in pairs.iter().enumerate() {
        codon_pairs.insert(codon, protein);
    }
    CodonsInfo{codonpairs: codon_pairs}
}
