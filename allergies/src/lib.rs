use std::collections::HashMap;

#[derive(Debug)]
pub struct Allergies{
    score: u32,
    allergy_map: HashMap<u32, Allergen>
}

#[derive(Debug, PartialEq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let allergy_tuples = vec![(0, Allergen::Eggs), (1, Allergen::Peanuts),
        (2, Allergen::Shellfish), (3, Allergen::Strawberries), (4, Allergen::Tomatoes),
        (5, Allergen::Chocolate), (6, Allergen::Pollen), (7, Allergen::Cats)];

        Allergies{score: score, allergy_map: allergy_tuples.into_iter().collect()}
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergy_list = self.allergies();
        allergy_list.into_iter().any(|x| x == allergen)
    }

    pub fn allergies(&self) -> Vec<&Allergen> {
        let mut allergy_list: Vec<&Allergen> = Vec::new();
        let mut allergy_score = self.score;
        while allergy_score != 0 {
            let log = f64::log2(allergy_score as f64).floor() as u32;
            match self.allergy_map.get(&log) {
                Some(allergy) => {
                    allergy_score -= u32::pow(2, log) ;
                    allergy_list.push(allergy.to_owned())
                },
                None => allergy_score -= u32::pow(2, log)
            }
        }
        allergy_list
    }
}
