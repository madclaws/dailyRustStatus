use std::collections::HashMap;
fn main() {
    println!("Help the bookseller");
    stock_list(vec![],
    vec!["A", "B", "C", "W"]);
}

fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    let mut result =  "".to_owned();
    let mut bookkeeper: HashMap<&str, u32> = HashMap::new();
    
    if list_art.is_empty() {
        return result
    }
    for book in list_art {
        let book_split = book.split(' ').collect::<Vec<&str>>();
        let (first_str, _t) = book_split[0].split_at(1);
        let qty: u32 = book_split[1].parse().unwrap();
        bookkeeper.entry(first_str)
        .and_modify(|quantity| *quantity += qty)
        .or_insert(book_split[1].parse().unwrap());
    }
    
    for index in 0..list_cat.len(){
        if let Some(qty) = bookkeeper.get(list_cat[index]) {
            result = format!("{}{}{}{}{}{}", result, "(", list_cat[index], " : ", qty, ")")
        } else {
            result = format!("{}{}{}{}{}", result, "(", list_cat[index], " : 0", ")")
        }
        if index != list_cat.len() - 1 {
            result = format!("{}{}", result, " - ")
        }
    }

    println!("{}", result);
    result
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(list_art: Vec<&str>, list_cat: Vec<&str>, exp: &str) -> () {
        println!("list_art: {:?};", list_art);
        println!("list_cat: {:?};", list_cat);
        let ans = stock_list(list_art, list_cat);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        let mut b = vec!["BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"];
        let mut c = vec!["A", "B", "C", "D"];
        dotest(b, c, "(A : 0) - (B : 1290) - (C : 515) - (D : 600)");

        b = vec!["ABAR 200", "CDXE 500", "BKWR 250", "BTSQ 890", "DRTY 600"];
        c = vec!["A", "B"];
        dotest(b, c, "(A : 200) - (B : 1140)");

    }
}