use regex::Regex;
use std::fs;

fn main() {
    let contents = get_input();
    let test_string = String::from(r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
    let matches = find_valid_mul(&test_string);

    for m in matches {
        println!("{m}");
    }
}

fn get_input() -> String {
    return fs::read_to_string("input.txt").expect("Error reading file");
}

fn find_valid_mul(line: &String) -> Vec<&str> {
    let pattern = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let matches: Vec<&str> = pattern.find_iter(&line).map(|m| m.as_str()).collect();
    matches
}
