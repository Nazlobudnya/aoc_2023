use std::num::ParseIntError;

use fancy_regex::{Captures, Regex};

fn parse_str(input: &str) -> Result<usize, ParseIntError> {
    match input {
        "one" => Ok(1),
        "two" => Ok(2),
        "three" => Ok(3),
        "four" => Ok(4),
        "five" => Ok(5),
        "six" => Ok(6),
        "seven" => Ok(7),
        "eight" => Ok(8),
        "nine" => Ok(9),
        x => x.parse::<usize>(),
    }
}

fn part_one(input: String) -> usize {
    input
        .lines()
        .map(|l: &str| {
            let first = l.find(|s: char| s.is_numeric()).unwrap();
            let second = l.rfind(|s: char| s.is_numeric()).unwrap();

            let num_one = parse_str(&l.chars().nth(first).unwrap().to_string()).unwrap();
            let num_two = parse_str(&l.chars().nth(second).unwrap().to_string()).unwrap();
            num_one * 10 + num_two
        })
        .sum()
}

fn part_two(input: String) -> usize {
    let re = Regex::new(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine))").unwrap();

    input
        .lines()
        .map(|l| {
            let matches = re
                .captures_iter(l)
                .map(|c| c.unwrap())
                .collect::<Vec<Captures>>();

            let f = parse_str(&matches.first().unwrap()[1]).unwrap();
            let s = parse_str(&matches.last().unwrap()[1]).unwrap();

            f * 10 + s
        })
        .sum()
}

pub fn solution(input: String) -> (usize, usize) {
    let ans_one = part_one(input.clone());
    let ans_two = part_two(input);

    (ans_one, ans_two)
}
