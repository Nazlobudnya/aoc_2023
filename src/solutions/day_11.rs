use std::collections::HashSet;

#[derive(Debug, Clone)]
struct GalaxyCoord {
    x: isize,
    y: isize,
}

impl GalaxyCoord {
    fn manhattan_distance(&self, other: &Self) -> usize {
        self.y.abs_diff(other.y) + self.x.abs_diff(other.x)
    }
}

#[derive(Debug)]
struct ExpansionTrack {
    x: Vec<isize>,
    y: Vec<isize>,
    expansion_size: isize,
}

impl ExpansionTrack {
    fn new(expansion_size: isize) -> Self {
        let expansion_size = match expansion_size {
            x if x < 0 => 0,
            x if x == 1 => 1,
            x => x - 1,
        };

        Self {
            x: vec![],
            y: vec![],
            expansion_size,
        }
    }
    fn expansion_size(&self, coord: (isize, isize)) -> (isize, isize) {
        let x_expansion_count = self.x.iter().position(|x| coord.0 <= *x);
        let y_expansion_count = self.y.iter().position(|y| coord.1 <= *y);

        let x = if let Some(x_exp) = x_expansion_count {
            x_exp
        } else {
            self.x.len()
        } as isize;

        let y = if let Some(y_exp) = y_expansion_count {
            y_exp
        } else {
            self.y.len()
        } as isize;

        (x, y)
    }

    fn expand_galaxy(&self, coord: &GalaxyCoord) -> GalaxyCoord {
        let (x, y) = self.expansion_size((coord.x, coord.y));

        GalaxyCoord {
            x: coord.x + x * self.expansion_size,
            y: coord.y + y * self.expansion_size,
        }
    }
}

#[derive(Debug, Clone)]
enum MapTile {
    Empty,
    Galaxy,
}

struct Map {
    galaxy: Vec<Vec<MapTile>>,
    galaxies_coord: Vec<GalaxyCoord>,
}

fn sum_of_min_distance(input: &str, expansion_size: isize) -> usize {
    let _height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let mut map = Map {
        galaxy: vec![],
        galaxies_coord: vec![],
    };

    let mut expansion_track = ExpansionTrack::new(expansion_size);

    let mut x_with_galaxies = HashSet::<usize>::with_capacity(width);

    input.lines().enumerate().for_each(|(y, line)| {
        map.galaxy.push(vec![MapTile::Empty; width]);

        let mut found_galaxy = false;

        line.bytes().enumerate().for_each(|(x, c)| match c {
            b'.' => {}
            b'#' => {
                found_galaxy = true;
                map.galaxy[y][x] = MapTile::Galaxy;
                map.galaxies_coord.push(GalaxyCoord {
                    x: x as isize,
                    y: y as isize,
                });
                x_with_galaxies.insert(x);
            }
            _ => unimplemented!(),
        });

        if !found_galaxy {
            expansion_track.y.push(y as isize);
        }
    });

    for x in 0..width {
        if !x_with_galaxies.contains(&x) {
            expansion_track.x.push(x as isize);
        }
    }

    let mut distances = vec![];
    for (i, coord) in map.galaxies_coord.iter().enumerate() {
        let f = expansion_track.expand_galaxy(coord);

        for other in map.galaxies_coord.iter().skip(i + 1) {
            distances.push(f.manhattan_distance(&expansion_track.expand_galaxy(other)));
        }
    }

    distances.iter().sum::<usize>()
}

pub fn solution(input: String) -> (usize, usize) {
    (
        sum_of_min_distance(&input, 1),
        sum_of_min_distance(&input, 1_000_000),
    )
}
