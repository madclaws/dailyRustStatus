pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = "".to_owned();
    for (i, x) in list.iter().enumerate(){
        if i + 1 == list.len() {
            let end_line = format!("And all for the want of a {}.", list[0]); 
            proverb.push_str(&end_line);
            return proverb;
        } else {
            let line = format!("For want of a {} the {} was lost.\n", x, list[i+1]);
            proverb.push_str(&line);
        }
    }
    return proverb
}
