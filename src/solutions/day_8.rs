use std::collections::HashMap;

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
struct Name([u8; 3]);

impl Name {
    fn parse(i: &str) -> Self {
        Self(i[..3].as_bytes().try_into().unwrap())
    }

    fn ends_with(&self, i: u8) -> bool {
        self.0.ends_with(&[i])
    }
}
#[derive(Debug)]
struct MapRunner {
    directions: Vec<char>,
    pub nodes: HashMap<Name, (Name, Name)>,
}

type Steps = usize;
impl MapRunner {
    fn run_to(&self, start: Name, to: Name) -> Steps {
        let mut current = start;
        let dir_len = self.directions.len();

        let mut steps: Steps = 0usize;
        while current != to {
            let direction = self.directions[steps % dir_len];

            let node = self.nodes.get(&current).unwrap();

            current = match direction {
                'L' => node.0,
                'R' => node.1,
                _ => unimplemented!(),
            };
            steps += 1;
        }
        steps
    }

    /// run from every node ending with `start` char
    /// run to every node ending with `to` char
    fn run_to_by_ending(&self, start: u8, to: u8) -> Steps {
        let mut current = self
            .nodes
            .iter()
            .filter(|(k, _)| k.ends_with(start))
            .map(|(k, __)| *k)
            .collect::<Vec<Name>>();

        let dir_len = self.directions.len();

        let mut steps_to_reach_end: Vec<Steps> = vec![];

        current.iter().for_each(|node| {
            let mut finished = false;
            let mut next = node.clone();
            let mut steps = 0usize;

            while !finished {
                let direction = self.directions[steps % dir_len];

                let found_node = self.nodes.get(&next).unwrap();

                next = match direction {
                    'L' => found_node.0,
                    'R' => found_node.1,
                    _ => unimplemented!(),
                };

                finished = next.ends_with(b'Z');
                steps += 1;
            }

            steps_to_reach_end.push(steps);
        });

        steps_to_reach_end.iter().fold(1, |a, b| lcm(a, *b))
    }
}

pub fn solution(input: String) -> (usize, usize) {
    let mut inp = input.split("\n\n");

    let directions: Vec<char> = inp.next().unwrap().chars().collect();

    let mut map_runner = MapRunner {
        directions,
        nodes: HashMap::new(),
    };

    inp.next().unwrap().split("\n").for_each(|node| {
        let (key, paths) = node.split_once(" = ").unwrap();
        let p = paths
            .replace("(", "")
            .replace(")", "")
            .split_once(", ")
            .and_then(|n| Some((Name::parse(n.0), Name::parse(n.1))))
            .unwrap();

        map_runner.nodes.insert(Name::parse(key), p);
    });

    (
        map_runner.run_to(Name::parse("AAA"), Name::parse("ZZZ")),
        map_runner.run_to_by_ending(b'A', b'Z'),
    )
}
