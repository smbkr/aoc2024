use regex::Regex;
use std::env::args;
use std::fs;

fn main() {
    let file_path = args().skip(1).next().unwrap();
    let contents = fs::read_to_string(&file_path).unwrap();

    let result = simple(&contents);
    let result2 = with_on_off_operators(&contents);

    println!("simple: {}", result);
    println!("with operators: {}", result2)
}

fn simple(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut operations: Vec<(i64, i64)> = vec![];
    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        operations.push((a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()));
    }

    operations.iter().map(|(a, b)| a * b).sum::<i64>()
}

fn with_on_off_operators(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d+,\d+)\)|(do\(\))|(don't\(\))").unwrap();

    let mut operations: Vec<(i64, i64)> = vec![];
    let mut enabled = true;

    for (_, [m]) in re.captures_iter(input).map(|c| c.extract()) {
        match m {
            m if m == "do()" => enabled = true,
            m if m == "don't()" => enabled = false,
            m => {
                if enabled {
                    let parsed = m
                        .split(',')
                        .map(|v| v.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();

                    operations.push((parsed[0], parsed[1]));
                }
            }
        }
    }

    operations.iter().map(|(a, b)| a * b).sum::<i64>()
}
