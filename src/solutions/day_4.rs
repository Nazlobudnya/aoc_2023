use std::collections::HashMap;

fn part_one(input: &String) -> usize {
    input
        .lines()
        .map(|line| {
            let (_, numbers) = line.split_once(": ").unwrap();

            let (winning, placed) = numbers.split_once(" | ").unwrap();

            let mut match_number = 0usize;
            let mut res = 0usize;

            let winning: Vec<usize> = winning
                .split_ascii_whitespace()
                .map(|s| s.parse::<usize>().expect("Must be a valid number"))
                .collect();
            let placed: Vec<usize> = placed
                .split_ascii_whitespace()
                .map(|s| s.parse::<usize>().expect("Must be a valid number"))
                .collect();

            for p in placed {
                let found = winning.iter().find(|&w| *w == p);
                if found.is_some() {
                    res = (2usize).pow(match_number as u32);
                    match_number += 1;
                }
            }

            res
        })
        .sum()
}

fn part_two(input: &String) -> usize {
    let mut hm = HashMap::new();

    for idx in 0..input.lines().count() {
        hm.insert(idx + 1, 1usize);
    }

    input.lines().enumerate().for_each(|(idx, line)| {
        let index = idx + 1;
        let (_, numbers) = line.split_once(": ").unwrap();

        let (winning, placed) = numbers.split_once(" | ").unwrap();

        let winning: Vec<usize> = winning
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>().expect("Must be a valid number"))
            .collect();
        let placed: Vec<usize> = placed
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>().expect("Must be a valid number"))
            .collect();

        let mut winning_nums_count = 0usize;
        for p in placed {
            let found = winning.iter().find(|&w| *w == p);
            if found.is_some() {
                winning_nums_count += 1;
            }
        }

        let how_much = hm.get(&index).unwrap().clone();

        for i in index..(index + winning_nums_count) {
            hm.get_mut(&(i + 1)).and_then(|s| Some(*s += how_much));
        }
    });

    hm.values().sum::<usize>()
}

pub fn solution(input: String) -> (usize, usize) {
    let ans_one = part_one(&input);
    let ans_two = part_two(&input);

    (ans_one, ans_two)
}
