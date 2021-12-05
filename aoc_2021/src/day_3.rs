use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
#[derive(Debug)]
pub struct BitCount {
    zero_count: i32,
    one_count: i32,
}

impl BitCount {
    fn new() -> Self {
        BitCount{zero_count: 0, one_count: 0}
    }

    fn increase_zero_count(&mut self) {
        self.zero_count += 1
    }

    fn increase_one_count(&mut self) {
        self.one_count += 1
    }
}
pub fn run() {
    println!("\nDay 3 => Binary Diagnostic");
    let raw_report = fs::read_to_string("/home/madclaws/labs/aoc_2021/inputs/day_3.txt").expect("Unable to load file");
    
    let mut column_wise_aggregate: HashMap<i32, BitCount> = HashMap::new();
    let report_lines: Vec<&str> = raw_report.split('\n').collect();
    
    for report_line in &report_lines {
        for (index, binary) in report_line.chars().enumerate() {
            column_wise_aggregate.entry(index as i32)
            .and_modify(|bitcount| {
                if binary == '0' {
                    bitcount.zero_count += 1;
                } else {
                    bitcount.one_count += 1;
                }
            })
            .or_insert_with(|| create_bitcount(binary));
        }
    }
    let mut colum_agg_vec: Vec<_> = column_wise_aggregate.into_iter().collect();
    colum_agg_vec.sort_by(|(k_a, _val_a), (k_b, _val_b)| k_a.cmp(k_b));
    
    let gamma_rate = create_gamma_rate_in_binary(&colum_agg_vec);
    let epsilon_rate = create_epsilon_rate_in_binary(&colum_agg_vec);
    println!("Power consumption of submarine => {}", convert_binary_to_decimal(gamma_rate)
         * convert_binary_to_decimal(epsilon_rate));

    let oxygen_rate = calculate_oxygen_generator_rate(&report_lines, &colum_agg_vec, report_lines[0].len());
    println!("oxygen rate => {}\n\n", oxygen_rate);
    
    let carbon_scrubber_rate = calculate_carbon_scrubber_rate(&report_lines, &colum_agg_vec, report_lines[0].len());
    println!("carbon_scrubber_rate => {}", carbon_scrubber_rate);

    println!("Life support rating of submarine => {}", oxygen_rate * carbon_scrubber_rate);
}

fn create_gamma_rate_in_binary(column_aggregate: &[(i32, BitCount)]) -> String {
    let mut gamma_rate: String = String::from("");
    for (_k, v) in column_aggregate {
        if v.zero_count > v.one_count {
            gamma_rate.push('0')
        } else {
            gamma_rate.push('1')
        }
    }
    println!("{:?}", gamma_rate);
    gamma_rate
}

fn create_epsilon_rate_in_binary(column_aggregate: &[(i32, BitCount)]) -> String {
    let mut epsilon_rate: String = String::from("");
    for (_k, v) in column_aggregate {
        if v.zero_count > v.one_count {
            epsilon_rate.push('1')
        } else {
            epsilon_rate.push('0')
        }
    }
    println!("{:?}", epsilon_rate);
    epsilon_rate
}

fn create_bitcount(bit: char) -> BitCount {
    if bit == '0' {
        BitCount{zero_count: 1, one_count: 0}
    } else {
        BitCount{zero_count: 0, one_count: 1}
    }
}

fn convert_binary_to_decimal(binary: String) -> u32 {
    let mut decimal_number: u32 = 0;
    let mut power = binary.len() as u32;
    for elem in binary.chars() {
        let element: u32 = elem.to_digit(10).unwrap();
        power -= 1;
        decimal_number += element * u32::pow(2, power);
    }
    decimal_number
}

fn calculate_oxygen_generator_rate(report_lines: &[&str], column_agg: &[(i32, BitCount)], line_len: usize) -> u32{
    let mut filtered_set: HashSet<String> = HashSet::new();   
    
    for pos in 0..line_len {
        let bitcount = get_bitcount(&filtered_set, pos);
        for &report_line in report_lines{
            if pos > 0 && !filtered_set.contains(report_line) {
                continue;
            }
            let most_common_bit;
            if pos > 0 {
                most_common_bit = get_most_common_bit_by_struct(&bitcount);
            } else {
                most_common_bit = get_most_common_bit(&column_agg[pos as usize]); 
            }
            if most_common_bit == report_line.chars().nth(pos as usize).unwrap() {
                filtered_set.insert(report_line.to_owned());
            } else if pos > 0{
                filtered_set.remove(report_line);
            }
        }
        if filtered_set.len() == 1 {
            break;
        }
    }
    let oxygen_list: Vec<String> = filtered_set.into_iter().collect();
    println!("{:?}", oxygen_list[0]);
    convert_binary_to_decimal(oxygen_list[0].clone()) 
}

fn calculate_carbon_scrubber_rate(report_lines: &[&str], column_agg: &[(i32, BitCount)], line_len: usize) -> u32{
    let mut filtered_set: HashSet<String> = HashSet::new();   
    
    for pos in 0..line_len {
        let bitcount = get_bitcount(&filtered_set, pos);
        for &report_line in report_lines{
            if pos > 0 && !filtered_set.contains(report_line) {
                continue;
            }
            let least_common_bit;
            if pos > 0 {
                least_common_bit = get_least_common_bit_by_struct(&bitcount);
            } else {
                least_common_bit = get_least_common_bit(&column_agg[pos as usize]); 
            }
            
            if least_common_bit == report_line.chars().nth(pos as usize).unwrap() {
                filtered_set.insert(report_line.to_owned());
            } else if pos > 0{
                filtered_set.remove(report_line);
            }
        }
        if filtered_set.len() == 1 {
            break;
        }
    }
    let carbonlist: Vec<String> = filtered_set.into_iter().collect();
    println!("{:?}", carbonlist[0]);
    convert_binary_to_decimal(carbonlist[0].clone()) 
}

fn get_most_common_bit(column_agg: &(i32, BitCount)) -> char {
    let bit_count = &column_agg.1;
    if bit_count.zero_count > bit_count.one_count {
        '0'
    } else {
        '1'
    }
}

fn get_most_common_bit_by_struct(bitcount: &BitCount) -> char {
    if bitcount.zero_count > bitcount.one_count {
        '0'
    } else {
        '1'
    }
}

fn get_least_common_bit(column_agg: &(i32, BitCount)) -> char {
    let bit_count = &column_agg.1;
    if bit_count.zero_count > bit_count.one_count {
        '1'
    } else {
        '0'
    }
}

fn get_least_common_bit_by_struct(bitcount: &BitCount) -> char {
    if bitcount.zero_count > bitcount.one_count {
        '1'
    } else {
        '0'
    }
}

fn get_bitcount(filtered_set: &HashSet<String>, pos: usize) -> BitCount{
    let mut bitcount: BitCount = BitCount::new();
    for key in filtered_set {
        if key.chars().nth(pos).unwrap() == '0' {
            bitcount.increase_zero_count();
        } else {
            bitcount.increase_one_count();
        }
    }
    bitcount
}