use std::fs::File;
use std::collections::HashMap;

fn main() {
    println!("Molecular Mass Calculator");
    calculate_molecular_mass("C20H25N3O");
}

fn calculate_molecular_mass(formula: &str) -> f32 {
    let mut periodic_map: HashMap<&str, f32> = HashMap::new();

    let file = File::open("periodic_table.csv").expect("CSV Doesn't exist");
    let mut reader = csv::ReaderBuilder::new()
                     .has_headers(false)
                     .from_reader(file);

    let mut records: Vec<csv::StringRecord> = Vec::new();
    for result in reader.records() {
        let record = result.expect("An element record");
        records.push(record);
    }

    for index in 0..records.len() {
        let element = &records[index][0];
        let amu = &records[index][1].parse::<f32>().unwrap();
        periodic_map.insert(element, *amu);
    }
    calculate(formula, &periodic_map)
}

fn calculate(formula: &str, periodic_map: &HashMap<&str, f32>) -> f32 {
    let mut molecular_mass = 0.0;
    let mut atom = String::from("");
    let mut num_atoms: String = String::from("");
    for ch in formula.chars() {
        if ch.is_uppercase() {
            if !atom.is_empty() {
                let amu = periodic_map.get(&atom[..]).unwrap();
                molecular_mass += amu * get_total_atoms(&num_atoms) as f32;
                num_atoms.clear();
                atom.clear();
            }
            atom.push(ch);
        } else if ch.is_numeric() {
            num_atoms.push(ch);
        } else {
            atom.push(ch)
        }
    }
    if !atom.is_empty() {
        let amu = periodic_map.get(&atom[..]).unwrap();
        molecular_mass += amu * get_total_atoms(&num_atoms) as f32;
    }
    println!("Molecular mass of {} is {:?}", formula, molecular_mass);
    molecular_mass
}

fn get_total_atoms(num_atoms: &str) -> f32 {
    if num_atoms.is_empty() {
        1.0
    } else {
        num_atoms.parse::<f32>().unwrap()
    }
}


#[cfg(test)]
#[test]
fn test_molecular_mass_calculation() {
    assert_eq!(f32::trunc(calculate_molecular_mass("Pa") * 100.0) / 100.0, 231.03);
    assert_eq!(f32::trunc(calculate_molecular_mass("OCS") * 100.0) / 100.0, 60.07);
    assert_eq!(f32::trunc(calculate_molecular_mass("C4H4AsH") * 100.0) / 100.0, 128.00);
    assert_eq!(f32::trunc(calculate_molecular_mass("C20H25N3O") * 100.0) / 100.0, 323.44);

}