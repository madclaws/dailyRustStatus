use std::collections::HashMap;
use std::cmp::Ordering;
fn main() {
    println!("Prize draw!");
    rank("COLIN,AMANDBA,AMANDAB,CAROL,PauL,JOSEPH", vec![1, 4, 4, 5, 2, 1],
    4);
}

fn rank(st: &str, we: Vec<i32>, n: usize) -> &str {
    let mut rank_map: HashMap<&str, i32> = HashMap::new();
    let name_list: Vec<&str> = st.split(',').collect();
    
    if we.len() < name_list.len() 
        || st.is_empty()
    {
        return "No participants"
    }

    for index in 0..we.len() {
        let som = calculate_som(name_list[index]);
        rank_map.insert(name_list[index], som * we[index]);
    }
    
    let mut rank_list: Vec<_> = rank_map.into_iter().collect();

    rank_list.sort_by(|(k1, v1), (k2, v2)| {
        match v2.cmp(v1) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            _ => k1.cmp(k2)
        }
    });

    if n - 1 >= rank_list.len() {
        return "Not enough participants"
    }
    
    let (firstname, _winner_number) = rank_list[n - 1];
    firstname
}

fn calculate_som(firstname: &str) -> i32 {
    let rank_sum = firstname.to_lowercase().chars().fold(0, |acc, c| {
        acc + (c as i32) - 97 + 1 
    });
    firstname.len() as i32 + rank_sum
}


fn testing(st: &str, we: Vec<i32>, n: usize, exp: &str) -> () {
    assert_eq!(rank(st, we, n), exp)
}

#[test]
fn basics_rank() {

    testing("Addison,Jayden,Sofia,Michael,Andrew,Lily,Benjamin", vec![4, 2, 1, 4, 3, 1, 2], 4, "Benjamin");
    testing("Elijah,Chloe,Elizabeth,Matthew,Natalie,Jayden", vec![1, 3, 5, 5, 3, 6], 2, "Matthew");
    testing("Aubrey,Olivai,Abigail,Chloe,Andrew,Elizabeth", vec![3, 1, 4, 4, 3, 2], 4, "Abigail");
    testing("Lagon,Lily", vec![1, 5], 2, "Lagon");
    
  }
