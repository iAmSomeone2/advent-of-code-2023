use std::collections::HashSet;
use std::fs;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Galaxy {
    /// Row in the [GalaxyMap] where this [Galaxy] is located
    x: u64,
    /// Column in the [GalaxyMap] where this [Galaxy] is located
    y: u64,
}

impl Galaxy {
    fn steps_to(&self, other: &Galaxy) -> u64 {
        return if self.x == other.x {
            self.y.abs_diff(other.y)
        } else if self.y == other.y {
            self.x.abs_diff(other.x)
        } else {
            self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
        };
    }
}

#[derive(Debug)]
struct Pair<T> {
    a: T,
    b: T,
}

impl<T> Pair<T> {
    fn new(a: T, b: T) -> Self {
        Self { a, b }
    }
}

impl<T> PartialEq for Pair<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.a.eq(&other.a) && self.b.eq(&other.b) || self.a.eq(&other.b) && self.b.eq(&other.a)
    }
}

impl<T> Eq for Pair<T> where T: Eq {}

impl Hash for Pair<usize> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let val = self.a + self.b;
        val.hash(state);
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct GalaxyMap {
    galaxies: Vec<Galaxy>,
    width: u64,
    height: u64,
}

#[derive(Debug, Eq, PartialEq)]
struct ParseGalaxyMapError;

impl FromStr for GalaxyMap {
    type Err = ParseGalaxyMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut height = 0;
        let galaxies = s
            .lines()
            .enumerate()
            .map(|(row_idx, line)| {
                height += 1;
                let row = row_idx.clone() as u64;
                line.chars().enumerate().filter_map(move |(col_idx, c)| {
                    let col = col_idx.clone() as u64;
                    match c {
                        '#' => Some(Galaxy { x: col, y: row }),
                        _ => None,
                    }
                })
            })
            .flatten()
            .collect::<Vec<Galaxy>>();

        // There has to be a better way to do this
        let width = galaxies
            .iter()
            .max_by(|&a, &b| a.x.cmp(&b.x))
            .expect("failed to find rightmost Galaxy")
            .x
            + 1;

        Ok(Self {
            galaxies,
            width,
            height,
        })
    }
}

impl GalaxyMap {
    /// Expands the number of empty rows and columns between [Galaxy] instances
    fn expand_empty_space(&mut self, amt: u64) {
        let amt = if amt == 1 {
            amt
        } else {
            amt - 1
        };

        let mut row = 0;
        while row < self.height {
            let is_empty = self
                .galaxies
                .iter()
                .find(|&galaxy| galaxy.y == row)
                .is_none();

            if is_empty {
                self.galaxies
                    .iter_mut()
                    .filter(|galaxy| galaxy.y > row)
                    .for_each(|galaxy| {
                        galaxy.y += amt;
                    });
                self.height += amt;
                row += amt;
            }
            row += 1;
        }

        let mut col = 0;
        while col < self.width {
            let is_empty = self
                .galaxies
                .iter()
                .find(|&galaxy| galaxy.x == col)
                .is_none();

            if is_empty {
                self.galaxies
                    .iter_mut()
                    .filter(|galaxy| galaxy.x > col)
                    .for_each(|galaxy| {
                        galaxy.x += amt;
                    });
                self.width += amt;
                col += amt;
            }
            col += 1;
        }
    }

    fn pair_galaxy_ids(galaxies: &Vec<Galaxy>) -> HashSet<Pair<usize>> {
        let mut pair_set = HashSet::new();
        for i in 0..galaxies.len() {
            for j in 0..galaxies.len() {
                if i != j {
                    pair_set.insert(Pair::new(i, j));
                }
            }
        }

        pair_set
    }

    fn sum_galaxy_steps(&self) -> u64 {
        let id_pairs = GalaxyMap::pair_galaxy_ids(&self.galaxies);

        id_pairs
            .iter()
            .map(|pair| {
                let galaxy_a = &self.galaxies[pair.a];
                let galaxy_b = &self.galaxies[pair.b];
                galaxy_a.steps_to(galaxy_b)
            })
            .sum()
    }
}

const P2_AMT: u64 = 1_000_000;

fn main() {
    let mut galaxy_map_p1 = fs::read_to_string("input.txt")
        .expect("failed to open input file")
        .parse::<GalaxyMap>()
        .expect("failed to parse input data");
    let mut galaxy_map_p2 = galaxy_map_p1.clone();

    galaxy_map_p1.expand_empty_space(1);
    let sum_of_distances = galaxy_map_p1.sum_galaxy_steps();
    println!("Part 1 result: {}", sum_of_distances);

    galaxy_map_p2.expand_empty_space(P2_AMT);
    let sum_of_distances = galaxy_map_p2.sum_galaxy_steps();
    println!("Part 2 result: {}", sum_of_distances);
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;

    const TEST_INPUT: &str = "...#......\n\
                               .......#..\n\
                               #.........\n\
                               ..........\n\
                               ......#...\n\
                               .#........\n\
                               .........#\n\
                               ..........\n\
                               .......#..\n\
                               #...#.....";

    lazy_static! {
        static ref TEST_GALAXY_MAP: GalaxyMap = GalaxyMap {
            width: 10,
            height: 10,
            galaxies: vec![
                Galaxy { x: 3, y: 0 },
                Galaxy { x: 7, y: 1 },
                Galaxy { x: 0, y: 2 },
                Galaxy { x: 6, y: 4 },
                Galaxy { x: 1, y: 5 },
                Galaxy { x: 9, y: 6 },
                Galaxy { x: 7, y: 8 },
                Galaxy { x: 0, y: 9 },
                Galaxy { x: 4, y: 9 }
            ],
        };
    }

    mod pair {
        use crate::Pair;

        #[test]
        fn equal() {
            let test_data = [
                (Pair::new(1, 2), Pair::new(1, 2), true),
                (Pair::new(2, 1), Pair::new(1, 2), true),
                (Pair::new(2, 3), Pair::new(4, 2), false),
            ];

            for (a, b, expected) in test_data {
                assert_eq!(a.eq(&b), expected);
            }
        }
    }

    mod galaxy {
        use crate::Galaxy;

        #[test]
        fn vertical_distance() {
            let g0 = Galaxy { x: 4, y: 10 };
            let g1 = Galaxy { x: 4, y: 3 };
            let expected = 7;

            assert_eq!(g0.steps_to(&g1), expected);
        }

        #[test]
        fn horizontal_distance() {
            let g0 = Galaxy { x: 10, y: 3 };
            let g1 = Galaxy { x: 4, y: 3 };
            let expected = 6;

            assert_eq!(g0.steps_to(&g1), expected);
        }

        #[test]
        fn diagonal_distance() {
            let g0 = Galaxy { x: 1, y: 6 };
            let g1 = Galaxy { x: 5, y: 11 };
            let expected = 9;

            assert_eq!(g0.steps_to(&g1), expected);
        }
    }

    mod galaxy_map {
        use crate::test::{TEST_GALAXY_MAP, TEST_INPUT};
        use crate::GalaxyMap;

        #[test]
        fn parse_from_str() {
            let actual_map = TEST_INPUT.parse::<GalaxyMap>();
            assert!(actual_map.is_ok());

            let actual_map = actual_map.unwrap();
            assert_eq!(actual_map.height, TEST_GALAXY_MAP.height);
            assert_eq!(actual_map.width, TEST_GALAXY_MAP.width);
            assert_eq!(actual_map.galaxies, TEST_GALAXY_MAP.galaxies);
        }

        #[test]
        fn expand_empty_space() {
            let mut test_map = TEST_GALAXY_MAP.clone();

            test_map.expand_empty_space(1);
            assert_eq!(test_map.height, 12);
            assert_eq!(test_map.width, 13);
        }

        #[test]
        fn pair_galaxy_ids() {
            let test_map = TEST_GALAXY_MAP.clone();

            let ids_map = GalaxyMap::pair_galaxy_ids(&test_map.galaxies);
            assert_eq!(ids_map.len(), 36);
        }

        #[test]
        fn sum_galaxy_steps_p1() {
            let mut test_map = TEST_GALAXY_MAP.clone();
            test_map.expand_empty_space(1);

            assert_eq!(test_map.sum_galaxy_steps(), 374);
        }

        #[test]
        fn sum_galaxy_steps_p2() {
            let mut test_map = TEST_GALAXY_MAP.clone();
            test_map.expand_empty_space(100);

            assert_eq!(test_map.sum_galaxy_steps(), 8410);
        }
    }
}
