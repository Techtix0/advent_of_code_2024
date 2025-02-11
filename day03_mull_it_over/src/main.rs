use regex::Regex;
use std::fs;

fn main() {
    let contents = get_input();
    let mut lines = contents.lines();
    let mut line = lines.next();
    let mut mults: Vec<&str> = vec![];

    while line != None {
        let matches = find_valid_mul(&line.unwrap());
        for m in matches {
            mults.push(m);
        }
        line = lines.next();
    }

    println!("{}", calculate(mults));
}

fn get_input() -> String {
    return fs::read_to_string("input.txt").expect("Error reading file");
}

fn find_valid_mul(line: &str) -> Vec<&str> {
    let pattern = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let matches: Vec<&str> = pattern.find_iter(&line).map(|m| m.as_str()).collect();
    matches
}

fn calculate(mults: Vec<&str>) -> i32 {
    let mut calcs: Vec<i32> = vec![];

    for mult in mults {
        calcs.push(
            mult.replace(r"mul(", "")
                .replace(r")", "")
                .split(",")
                .map(|num| num.parse::<i32>().unwrap())
                .fold(1, |acc, x| acc * x)
        );
    }
    calcs.iter().sum()
}
