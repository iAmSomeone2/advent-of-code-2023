use std::fs;
use std::collections::HashMap;
use lazy_static::lazy_static;

const DIGIT_REPLACE_MAP: &[(&str, &str)] = &[
    ("oneight", "18"),
    ("eightwo", "82"),
    ("nineight", "98"),
    ("twone", "21"),
    ("threeight", "38"),
    ("fiveight", "58"),
    ("sevenine", "79"),
    ("zero", "0"),
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

lazy_static! {
    static ref DIGIT_MAP: HashMap<&'static str, u32> = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
}

fn replace_with_digits(line: &str) -> String {
    let mut tmp_str = String::from(line);
    for (original, replacement) in DIGIT_REPLACE_MAP {
        tmp_str = tmp_str.replace(original, replacement);
    }
    tmp_str
}

fn parse_line_digits_v1(line: &str) -> u32 {
    let digits = line
        .chars()
        .filter(|c| c.is_ascii_digit())
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>();
    (digits[0] * 10) + digits[digits.len() - 1]
}

fn parse_line_digits_v2(line: &str) -> u32 {
    let new_line = replace_with_digits(line);
    parse_line_digits_v1(&new_line)
}

fn parse_line_values(input_str: &str) -> Vec<u32> {
    input_str.lines()
        .map(parse_line_digits_v1)
        .collect()
}

fn parse_line_values_v2(input_str: &str) -> Vec<u32> {
    input_str.lines()
        .map(parse_line_digits_v2)
        .collect()
}

fn main() {
    let input_data = fs::read_to_string("input.txt")
        .expect("could not open input data");
    let line_val_sum: u32 = parse_line_values(&input_data)
        .iter().sum();
    println!("Part 1 result: {line_val_sum}");

    let line_val_sum: u32 = parse_line_values_v2(&input_data)
        .iter().sum();
    println!("Part 2 result: {line_val_sum}");
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA_V1: &[(&str, u32)] = &[
        ("1abc2", 12),
        ("pqr3stu8vwx", 38),
        ("a1b2c3d4e5f", 15),
        ("treb7uchet", 77),
    ];

    const TEST_DATA_V2: &[(&str, u32)] = &[
        ("two1nine", 29),
        ("eightwothree", 83),
        ("abcone2threexyz", 13),
        ("xtwone3four", 24),
        ("4nineeightseven2", 42),
        ("zoneight234", 14),
        ("7pqrstsixteen", 76),
        ("2oneight", 28),
    ];

    const REPLACE_TEST_DATA: &[(&str, &str)] = &[
        ("two1nine", "219"),
        ("eightwothree", "823"),
        ("abcone2threexyz", "abc123xyz"),
        ("xtwone3four", "x2134"),
        ("4nineeightseven2", "49872"),
        ("zoneight234", "z18234"),
        ("7pqrstsixteen", "7pqrst6teen"),
        ("2oneight", "218"),
        ("35eightwo", "3582"),
        ("seveninepqrz5", "79pqrz5"),
    ];

    #[test]
    fn replace_with_digits_test() {
        for (original, expected) in REPLACE_TEST_DATA {
            assert_eq!(replace_with_digits(original), *expected);
        }
    }

    #[test]
    fn parse_line_digits_v1_test() {
        for (line, expected_val) in TEST_DATA_V1 {
            assert_eq!(parse_line_digits_v1(line), *expected_val);
        }
    }

    #[test]
    fn parse_line_digits_v2_test() {
        for (line, expected_val) in TEST_DATA_V2 {
            assert_eq!(parse_line_digits_v2(line), *expected_val);
        }
    }

    #[test]
    fn parse_line_values_test() {
        let test_lines: String = TEST_DATA_V1.iter()
            .fold(String::from(""), |mut acc, (line, _)| {
                acc.push_str(line);
                acc.push('\n');
                acc
            });
        let expected_vals: Vec<u32> = TEST_DATA_V1.iter()
            .map(|(_, value)| *value)
            .collect();

        assert_eq!(parse_line_values(&test_lines), expected_vals);
    }

    #[test]
    fn parse_line_values_v2_test() {
        let mut test_data = Vec::from(TEST_DATA_V1);
        for data in TEST_DATA_V2 {
            test_data.push(*data);
        }
        let test_lines: String = test_data.iter()
            .fold(String::from(""), |mut acc, (line, _)| {
                acc.push_str(line);
                acc.push('\n');
                acc
            });
        let expected_vals: Vec<u32> = test_data.iter()
            .map(|(_, value)| *value)
            .collect();
        let expected_total: u32 = expected_vals.iter().sum();

        let line_vals = parse_line_values_v2(&test_lines);
        assert_eq!(line_vals, expected_vals);
        assert_eq!(line_vals.iter().sum::<u32>(), expected_total);
    }
}
