use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
pub struct BitCount {
    zero_count: i32,
    one_count: i32,
}

pub fn run() {
    println!("\nDay 3 => Binary Diagnostic");
    let raw_report = fs::read_to_string("/home/madclaws/labs/aoc_2021/inputs/day_3.txt").expect("Unable to load file");
    
    let mut column_wise_aggregate: HashMap<i32, BitCount> = HashMap::new();
    let report_lines: Vec<&str> = raw_report.split('\n').collect();
    
    for report_line in report_lines {
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