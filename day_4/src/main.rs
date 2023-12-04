use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
struct Scratchcard {
    id: usize,
    count: usize,
    winning_numbers: Vec<u64>,
    scratched_numbers: Vec<u64>,
}

#[derive(Debug, Eq, PartialEq)]
struct ParseScratchcardError;

impl FromStr for Scratchcard {
    type Err = ParseScratchcardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut id_data_split = s.split(':');
        // Get card ID
        let id = match id_data_split.next() {
            Some(id_str) => match id_str.split(' ').last() {
                Some(num_str) => num_str.parse::<usize>(),
                None => Ok(0),
            },
            None => Ok(0),
        };
        if id.is_err() || id == Ok(0) {
            return Err(ParseScratchcardError);
        }
        let id = id.unwrap();

        // Split numbers into winning and scratched
        let numbers_split = id_data_split.next().map(|s| s.split('|'));
        if numbers_split.is_none() {
            return Err(ParseScratchcardError);
        }
        let mut numbers_split = numbers_split.unwrap();

        // Get winning numbers
        let winning_num_split = numbers_split.next().map(|s| s.split(' '));
        if winning_num_split.is_none() {
            return Err(ParseScratchcardError);
        }
        let winning_num_split = winning_num_split.unwrap();
        let winning_numbers = winning_num_split
            .filter_map(|num| num.parse::<u64>().ok())
            .collect();

        // Get scratched numbers
        let scratched_num_split = numbers_split.next().map(|s| s.split(' '));
        if scratched_num_split.is_none() {
            return Err(ParseScratchcardError);
        }
        let scratched_num_split = scratched_num_split.unwrap();
        let scratched_numbers = scratched_num_split
            .filter_map(|num| num.parse::<u64>().ok())
            .collect();

        Ok(Self {
            id,
            count: 1,
            winning_numbers,
            scratched_numbers,
        })
    }
}

impl Scratchcard {
    pub fn from_file(path: &str) -> Vec<Scratchcard> {
        let input_file = File::open(path).expect("failed to open input file");
        let reader = BufReader::new(input_file);
        reader
            .lines()
            .map_while(Result::ok)
            .filter_map(|line| line.parse::<Scratchcard>().ok())
            .collect()
    }

    pub fn calculate_score(&self) -> u64 {
        self.scratched_numbers
            .iter()
            .filter(|&num| self.winning_numbers.contains(num))
            .fold(0, |acc, _| {
                if acc == 0 {
                    return 1;
                }
                acc * 2
            })
    }

    pub fn calculate_total_score(scratchcards: &[Self]) -> u64 {
        scratchcards.iter().map(|card| card.calculate_score()).sum()
    }

    pub fn calculate_matching_count(&self) -> usize {
        self.scratched_numbers
            .iter()
            .filter(|&num| self.winning_numbers.contains(num))
            .count()
    }

    /// Runs the count and copy algorithm for part 2
    pub fn run_copy_game(scratchcards: &mut [Self]) -> usize {
        let card_count = scratchcards.len();
        for i in 0..card_count {
            let match_count = scratchcards[i].calculate_matching_count();
            let copy_range = if (i + match_count) > card_count {
                (i + 1)..=(card_count - 1)
            } else {
                (i + 1)..=(i + match_count)
            };
            for j in copy_range {
                scratchcards[j].count += scratchcards[i].count;
            }
        }

        scratchcards
            .iter()
            .fold(0, |card_count, card| card_count + card.count)
    }
}

fn main() {
    let mut scratchcards = Scratchcard::from_file("input.txt");

    let total_score = Scratchcard::calculate_total_score(&scratchcards);
    println!("Part 1 result: {total_score}");

    let total_cards = Scratchcard::run_copy_game(&mut scratchcards);
    println!("Part 2 result: {total_cards}");
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
                              Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
                              Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
                              Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
                              Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
                              Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn scratchcard_from_str_test() {
        let expected_card = Scratchcard {
            id: 1,
            count: 1,
            winning_numbers: vec![41, 48, 83, 86, 17],
            scratched_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };

        let line = TEST_INPUT.lines().next().unwrap();
        assert_eq!(line.parse::<Scratchcard>(), Ok(expected_card));

        let line = "Card 1: q z f";
        assert_eq!(line.parse::<Scratchcard>(), Err(ParseScratchcardError));
    }

    #[test]
    fn scratchcard_calculate_score_test() {
        let expected_card = Scratchcard {
            id: 1,
            count: 1,
            winning_numbers: vec![41, 48, 83, 86, 17],
            scratched_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };
        let expected_score = 8;
        assert_eq!(expected_card.calculate_score(), expected_score);

        let expected_card = Scratchcard {
            id: 5,
            count: 1,
            winning_numbers: vec![87, 83, 26, 28, 32],
            scratched_numbers: vec![88, 30, 70, 12, 93, 22, 82, 36],
        };
        let expected_score = 0;
        assert_eq!(expected_card.calculate_score(), expected_score);
    }

    #[test]
    fn scratchcard_calculate_total_score_test() {
        let scratchcards: Vec<Scratchcard> = TEST_INPUT
            .lines()
            .filter_map(|line| line.parse().ok())
            .collect();
        let expected = 13;
        assert_eq!(Scratchcard::calculate_total_score(&scratchcards), expected);
    }

    #[test]
    fn scratchcard_calculate_matching_count_test() {
        let expected_card = Scratchcard {
            id: 1,
            count: 1,
            winning_numbers: vec![41, 48, 83, 86, 17],
            scratched_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };
        let expected = 4;

        assert_eq!(expected_card.calculate_matching_count(), expected);
    }

    #[test]
    fn scratchcard_run_copy_game_test() {
        let mut scratchcards: Vec<Scratchcard> = TEST_INPUT
            .lines()
            .filter_map(|line| line.parse().ok())
            .collect();
        let expected = 30;
        assert_eq!(Scratchcard::run_copy_game(&mut scratchcards), expected);
    }
}
