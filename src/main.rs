//use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");
    let day1_filepath: &str = "data/day1_input.txt";
    let contents = fs::read_to_string(day1_filepath).expect("Error");

    let mut sum: u32 = 0;
    let mut number: u32 = 0;

    //iterate and sum over
    for line in contents.lines() {
        let mut start_num:char = '0';
        let mut end_num:char = '0';
        let reversed: String = line.chars().rev().collect();
        for c in line.chars() {
            if c.is_digit(10) {
                start_num = c;
                break;
            }
        }
        for c in reversed.chars() {
            if c.is_digit(10) {
                end_num = c;
                break;
            }
        }
        number = format!("{}{}", start_num, end_num).parse().unwrap();
        sum += number;
    }
    println!("Sum is {sum}");
}

