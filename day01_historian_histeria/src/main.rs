use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = user_input(&args);
    let processed_contents = process_input(&contents);

    println!(
        "Sum of all distances: {}",
        sum_of_all_distances(processed_contents.clone())
    );

    let mut repeats: HashMap<u32, u32> = HashMap::new();
    for num in processed_contents.1 {
        repeats
            .entry(num)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let mut result = 0;

    let mut iterator = processed_contents.0.iter();
    let mut current = iterator.next();

    while current != None {
        result += current.unwrap() * repeats.get(&current.unwrap()).unwrap_or_else(|| &0);

        current = iterator.next();
    }

    println!("Final result: {result}");
}

/// reads file from path specified in first command line argument
fn user_input(args: &[String]) -> String {
    let path = args[1].clone();
    fs::read_to_string(&path).expect("Error reading file.")
}

/// Created two sorted vectors containing u32 of the left and right column from an input string.
/// # example of input string:
/// ```
/// 123   456
/// 789   101
/// 324   345
/// ```
/// The two columns must be seperated by 3 spaces.
fn process_input(input: &String) -> (Vec<u32>, Vec<u32>) {
    let mut vec0 = Vec::new();
    let mut vec1 = Vec::new();

    let mut i = 0;
    for line in input.split("\n") {
        for num in line.trim().split_whitespace() {
            let number = if !num.is_empty() {
                num.trim().parse::<u32>().expect("Error parsing into u32.")
            } else {
                break;
            };

            if i % 2 == 0 {
                vec0.push(number);
            } else {
                vec1.push(number);
            }

            i += 1;
        }
    }
    vec0.sort();
    vec1.sort();
    (vec0, vec1)
}

/// Calculates the sum of the absolute difference of each row of both vectors.
fn sum_of_all_distances(mut vecs: (Vec<u32>, Vec<u32>)) -> u32 {
    let mut result: u32 = 0;
    for _ in 0..vecs.1.len() {
        result += vecs.0.pop().unwrap().abs_diff(vecs.1.pop().unwrap());
    }

    result
}
