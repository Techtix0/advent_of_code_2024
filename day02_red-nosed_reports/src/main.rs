use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = read_file(&args);

    println!("{}", total_safe(contents));
}

fn read_file(args: &[String]) -> String {
    let path = args[1].clone();
    fs::read_to_string(path).expect("Error reading file")
}

fn total_safe(contents: String) -> u32 {
    let mut iterator = contents.split("\n");
    let mut line = iterator.next().unwrap();

    let mut amount: u32 = 0;
    while !line.is_empty() {
        let mut levels: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();

        if is_safe(&mut levels) {
            amount += 1;
        } else {
            for i in 0..levels.len() {
                let mut unsafe_levels = levels.clone();
                unsafe_levels.remove(i);
                if is_safe(&mut unsafe_levels) {
                    amount += 1;
                    break;
                }
            }
        }
        line = iterator.next().unwrap();
    }

    amount
}

fn is_safe(levels: &Vec<i32>) -> bool {
    let mut lag: i32 = levels[0];
    let mut lead: i32 = levels[1];
    let mut difference: i32 = lead - lag;

    if !is_within_bounds(&difference) {
        return false;
    }

    let mut is_increasing: bool = difference.is_positive();
    let mut was_increasing: bool;

    for i in 2..levels.len() {
        lag = levels[i - 1];
        lead = levels[i];
        difference = lead - lag;

        was_increasing = is_increasing;
        is_increasing = difference.is_positive();

        if was_increasing != is_increasing || !is_within_bounds(&difference) {
            return false;
        }
    }
    true
}

fn is_within_bounds(num: &i32) -> bool {
    if num.abs() > 0 && num.abs() < 4 {
        true
    } else {
        false
    }
}
