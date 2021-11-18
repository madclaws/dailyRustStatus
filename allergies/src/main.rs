use allergies::*;

fn main() {
    let allergy = Allergies::new(509);
    // println!("{:?}", allergy);
    // println!("{}", allergy.is_allergic_to(&Allergen::Eggs));
    println!("{:?}", allergy.allergies());    
}