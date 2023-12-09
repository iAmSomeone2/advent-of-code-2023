use std::fs;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
struct OASISReport {
    histories: Vec<Vec<i32>>,
}

#[derive(Debug, Eq, PartialEq)]
struct ParseOASISError;

impl FromStr for OASISReport {
    type Err = ParseOASISError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let histories = s
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|num_str| num_str.parse::<i32>().ok())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

        if histories.len() != 0 {
            Ok(Self { histories })
        } else {
            Err(ParseOASISError)
        }
    }
}

impl OASISReport {
    fn compute_differences(vals: &[i32]) -> Vec<i32> {
        vals.iter()
            .enumerate()
            .take(vals.len() - 1)
            .map(|(i, val)| {
                let next_val = &vals[i + 1];
                next_val - val
            })
            .collect::<Vec<i32>>()
    }

    fn infer_next_val(history: &[i32]) -> i32 {
        if history.iter().all(|val| *val == 0) {
            // Base case
            return 0;
        }

        history[history.len() - 1]
            + OASISReport::infer_next_val(&OASISReport::compute_differences(&history))
    }

    fn sum_all_next_vals(&self) -> i32 {
        self.histories
            .iter()
            .map(|history| OASISReport::infer_next_val(&history))
            .sum()
    }

    fn infer_prev_val(history: &[i32]) -> i32 {
        if history.iter().all(|val| *val == 0) {
            // Base case
            return 0;
        }

        history[0] - OASISReport::infer_prev_val(&OASISReport::compute_differences(&history))
    }

    fn sum_all_prev_vals(&self) -> i32 {
        self.histories
            .iter()
            .map(|history| OASISReport::infer_prev_val(&history))
            .sum()
    }
}

fn main() {
    let oasis_report = fs::read_to_string("input.txt")
        .expect("failed to read input file")
        .parse::<OASISReport>()
        .expect("failed to parse input");

    let next_vals_sum = oasis_report.sum_all_next_vals();
    println!("Part 1 result: {next_vals_sum}");

    let prev_vals_sum = oasis_report.sum_all_prev_vals();
    println!("Part 2 result: {prev_vals_sum}");
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "0 3 6 9 12 15\n\
                              1 3 6 10 15 21\n\
                              10 13 16 21 30 45";

    #[test]
    fn parse_oasis_report_from_str() {
        let expected = OASISReport {
            histories: vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![1, 3, 6, 10, 15, 21],
                vec![10, 13, 16, 21, 30, 45],
            ],
        };

        assert_eq!(TEST_INPUT.parse(), Ok(expected));
    }

    #[test]
    fn predict_next_val() {
        let test_data = [
            ([0, 3, 6, 9, 12, 15], 18),
            ([1, 3, 6, 10, 15, 21], 28),
            ([10, 13, 16, 21, 30, 45], 68),
        ];

        for (history, expected) in test_data {
            assert_eq!(OASISReport::infer_next_val(&history), expected);
        }
    }

    #[test]
    fn predict_previous_val() {
        let history = [10, 13, 16, 21, 30, 45];
        let expected = 5;

        assert_eq!(OASISReport::infer_prev_val(&history), expected);
    }
}
