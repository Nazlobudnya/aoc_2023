use crate::solutions::utils::str_get_numbers_delim;

fn part_one(input: &String) -> usize {
    input
        .lines()
        .map(|line| {
            let i = str_get_numbers_delim::<isize>(line, " ");
            let num = i.last().unwrap().clone() as isize;
            let mut next_sequences: Vec<isize> = vec![num];

            let mut next: Vec<isize> = i;
            loop {
                let seq: Vec<isize> = next.windows(2).map(|a| a[1] - a[0]).collect();

                if seq.iter().all(|&x| x == 0) {
                    break;
                } else {
                    next_sequences.push(seq.last().unwrap().clone());
                    next = seq;
                }
            }

            next_sequences
        })
        .map(|s| s.iter().sum::<isize>())
        .sum::<isize>() as usize
}

fn part_two(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let i = str_get_numbers_delim::<isize>(line, " ");
            let num = *i.first().unwrap() as isize;
            let mut next_sequences: Vec<isize> = vec![num];

            let mut next: Vec<isize> = i;
            loop {
                let seq: Vec<isize> = next.windows(2).map(|a| a[1] - a[0]).collect();
                if seq.iter().all(|&x| x == 0) {
                    break;
                } else {
                    let f = *seq.first().unwrap();
                    next_sequences.push(f);
                    next = seq;
                }
            }

            next_sequences.reverse();
            next_sequences
        })
        .map(|s| s.iter().skip(1).fold(s[0], |acc, curr| curr - acc))
        .sum::<isize>() as usize
}

pub fn solution(input: String) -> (usize, usize) {
    (part_one(&input), part_two(&input))
}
