use itertools::Itertools;

use super::utils::str_get_numbers_regex;

fn part_one(input: &String) -> usize {
    let mut lines = input.lines();

    let times = str_get_numbers_regex(lines.next().unwrap());

    let records = str_get_numbers_regex(lines.next().unwrap());

    let mut res = 1usize;

    for (&time, &record) in times.iter().zip(&records) {
        let mut temp = 0usize;
        for i in 0..time {
            let speed = i * (time - i);
            if speed > record {
                temp += 1;
            }
        }
        res *= temp;
    }

    res
}

fn part_two(input: &String) -> usize {
    let mut lines = input.lines();

    let time: usize = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .join("")
        .parse::<usize>()
        .unwrap();

    let record = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .join("")
        .parse::<usize>()
        .unwrap();

    let mut res = 0usize;
    for i in 0..time {
        let speed = i * (time - i);
        if speed > record {
            res += 1;
        }
    }

    res
}

pub fn solution(input: String) -> (usize, usize) {
    (part_one(&input), part_two(&input))
}
