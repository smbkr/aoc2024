use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Failed to read file");
    let (mut left, mut right) = parse_input(&contents);

    left.sort_unstable();
    right.sort_unstable();

    let total_distance: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    println!("Total distance: {}", total_distance);

    let appearances = count_appearances(&right);
    let similarity_score: i32 = left
        .iter()
        .map(|i| match appearances.get(i) {
            Some(value) => *value * i,
            None => 0,
        })
        .sum();

    println!("Similarity score: {}", similarity_score);
}

fn parse_input(input: &String) -> (Vec<i32>, Vec<i32>) {
    let lines = input.lines();

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in lines {
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        left.push(values[0]);
        right.push(values[1]);
    }

    (left, right)
}

fn count_appearances(input: &Vec<i32>) -> HashMap<i32, i32> {
    let mut appearances: HashMap<i32, i32> = HashMap::new();

    for i in input {
        match appearances.get_mut(i) {
            Some(value) => *value += 1,
            None => {
                appearances.insert(*i, 1);
            }
        }
    }

    appearances
}
