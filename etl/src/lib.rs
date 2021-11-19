use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut transform_map: BTreeMap<char, i32> = BTreeMap::new();
    for (k, vector) in h {
        for &data in vector {
            transform_map.insert(data.to_lowercase().next().unwrap(), *k);
        }
    }
    transform_map
}
