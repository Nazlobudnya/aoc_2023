use std::{ops::Range, time::Instant};

fn source_overlap(seeds: &Range<usize>, source: &Range<usize>) -> Option<Vec<Range<usize>>> {
    if seeds.start >= source.end || seeds.end <= source.start {
        return None;
    }

    if seeds.start == source.start && seeds.end == source.end {
        return Some(vec![seeds.start..seeds.end]);
    }

    if seeds.start == source.start {
        if seeds.end > source.end {
            return Some(vec![seeds.start..source.end]);
        } else {
            return Some(vec![seeds.start..seeds.end]);
        }
    }

    if seeds.end == source.end {
        if seeds.start > source.start {
            return Some(vec![seeds.start..seeds.end]);
        } else {
            return Some(vec![source.start..seeds.end]);
        }
    }

    if seeds.start < source.start {
        return Some(vec![source.start..seeds.end, seeds.start..source.start]);
    }

    if seeds.start > source.start && seeds.end < source.end {
        return Some(vec![seeds.clone()]);
    }

    if seeds.start > source.start && seeds.end > source.end {
        return Some(vec![seeds.start..source.end, source.end..seeds.end]);
    }

    None
}

fn part_one(input: &String) -> usize {
    let mut s = input.split("\n\n");
    let seeds = s.next().unwrap();
    let mut seeds = seeds
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    for i in s {
        let mapping: Vec<(Range<usize>, Range<usize>)> = i
            .split("\n")
            .skip(1)
            .map(|loc| {
                loc.split(" ")
                    .map(|l| l.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .map(|v| {
                let (target, source, range_length) = (v[0], v[1], v[2]);
                let source = source..source + range_length;
                let target = target..target + range_length;
                (source, target)
            })
            .collect();

        for s in seeds.iter_mut() {
            for (source, target) in &mapping {
                if source.contains(s) {
                    *s = (*s - source.start) + target.start;
                    break;
                }
            }
        }
    }

    seeds.into_iter().min().unwrap()
}

fn part_two(input: &String) -> usize {
    let mut s = input.split("\n\n");
    let mut seeds: Vec<Vec<Range<usize>>> = s
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .chunks(2)
        .map(|x| vec![x[0]..x[0] + x[1]])
        .collect();

    for i in s {
        let mapping: Vec<(Range<usize>, Range<usize>)> = i
            .split("\n")
            .skip(1)
            .map(|loc| {
                loc.split(" ")
                    .map(|l| l.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .map(|v| {
                let (target, source, range_length) = (v[0], v[1], v[2]);
                let source = source..source + range_length;
                let target = target..target + range_length;
                (source, target)
            })
            .collect();

        for s in seeds.iter_mut() {
            let mut done: Vec<Range<usize>> = vec![];

            for (source, target) in &mapping {
                let mut temp: Vec<Range<usize>> = vec![];
                for i in s.iter_mut() {
                    let overlap = source_overlap(i, source);

                    if let Some(mut r) = overlap {
                        r[0] = (target.start + (r[0].start - source.start))
                            ..(target.end - (source.end - r[0].end));
                        done.push(r[0].clone());

                        temp.extend(r.into_iter().skip(1).collect::<Vec<Range<usize>>>());
                    } else {
                        temp.push(i.clone());
                    }
                }

                *s = temp;
            }

            s.extend(done);
        }
    }
    let s = seeds
        .into_iter()
        .flatten()
        .min_by(|x, y| x.start.cmp(&y.start))
        .map(|x| x.start)
        .unwrap();

    42
}

pub fn solution(input: String) -> (usize, usize) {
    let ans_one = part_one(&input);

    let ans_two = part_two(&input);

    (ans_one, ans_two)
}
