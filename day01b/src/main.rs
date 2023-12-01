use std::collections::HashMap;

use regex::Regex;

pub fn main() {
    let number_values: HashMap<&str, usize> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    println!(
        "{}",
        include_str!("../input.txt")
            .split("\n")
            .map(|line| first_last_number(line, &number_values))
            .sum::<usize>()
    );
}

const NUMBERS_REGEX: &str = r"(1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine)";

fn first_last_number(line: &str, number_values: &HashMap<&str, usize>) -> usize {
    let matches: Vec<_> = Regex::new(NUMBERS_REGEX)
        .unwrap()
        .find_iter(line)
        .map(|m| m.as_str())
        .collect();

    let first: usize = match matches.first() {
        Some(val) => match val.parse::<usize>() {
            Ok(parsed) => parsed,
            Err(_e) => *number_values.get(val).unwrap(),
        },
        None => 0,
    };
    let second: usize = match matches.last() {
        Some(val) => match val.parse::<usize>() {
            Ok(parsed) => parsed,
            Err(_e) => *number_values.get(val).unwrap(),
        },
        None => 0,
    };

    first * 10 + second
}
