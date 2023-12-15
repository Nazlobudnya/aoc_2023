use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}
const CARDS: [u8; 13] = [
    b'A', b'K', b'Q', b'J', b'T', b'9', b'8', b'7', b'6', b'5', b'4', b'3', b'2',
];

fn kind(hand: &Vec<u8>) -> Kind {
    let mut counts = [0u8; 13];
    for c in hand {
        counts[*c as usize] += 1;
    }
    let counts: Vec<u8> = CARDS
        .iter()
        .filter_map(|c| {
            if counts[*c as usize] > 0 {
                Some(counts[*c as usize])
            } else {
                None
            }
        })
        .collect();
    match counts.len() {
        5 => Kind::HighCard,
        4 => Kind::OnePair,
        2 => {
            if counts[0] == 4 || counts[1] == 4 {
                return Kind::Four;
            } else {
                return Kind::Full;
            }
        }
        1 => Kind::Five,
        _ => {
            if counts.into_iter().max().unwrap() == 3 {
                Kind::Three
            } else {
                Kind::TwoPair
            }
        }
    }
}

use itertools::Itertools;
#[derive(Debug)]
struct Play<'a> {
    hand: &'a str,
    hand_values: Vec<u8>,
    hand_count: [u8; 13],
    joker_count: u8,
    bid: usize,
}

impl<'a> PartialOrd for Play<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand.partial_cmp(&other.hand) == Some(Ordering::Equal) {
            return Some(Ordering::Equal);
        }

        let mut self_present_count: Vec<&u8> = self
            .hand_count
            .iter()
            .filter(|&&c| c > 0)
            .sorted_by(|a, b| b.cmp(a))
            .collect();

        if self_present_count.is_empty() {
            self_present_count.push(&0u8);
        }

        let mut other_present_count: Vec<&u8> = other
            .hand_count
            .iter()
            .filter(|&&c| c > 0)
            .sorted_by(|a, b| b.cmp(a))
            .collect();

        if other_present_count.is_empty() {
            other_present_count.push(&0u8);
        }
        let mut iter = other_present_count.into_iter();

        let mut jokers_used = false;
        for (idx, m) in self_present_count.into_iter().enumerate() {
            if let Some(o) = iter.next() {
                if m == o {
                    if !jokers_used {
                        match (m + self.joker_count).cmp(&(o + other.joker_count)) {
                            Ordering::Equal => {
                                jokers_used = true;
                            }
                            Ordering::Greater => return Some(Ordering::Greater),
                            Ordering::Less => return Some(Ordering::Less),
                        }
                    }

                    continue;
                }

                if m > o {
                    if !jokers_used {
                        match (m + self.joker_count).cmp(&(o + other.joker_count)) {
                            Ordering::Equal => {
                                jokers_used = true;
                                continue;
                            }
                            Ordering::Greater => return Some(Ordering::Greater),
                            Ordering::Less => return Some(Ordering::Less),
                        }
                    } else {
                        return Some(Ordering::Greater);
                    }
                }

                if !jokers_used {
                    match (m + self.joker_count).cmp(&(o + other.joker_count)) {
                        Ordering::Equal => {
                            jokers_used = true;
                            continue;
                        }
                        Ordering::Greater => return Some(Ordering::Greater),
                        Ordering::Less => return Some(Ordering::Less),
                    }
                } else {
                    return Some(Ordering::Less);
                }
            } else {
                break;
            }
        }

        // if l1 > l2 {
        //     return Some(Ordering::Less);
        // }

        // if l1 < l2 {
        //     return Some(Ordering::Greater);
        // }

        // if self.joker_count > other.joker_count {
        //     return Some(Ordering::Greater);
        // }

        // if self.joker_count < other.joker_count {
        //     return Some(Ordering::Less);
        // }

        for i in 0..self.hand_values.len() {
            let s = self.hand_values[i];
            let o = other.hand_values[i];
            if s == o {
                continue;
            }

            return Some(s.cmp(&o));
        }

        Some(Ordering::Equal)
    }
}

impl<'a> PartialEq for Play<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

fn parse_plays<'a>(
    input: &'a String,
    card_strength: &HashMap<char, u8>,
    count_joker: bool,
) -> Vec<Play<'a>> {
    input
        .lines()
        .map(|l| {
            let (mut hand, bid) = l.split_once(" ").unwrap();

            let mut card_count: [u8; 13] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            let mut joker_count = 0u8;
            Play {
                hand,
                hand_values: hand
                    .chars()
                    .map(|c| {
                        let strength = *card_strength.get(&c).unwrap();
                        if count_joker && c == 'J' {
                            joker_count += 1;
                        } else {
                            card_count[(strength - 1) as usize] += 1;
                        }
                        strength
                    })
                    .collect(),
                joker_count: joker_count,
                hand_count: card_count,
                bid: bid.parse::<usize>().unwrap(),
            }
        })
        .collect()
}

fn part_one(input: &String) -> usize {
    let card_strength: HashMap<char, u8> = HashMap::from([
        ('2', 1),
        ('3', 2),
        ('4', 3),
        ('5', 4),
        ('6', 5),
        ('7', 6),
        ('8', 7),
        ('9', 8),
        ('T', 9),
        ('J', 10),
        ('Q', 11),
        ('K', 12),
        ('A', 13),
    ]);

    let plays = parse_plays(input, &card_strength, false);

    let mut res = 0usize;
    for (idx, play) in plays
        .iter()
        .sorted_by(|a, b| a.partial_cmp(b).unwrap())
        .enumerate()
    {
        res += (idx + 1) * play.bid;
    }
    res
}

fn part_two(input: &String) -> usize {
    let card_strength: HashMap<char, u8> = HashMap::from([
        ('J', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('Q', 11),
        ('K', 12),
        ('A', 13),
    ]);

    let plays = parse_plays(input, &card_strength, true);

    let p1 = Play {
        hand: "AAAAJ",
        hand_values: vec![13, 13, 13, 13, 1],
        hand_count: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4],
        joker_count: 1,
        bid: 614,
    };
    let p2 = Play {
        hand: "JJJJJ",
        hand_values: vec![1, 1, 1, 1, 1],
        hand_count: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        joker_count: 5,
        bid: 10,
    };

    println!(
        "Comparing {} J = {} with {} J = {}, Result is {:?}",
        p1.hand,
        p1.joker_count,
        p2.hand,
        p2.joker_count,
        p1.partial_cmp(&p2)
    );

    // let mut last_15_plays = vec![];

    let mut res = 0usize;
    for (idx, play) in plays
        .iter()
        .sorted_by(|a, b| {
            let c = a.partial_cmp(b).unwrap();
            // println!("Comparing {} and {}. Result = {:?}", a.hand, b.hand, c);
            c
        })
        .enumerate()
    {
        // if idx >= plays.len() - 15 {
        //     last_15_plays.push(play);
        // }

        println!("{}", play.hand);
        res += (idx + 1) * play.bid;
    }

    // println!("Last 15 ");
    // for play in last_15_plays.iter().sorted_by(|a, b| {
    //     let c = a.partial_cmp(b).unwrap();
    //     // println!("Comparing {} and {}. Result = {:?}", a.hand, b.hand, c);
    //     c
    // }) {
    //     println!("{play:?}");
    // }
    res
}

pub fn solution(input: String) -> (usize, usize) {
    // part one 248105065
    /*  part two
        < 249760380
        !249511008
        < 249994709
        !249504209

        == 249515436
    */
    (part_one(&input), part_two(&input))
}
