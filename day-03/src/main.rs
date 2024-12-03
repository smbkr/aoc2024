use regex::Regex;
use std::env::args;
use std::fs;

fn main() {
    let file_path = args().skip(1).next().unwrap();
    let contents = fs::read_to_string(&file_path).unwrap();

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut operations: Vec<(i64, i64)> = vec![];
    for (_, [a, b]) in re.captures_iter(&contents).map(|c| c.extract()) {
        operations.push((a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()));
    }

    let result = operations.iter().map(|(a, b)| a * b).sum::<i64>();

    println!("{}", result);
}
