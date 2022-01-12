use std::fs::File;
use std::collections::HashMap;

fn main() {
    println!("enso classification!");
    println!("{:?}", calculate_enso_classification(2016));
}

fn calculate_enso_classification(year: u32) -> (String, String) {
    let mut classification: String = String::from("");
    let mut strength: String = String::from("");
    let mut mei_map: HashMap<u32, f32> = HashMap::new();
    let file = File::open("mei_stats.txt").expect("FileCSV Doesn't exist");
    let mut reader = csv::ReaderBuilder::new()
                     .delimiter(b'\t')
                     .from_reader(file);   

    let mut records: Vec<csv::StringRecord> = Vec::new();

    for result in reader.records() {
        let record = result.expect("Record not found");
        records.push(record);
    }

    for item in &records {
        let year = item[0].parse::<u32>().unwrap();
        let max_mei: f32 = (1..=12).map(|i| item[i].parse::<f32>().unwrap())
        .collect::<Vec<f32>>().into_iter().fold(0.0, |mut max, mei| {
            if mei > max {
                max = mei
            }
            max
        });
        mei_map.insert(year, max_mei);
    }

    if let Some(mei) = mei_map.get(&year) {
        if mei >= &0.5 {
            classification.push_str("El Nino");
            if mei <= &1.0 {
                strength.push_str("Weak");
            } else if mei <= &1.5 {
                strength.push_str("Moderate");
            } else if mei <= &2.0 {
                strength.push_str("Strong");
            } else {
                strength.push_str("Very strong");
            }
        } else if mei <= &-0.5 {
            classification.push_str("La Nina");
            if mei >= &-0.9 {
                strength.push_str("Weak");
            } else if mei >= &-1.4 {
                strength.push_str("Moderate");
            } else if mei >= &-1.5 {
                strength.push_str("Strong");
            } else {
                strength.push_str("Very strong");
            }
        } else {
            classification.push_str("Neither");
            strength.push_str("None");    
        }
    } else {
        classification.push_str("Neither");
        strength.push_str("None");
    }

    (classification, strength)
}