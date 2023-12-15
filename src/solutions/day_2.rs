#[derive(Debug)]
struct Bag {
    red: usize,
    green: usize,
    blue: usize,
}

const BASE_BAG: Bag = Bag {
    red: 12,
    green: 13,
    blue: 14,
};

#[derive(Debug)]
struct BagSet {
    idx: usize,
    bags: Vec<Bag>,
}

fn possible_bag_sets(input: &String) -> Vec<BagSet> {
    let mut idx = 1usize;
    input
        .lines()
        .map(|line| {
            let parts = line.split(": ").nth(1).unwrap();

            let play = parts.split("; ");

            let bags = play
                .map(|p| {
                    let mut game = Bag {
                        red: 0,
                        green: 0,
                        blue: 0,
                    };

                    let cubes = p.trim().split(", ").for_each(|c| {
                        let mut cube = c.trim().split(" ");

                        let number = cube
                            .next()
                            .unwrap()
                            .parse::<usize>()
                            .expect("Should be a valid usize");
                        let name = cube.next().unwrap();

                        match name {
                            "red" => game.red = number,
                            "green" => game.green = number,
                            "blue" => game.blue = number,
                            _ => {
                                panic!("Allowed cube colors are red | green | blue")
                            }
                        }
                    });

                    game
                })
                .collect();

            let bag_set = BagSet { idx, bags };

            idx += 1;
            return bag_set;
        })
        .collect()
}

fn part_two(input: String) -> usize {
    let bags = possible_bag_sets(&input);

    bags.iter()
        .map(|set| {
            let mut red = 0usize;
            let mut green = 0usize;
            let mut blue = 0usize;

            set.bags.iter().for_each(|bag| {
                red = red.max(bag.red);
                green = green.max(bag.green);
                blue = blue.max(bag.blue);
            });

            red * green * blue
        })
        .sum()
}

fn part_one(input: String) -> usize {
    let bags = possible_bag_sets(&input);

    bags.into_iter()
        .filter(|b| {
            b.bags.iter().all(|b| {
                b.red <= BASE_BAG.red && b.green <= BASE_BAG.green && b.blue <= BASE_BAG.blue
            })
        })
        .map(|b| b.idx)
        .sum()
}

pub fn solution(input: String) -> (usize, usize) {
    let ans_one = part_one(input.clone());
    let ans_two = part_two(input);

    (ans_one, ans_two)
}
