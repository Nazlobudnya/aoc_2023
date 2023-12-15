use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
struct PipeType {
    pub symbol: u8,
    pub coord: (usize, usize),
    pub entries: [(usize, usize); 2],
    pub in_loop: bool,
}
#[derive(Debug)]
struct Path {
    x: Vec<usize>,
    y: Vec<usize>,
}

impl Path {
    fn append(&mut self, p: (usize, usize)) {
        self.x.push(p.0);
        self.y.push(p.1);
    }
}

#[derive(Debug)]
enum Elem {
    Ground,
    Start(PipeType),
    Pipe(PipeType),
}

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
impl Display for Elem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Elem::Ground => write!(f, "."),
            Elem::Pipe(b) => write!(f, "{}", b.symbol as char),
            Elem::Start(_) => write!(f, "S"),
        }
    }
}

impl PipeType {
    fn can_enter(&self, other: &Self) -> bool {
        other.entries.iter().any(|e| *e == self.coord)
            && self.entries.iter().any(|e| *e == other.coord)
    }

    fn can_enter_to(&self, to: &Self) -> bool {
        to.entries.iter().any(|e| *e == self.coord)
    }

    fn can_enter_from(&self, from: &Self) -> bool {
        self.entries.iter().any(|e| *e == from.coord)
    }
}

#[derive(Debug)]
struct Pipeline {
    pipes: Vec<Vec<Elem>>,
}

fn part_one(input: &str) -> usize {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut start: PipeType = PipeType {
        symbol: b'S',
        coord: (0, 0),
        entries: [(0, 0), (0, 0)],
        in_loop: false,
    };

    let sides: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut pipeline = Pipeline {
        pipes: Vec::with_capacity(height),
    };
    input.lines().enumerate().for_each(|(y, l)| {
        pipeline.pipes.push(Vec::with_capacity(width));
        l.bytes().enumerate().for_each(|(x, b)| {
            let elem = match b {
                b'.' => Elem::Ground,
                b'|' => Elem::Pipe(PipeType {
                    symbol: b,
                    coord: (y, x),
                    entries: [(y - 1, x), (y + 1, x)],
                    in_loop: false,
                }),
                b'-' => Elem::Pipe(PipeType {
                    symbol: b,
                    coord: (y, x),
                    entries: [(y, x + 1), (y, x - 1)],
                    in_loop: false,
                }),

                b'L' => Elem::Pipe(PipeType {
                    symbol: b,
                    coord: (y, x),
                    entries: [(y - 1, x), (y, x + 1)],
                    in_loop: false,
                }),

                b'J' => Elem::Pipe(PipeType {
                    symbol: b,
                    coord: (y, x),

                    entries: [(y - 1, x), (y, x - 1)],
                    in_loop: false,
                }),

                b'7' => Elem::Pipe(PipeType {
                    symbol: b,
                    coord: (y, x),
                    entries: [(y + 1, x), (y, x - 1)],
                    in_loop: false,
                }),

                b'F' => Elem::Pipe(PipeType {
                    symbol: b,
                    coord: (y, x),
                    entries: [(y + 1, x), (y, x + 1)],
                    in_loop: false,
                }),
                b'S' => {
                    start = PipeType {
                        symbol: b,
                        coord: (y, x),
                        entries: [(y, x), (y, x)],
                        in_loop: false,
                    };
                    Elem::Start(start)
                }
                _ => unimplemented!(),
            };

            pipeline.pipes[y].push(elem);
        });
    });

    let mut steps = 1usize;
    let mut starting_point: Option<PipeType> = None;
    for (y, x) in sides {
        let (yy, xx) = (
            start.coord.0.checked_add_signed(y),
            start.coord.1.checked_add_signed(x),
        );
        if yy.is_some() && xx.is_some() {
            let p: &Elem = &pipeline.pipes[yy.unwrap()][xx.unwrap()];

            match p {
                Elem::Ground => {}
                Elem::Pipe(p) => {
                    if p.can_enter_from(&start) {
                        starting_point = Some(*p);
                        break;
                    }
                }
                Elem::Start(_) => {}
            }
        }
    }

    if starting_point.is_none() {
        panic!("Starting point must have pipes connected to it");
    }

    let starting_point = starting_point.unwrap();

    let mut current = starting_point;
    let mut prev = start.coord;
    let mut path = Path {
        x: vec![prev.0],
        y: vec![prev.1],
    };

    while current.coord != start.coord {
        for &(y, x) in current.entries.iter().filter(|c| **c != prev) {
            let p = &mut pipeline.pipes[y][x];

            match p {
                Elem::Ground => {}
                Elem::Start(p) => {
                    current = *p;
                    steps += 1;
                    break;
                }
                Elem::Pipe(p) => {
                    if p.can_enter_from(&current) {
                        prev = current.coord;
                        current = *p;
                        steps += 1;
                        p.symbol = b'O';
                        p.in_loop = true;
                        path.append(current.coord);
                        break;
                    }
                }
            }
        }
    }

    println!("PART 2 ans {}", part_2(path));

    steps / 2
}

fn part_2(path: Path) -> usize {
    let mut trailing = 0usize;

    for i in 0..path.x.len() {
        if i == path.x.len() - 1 {
            trailing += (path.y[i] + path.y[0]) * (path.x[i] - path.x[0])
        } else {
            trailing += (path.y[i] + path.y[i + 1]) * (path.x[i] - path.x[i + 1])
        }
    }

    let area = trailing / 2;

    println!("len / 2 is {} {trailing} {area}", path.x.len() / 2);

    area - (path.x.len() / 2)
}

fn part_two(input: &str) -> usize {
    42
}

pub fn solution(input: String) -> (usize, usize) {
    (part_one(&input), part_two(&input))
}
