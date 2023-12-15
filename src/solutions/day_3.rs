use std::collections::HashSet;

use itertools::Itertools;

const SYMBOL_PROXIMITY_COORDS: [(isize, isize); 8] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];
#[derive(Clone, Debug)]
struct EngineNumber {
    str_number: String,
    number: usize,
    indexes: Vec<(usize, usize)>,
}

impl EngineNumber {
    fn reset(&mut self) {
        self.str_number.clear();
        self.number = 0;
        self.indexes.clear();
    }
}

fn parse_input(input: &String) -> (usize, usize, Vec<Vec<char>>) {
    let inp: Vec<&str> = input.split("\n").collect();

    let cols = inp[0].len();
    let rows = inp.len();

    let mut v: Vec<Vec<char>> = {
        let mut s = Vec::with_capacity(rows);
        for i in 0..rows {
            s.push(Vec::with_capacity(cols));
        }
        s
    };

    for y in 0..rows {
        let mut cs = inp[y].chars();
        for _x in 0..cols {
            v[y].push(cs.next().unwrap());
        }
    }

    (cols, rows, v)
}

fn part_two(input: String) -> usize {
    let (cols, rows, char_map) = parse_input(&input);

    let mut numbers: Vec<EngineNumber> = vec![];
    let mut current_number = EngineNumber {
        str_number: "".to_string(),
        number: 0,
        indexes: vec![],
    };

    let mut proximities: HashSet<Vec<(usize, usize)>> = HashSet::new();

    for y in 0..rows {
        for x in 0..cols {
            if char_map[y][x].is_numeric() {
                current_number.str_number.push(char_map[y][x]);
                current_number.indexes.push((y, x));
            } else {
                if current_number.str_number.len() != 0 {
                    current_number.number = current_number
                        .str_number
                        .parse::<usize>()
                        .expect("Must be a valid number");
                    numbers.push(current_number.clone());
                    current_number.reset();
                }

                if char_map[y][x] == '*' {
                    let mut v: Vec<(usize, usize)> = vec![];
                    for coords in SYMBOL_PROXIMITY_COORDS {
                        let new_x = (y as isize).checked_add(coords.0);
                        let new_y = (x as isize).checked_add(coords.1);

                        if new_x.is_some() && new_y.is_some() {
                            v.push((new_x.unwrap() as usize, new_y.unwrap() as usize));
                        }
                    }

                    proximities.insert(v);
                }
            }
        }
    }

    let mut res = 0usize;
    for coord_set in proximities {
        let found_numbers: HashSet<usize> = coord_set
            .iter()
            .map(|coord| {
                let num = numbers.iter().find(|&n| {
                    if n.indexes.iter().find(|&i| *i == *coord).is_some() {
                        true
                    } else {
                        false
                    }
                });
                num
            })
            .filter(|n| n.is_some())
            .map(|n| n.unwrap().number)
            .collect();

        if found_numbers.len() == 2 {
            res += found_numbers.iter().product::<usize>();
        }
    }

    res
}

fn part_one(input: String) -> usize {
    let (cols, rows, char_map) = parse_input(&input);

    let mut numbers: Vec<EngineNumber> = vec![];
    let mut current_number = EngineNumber {
        str_number: "".to_string(),
        number: 0,
        indexes: vec![],
    };

    let mut proximities: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..rows {
        for x in 0..cols {
            if char_map[y][x].is_numeric() {
                current_number.str_number.push(char_map[y][x]);
                current_number.indexes.push((y, x));
            } else {
                if current_number.str_number.len() != 0 {
                    current_number.number = current_number
                        .str_number
                        .parse::<usize>()
                        .expect("Must be a valid number");
                    numbers.push(current_number.clone());
                    current_number.str_number.clear();
                    current_number.number = 0;
                    current_number.indexes.clear();
                }

                if char_map[y][x] != '.' {
                    for coords in SYMBOL_PROXIMITY_COORDS {
                        let new_x = (y as isize).checked_add(coords.0);
                        let new_y = (x as isize).checked_add(coords.1);

                        if new_x.is_some() && new_y.is_some() {
                            proximities.insert((new_x.unwrap() as usize, new_y.unwrap() as usize));
                        }
                    }
                }
            }
        }
    }

    let mut res = 0usize;
    for coord in proximities {
        let num = numbers.iter().find_position(|&n| {
            if n.indexes.iter().find(|&i| *i == coord).is_some() {
                true
            } else {
                false
            }
        });

        if num.is_some() {
            let n = num.unwrap();
            res += n.1.number;
            numbers.remove(n.0);
        }
    }

    res
}

pub fn solution(input: String) -> (usize, usize) {
    let ans_one = part_one(input.clone());
    let ans_two = part_two(input);

    (ans_one, ans_two)
}
