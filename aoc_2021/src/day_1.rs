use std::fs;

pub fn run() {
    println!("\nDay 1 => Sonar Sweep");
    
    let input: String = fs::read_to_string("/home/madclaws/labs/aoc_2021/inputs/day_1.txt").expect("Unable to load the file");
    let input_list: Vec<i32> = input.split('\n').map(|x| x.parse::<i32>().unwrap()).collect();
    println!("Measurements that are larger than the previous measurement are => {}", calculate_total_larger_measurements(&input_list));

    let threesome_list = calculate_threesome_list(&input_list);

    println!("Sums larger than previous threesome => {}", calculate_total_larger_measurements(&threesome_list));
    
}


fn calculate_total_larger_measurements(input_list: &[i32]) -> u32{
    let mut larger_measurement_count: u32 = 0;
    for index in 0..input_list.len() {
        if index > 0 && input_list[index] > input_list[index - 1]{
            larger_measurement_count += 1;
        }
    }
    larger_measurement_count
}

fn calculate_threesome_list(input_list: &[i32]) -> Vec<i32>{
    let mut threesome_list: Vec<i32> = Vec::new();
    let input_length = input_list.len();
    for index in 0..input_list.len() {
        if index + 1 < input_length && index + 2 < input_length {
            threesome_list.push(input_list[index] + input_list[index + 1] + input_list[index + 2]);
        }
    }
    threesome_list
}