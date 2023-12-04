use std::fs;
use std::str::FromStr;

#[derive(Debug, Default, Eq, PartialEq)]
struct CubeHand {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

#[derive(Debug, Eq, PartialEq)]
struct ParseCubeHandError;

impl FromStr for CubeHand {
    type Err = ParseCubeHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hand = CubeHand::default();
        s.split(", ").for_each(|color_str| {
            let mut color_split = color_str.split(' ');
            let count = match color_split.next() {
                Some(c) => c.parse::<u32>().unwrap_or(0),
                None => 0,
            };

            if let Some(name) = color_split.next() {
                match name {
                    "red" => {
                        hand.red = count;
                    }
                    "green" => {
                        hand.green = count;
                    }
                    "blue" => {
                        hand.blue = count;
                    }
                    _ => {}
                }
            }
        });

        Ok(hand)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct CubeGame {
    id: usize,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

#[derive(Debug, Eq, PartialEq)]
struct ParseCubeGameError;

impl FromStr for CubeGame {
    type Err = ParseCubeGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut main_split = s.split(": ");

        // Get game ID
        let game_id = match main_split.next() {
            Some(id_str) => match id_str.split(' ').last() {
                Some(num_str) => num_str.parse::<usize>().ok(),
                None => None,
            },
            None => None,
        };
        if game_id.is_none() {
            return Err(ParseCubeGameError);
        }
        let game_id = game_id.unwrap();

        // Get hands data
        let hands: Option<Vec<CubeHand>> = main_split.next().map(|hands_data| {
            hands_data
                .split("; ")
                .filter_map(|data| data.parse::<CubeHand>().ok())
                .collect()
        });
        if hands.is_none() {
            return Err(ParseCubeGameError);
        }
        let hands = hands.unwrap();

        let max_red: u32 = hands.iter().max_by(|&x, &y| x.red.cmp(&y.red)).unwrap().red;
        let max_green: u32 = hands
            .iter()
            .max_by(|&x, &y| x.green.cmp(&y.green))
            .unwrap()
            .green;
        let max_blue: u32 = hands
            .iter()
            .max_by(|&x, &y| x.blue.cmp(&y.blue))
            .unwrap()
            .blue;

        Ok(Self {
            id: game_id,
            max_red,
            max_green,
            max_blue,
        })
    }
}

impl CubeGame {
    pub fn is_possible(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        if self.max_red > max_red {
            return false;
        }
        if self.max_green > max_green {
            return false;
        }
        if self.max_blue > max_blue {
            return false;
        }
        true
    }

    pub fn sum_of_possible_ids(
        games: &[Self],
        max_red: u32,
        max_green: u32,
        max_blue: u32,
    ) -> usize {
        games
            .iter()
            .filter(|game| game.is_possible(max_red, max_green, max_blue))
            .fold(0, |acc, game| acc + game.id)
    }

    pub fn sum_of_cube_powers(games: &[Self]) -> u32 {
        games.iter().fold(0, |acc, game| {
            let power = game.max_red * game.max_green * game.max_blue;
            acc + power
        })
    }
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {
    let input_str = fs::read_to_string("input.txt").expect("failed to open input data");
    let cube_games: Vec<CubeGame> = input_str
        .lines()
        .filter_map(|s| s.parse::<CubeGame>().ok())
        .collect();
    drop(input_str);

    let id_sum = CubeGame::sum_of_possible_ids(&cube_games, MAX_RED, MAX_GREEN, MAX_BLUE);
    println!("Part 1 result: {id_sum}");

    let power_sum = CubeGame::sum_of_cube_powers(&cube_games);
    println!("Part 2 result: {power_sum}");
}

#[cfg(test)]
mod test {
    use super::*;

    impl CubeHand {
        fn new(r: u32, g: u32, b: u32) -> Self {
            Self {
                red: r,
                green: g,
                blue: b,
            }
        }
    }

    #[test]
    fn cube_hand_from_str_test() {
        let test_data = [
            ("3 blue, 4 red", CubeHand::new(4, 0, 3)),
            ("1 red, 2 green, 6 blue", CubeHand::new(1, 2, 6)),
            ("2 green", CubeHand::new(0, 2, 0)),
        ];

        for (s, expected) in test_data {
            assert_eq!(s.parse(), Ok(expected));
        }
    }

    #[test]
    fn cube_game_from_str_test() {
        let test_str = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let expected = CubeGame {
            id: 2,
            max_red: 1,
            max_blue: 4,
            max_green: 3,
        };
        assert_eq!(test_str.parse(), Ok(expected));

        let test_str = "Game 11";
        assert_eq!(test_str.parse::<CubeGame>(), Err(ParseCubeGameError));
    }

    #[test]
    fn cube_game_is_possible_test() {
        let max_red = 3;
        let max_green = 5;
        let max_blue = 10;
        let test_data = [
            (
                CubeGame {
                    id: 2,
                    max_red: 1,
                    max_blue: 4,
                    max_green: 3,
                },
                true,
            ),
            (
                CubeGame {
                    id: 5,
                    max_red: 1,
                    max_blue: 11,
                    max_green: 3,
                },
                false,
            ),
        ];

        for (game, expected) in test_data {
            assert_eq!(game.is_possible(max_red, max_green, max_blue), expected);
        }
    }

    const GAMES: [CubeGame; 5] = [
        CubeGame {
            id: 1,
            max_red: 4,
            max_green: 2,
            max_blue: 6,
        },
        CubeGame {
            id: 2,
            max_red: 1,
            max_green: 3,
            max_blue: 4,
        },
        CubeGame {
            id: 3,
            max_red: 20,
            max_green: 13,
            max_blue: 6,
        },
        CubeGame {
            id: 4,
            max_red: 14,
            max_green: 3,
            max_blue: 15,
        },
        CubeGame {
            id: 5,
            max_red: 6,
            max_green: 3,
            max_blue: 2,
        },
    ];

    #[test]
    fn cube_game_sum_of_possible_ids_test() {
        let expected_sum = 8;

        assert_eq!(
            CubeGame::sum_of_possible_ids(&GAMES, MAX_RED, MAX_GREEN, MAX_BLUE),
            expected_sum
        );
    }

    #[test]
    fn cube_game_sum_of_cube_powers_test() {
        let expected_sum = 2286;

        assert_eq!(CubeGame::sum_of_cube_powers(&GAMES), expected_sum);
    }
}
