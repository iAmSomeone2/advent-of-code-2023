use indicatif::{ParallelProgressIterator, ProgressStyle};
use lazy_static::lazy_static;
use rayon::prelude::*;
use std::fs;
use std::str::FromStr;

lazy_static! {
    static ref PROGRESS_STYLE: ProgressStyle =
        ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>10}/{len:10}",)
            .unwrap()
            .progress_chars("##-");
}

#[derive(Debug, Eq, PartialEq)]
struct BoatMultiRace {
    times: Vec<u32>,
    distances: Vec<u32>,
    race_count: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct ParseBoatRaceError;

impl BoatMultiRace {
    fn parse_line_by_key(key: &str, line: &str) -> Result<Vec<u32>, ParseBoatRaceError> {
        let mut line_split = line.split(':');

        let line_key = line_split.next();
        if line_key.is_none() {
            return Err(ParseBoatRaceError);
        }
        if line_key.unwrap() != key {
            return Err(ParseBoatRaceError);
        }

        // Get and return numbers
        let numbers_str = line_split.next();
        if numbers_str.is_none() {
            return Ok(vec![]);
        }

        Ok(numbers_str
            .unwrap()
            .split_whitespace()
            .filter_map(|num_str| num_str.parse::<u32>().ok())
            .collect())
    }
}

impl FromStr for BoatMultiRace {
    type Err = ParseBoatRaceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        // Get times
        let current_line = lines.next();
        if current_line.is_none() {
            return Err(ParseBoatRaceError);
        }
        let times = BoatMultiRace::parse_line_by_key("Time", current_line.unwrap())?;

        // Get distances
        let current_line = lines.next();
        if current_line.is_none() {
            return Err(ParseBoatRaceError);
        }
        let distances = BoatMultiRace::parse_line_by_key("Distance", current_line.unwrap())?;

        if times.len() != distances.len() {
            return Err(ParseBoatRaceError);
        }
        let race_count = times.len();

        Ok(Self {
            times,
            distances,
            race_count,
        })
    }
}

impl BoatMultiRace {
    fn can_win(time: u32, winning_distance: u32, hold_time: u32) -> bool {
        let speed = hold_time;
        let remaining_time = time - hold_time;
        let distance_traveled = remaining_time * speed;

        distance_traveled >= winning_distance
    }

    pub fn count_winning_configs(&self, race_id: usize) -> usize {
        let time = self.times[race_id];
        let winning_distance = self.distances[race_id] + 1;

        (1..time)
            .filter(|t| {
                let hold_time = time - t;
                BoatMultiRace::can_win(time, winning_distance, hold_time)
            })
            .count()
    }

    pub fn count_all_winning_configs(&self) -> usize {
        (0..self.race_count)
            .map(|race_id| self.count_winning_configs(race_id))
            .reduce(|acc, count| acc * count)
            .unwrap_or(0)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct BoatSingleRace {
    time: usize,
    distance: usize,
}

impl BoatSingleRace {
    fn parse_line_by_key(key: &str, line: &str) -> Result<usize, ParseBoatRaceError> {
        let mut line_split = line.split(':');

        let line_key = line_split.next();
        if line_key.is_none() {
            return Err(ParseBoatRaceError);
        }
        if line_key.unwrap() != key {
            return Err(ParseBoatRaceError);
        }

        // Get and return number
        let numbers_str = line_split.next();
        if numbers_str.is_none() {
            return Err(ParseBoatRaceError);
        }
        let number_str = numbers_str.unwrap().replace(" ", "");

        number_str.parse().or(Err(ParseBoatRaceError))
    }
}

impl FromStr for BoatSingleRace {
    type Err = ParseBoatRaceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        // Get times
        let current_line = lines.next();
        if current_line.is_none() {
            return Err(ParseBoatRaceError);
        }
        let time = BoatSingleRace::parse_line_by_key("Time", current_line.unwrap())?;

        // Get distances
        let current_line = lines.next();
        if current_line.is_none() {
            return Err(ParseBoatRaceError);
        }
        let distance = BoatSingleRace::parse_line_by_key("Distance", current_line.unwrap())?;

        Ok(Self { time, distance })
    }
}

impl BoatSingleRace {
    fn can_win(&self, hold_time: usize) -> bool {
        let speed = hold_time;
        let remaining_time = self.time - hold_time;
        let distance_traveled = remaining_time * speed;

        distance_traveled > self.distance
    }

    pub fn count_winning_configs(&self) -> usize {
        (1..self.time)
            .into_par_iter()
            .progress_with_style(PROGRESS_STYLE.clone())
            .filter(|t| {
                let hold_time = self.time - t;
                self.can_win(hold_time)
            })
            .count()
    }
}

fn main() {
    let boat_race = fs::read_to_string("input.txt")
        .expect("failed to read input file")
        .parse::<BoatMultiRace>()
        .expect("failed to parse input data");

    let winning_combos = boat_race.count_all_winning_configs();
    println!("Part 1 result: {winning_combos}");

    let boat_race = fs::read_to_string("input.txt")
        .expect("failed to read input file")
        .parse::<BoatSingleRace>()
        .expect("failed to parse input data");
    let winning_combos = boat_race.count_winning_configs();
    println!("Part 2 result: {winning_combos}");
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "Time:      7  15   30\n\
                              Distance:  9  40  200";

    #[test]
    fn parse_boat_multi_race_from_str() {
        let expected = BoatMultiRace {
            times: vec![7, 15, 30],
            distances: vec![9, 40, 200],
            race_count: 3,
        };

        assert_eq!(TEST_INPUT.parse::<BoatMultiRace>(), Ok(expected));

        assert_eq!(
            "Timmy: 0 1 3".parse::<BoatMultiRace>(),
            Err(ParseBoatRaceError)
        );
    }

    #[test]
    fn boat_multi_race_can_win() {
        let test_data = [(0, false), (1, false), (2, true), (6, false), (7, false)];

        for (hold_time, expected) in test_data {
            assert_eq!(BoatMultiRace::can_win(7, 10, hold_time), expected);
        }
    }

    #[test]
    fn boat_multi_race_count_winning_configs() {
        let boat_race = BoatMultiRace {
            times: vec![7, 15, 30],
            distances: vec![9, 40, 200],
            race_count: 3,
        };
        let expected = 4;

        assert_eq!(boat_race.count_winning_configs(0), expected);
    }

    #[test]
    fn boat_multi_race_count_all_winning_configs() {
        let boat_race = BoatMultiRace {
            times: vec![7, 15, 30],
            distances: vec![9, 40, 200],
            race_count: 3,
        };
        let expected = 288;

        assert_eq!(boat_race.count_all_winning_configs(), expected);
    }

    #[test]
    fn parse_boat_single_race_from_str() {
        let expected = BoatSingleRace {
            time: 71_530,
            distance: 940_200,
        };

        assert_eq!(TEST_INPUT.parse::<BoatSingleRace>(), Ok(expected));

        assert_eq!(
            "Time: 03\nDistance: 22q".parse::<BoatSingleRace>(),
            Err(ParseBoatRaceError)
        );
    }
}
