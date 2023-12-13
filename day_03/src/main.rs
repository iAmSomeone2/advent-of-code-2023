use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::ops::RangeInclusive;

lazy_static! {
    static ref SYMBOL_REGEX: Regex = Regex::new(r"[\W&&[^.\n]]").unwrap();
    static ref NUMBER_REGEX: Regex = Regex::new(r"\d+").unwrap();
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct SchematicValue {
    pub value: u64,
    pub bounding_box: (RangeInclusive<usize>, RangeInclusive<usize>),
}

impl SchematicValue {
    pub fn new(value: u64, line_num: usize, line_range: RangeInclusive<usize>) -> Self {
        let x_bounds = if *line_range.start() > 0 {
            (*line_range.start() - 1)..=(*line_range.end() + 1)
        } else {
            0..=(*line_range.end() + 1)
        };
        let y_bounds = if line_num > 0 {
            (line_num - 1)..=(line_num + 1)
        } else {
            0..=(line_num + 1)
        };

        Self {
            value,
            bounding_box: (x_bounds, y_bounds),
        }
    }

    pub fn is_part_number(&self, symbols: &[PartSymbol]) -> bool {
        for symbol in symbols {
            let (x_pos, y_pos) = symbol.location;
            if self.bounding_box.0.contains(&x_pos) && self.bounding_box.1.contains(&y_pos) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct PartSymbol {
    symbol: String,
    location: (usize, usize),
}

impl PartSymbol {
    pub fn gear_ratio(&self, values: &[SchematicValue]) -> Option<u64> {
        if self.symbol != "*" {
            return None;
        }

        // Check overlap with each SchematicValue
        let adjacent_values: Vec<&SchematicValue> = values
            .iter()
            .filter(|value| self.does_overlap(value))
            .collect();
        if adjacent_values.len() != 2 {
            return None;
        }

        Some(adjacent_values[0].value * adjacent_values[1].value)
    }

    fn does_overlap(&self, value: &SchematicValue) -> bool {
        let (x, y) = self.location;
        let (x_bound, y_bound) = &value.bounding_box;
        x_bound.contains(&x) && y_bound.contains(&y)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Schematic {
    pub values: Vec<SchematicValue>,
    pub part_symbols: Vec<PartSymbol>,
}

impl Schematic {
    pub fn from_input_str(input_str: &str) -> Self {
        let mut values = vec![];
        let mut part_symbols = vec![];

        input_str.lines().enumerate().for_each(|(line_num, line)| {
            // Check for symbols
            part_symbols.extend(
                SYMBOL_REGEX
                    .captures_iter(line)
                    .filter_map(|c| c.get(0))
                    .map(|m| PartSymbol {
                        symbol: m.as_str().parse().unwrap(),
                        location: (m.start(), line_num),
                    }),
            );

            // Check for values
            values.extend(
                NUMBER_REGEX
                    .captures_iter(line)
                    .filter_map(|c| c.get(0))
                    .map(|m| {
                        let line_range = m.start()..=(m.end() - 1);
                        let value: u64 = m
                            .as_str()
                            .parse()
                            .expect("matched value should be a base-10 number");
                        SchematicValue::new(value, line_num, line_range)
                    }),
            );
        });

        Self {
            values,
            part_symbols,
        }
    }

    pub fn sum_part_numbers(&self) -> u64 {
        self.values
            .iter()
            .filter(|&value| value.is_part_number(&self.part_symbols))
            .fold(0, |acc, part_num| acc + part_num.value)
    }

    pub fn sum_gear_ratios(&self) -> u64 {
        self.part_symbols
            .iter()
            .filter_map(|part_symbol| part_symbol.gear_ratio(&self.values))
            .sum()
    }
}

fn main() {
    let input_txt = fs::read_to_string("input.txt").expect("failed to open input file");
    let schematic = Schematic::from_input_str(&input_txt);
    drop(input_txt);

    let part_num_sum = schematic.sum_part_numbers();
    println!("Part 1 result: {part_num_sum}");

    let gear_ratio_sum = schematic.sum_gear_ratios();
    println!("Part 2 result: {gear_ratio_sum}");
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_STR: &str = "467..114..\n\
                             ...*......\n\
                             ..35..633.\n\
                             ......#...\n\
                             617*......\n\
                             .....+.58.\n\
                             ..592.....\n\
                             ......755.\n\
                             ...$.*....\n\
                             .664.598..";

    lazy_static! {
        static ref TEST_SCHEMATIC: Schematic = Schematic {
            values: vec![
                SchematicValue {
                    value: 467,
                    bounding_box: (0..=3, 0..=1),
                },
                SchematicValue {
                    value: 114,
                    bounding_box: (4..=8, 0..=1),
                },
                SchematicValue {
                    value: 35,
                    bounding_box: (1..=4, 1..=3),
                },
                SchematicValue {
                    value: 633,
                    bounding_box: (5..=9, 1..=3),
                },
                SchematicValue {
                    value: 617,
                    bounding_box: (0..=3, 3..=5),
                },
                SchematicValue {
                    value: 58,
                    bounding_box: (6..=9, 4..=6),
                },
                SchematicValue {
                    value: 592,
                    bounding_box: (1..=5, 5..=7),
                },
                SchematicValue {
                    value: 755,
                    bounding_box: (5..=9, 6..=8),
                },
                SchematicValue {
                    value: 664,
                    bounding_box: (0..=4, 8..=10),
                },
                SchematicValue {
                    value: 598,
                    bounding_box: (4..=8, 8..=10),
                },
            ],
            part_symbols: vec![
                PartSymbol {
                    symbol: String::from('*'),
                    location: (3, 1)
                },
                PartSymbol {
                    symbol: String::from('#'),
                    location: (6, 3)
                },
                PartSymbol {
                    symbol: String::from('*'),
                    location: (3, 4)
                },
                PartSymbol {
                    symbol: String::from('+'),
                    location: (5, 5)
                },
                PartSymbol {
                    symbol: String::from('$'),
                    location: (3, 8)
                },
                PartSymbol {
                    symbol: String::from('*'),
                    location: (5, 8)
                },
            ],
        };
    }

    #[test]
    fn standard_new_schematic_value_test() {
        let value = 35;
        let line_num = 2;
        let line_range = 2..=3;

        assert_eq!(
            SchematicValue::new(value, line_num, line_range),
            SchematicValue {
                value,
                bounding_box: (1..=4, 1..=3)
            }
        )
    }

    #[test]
    fn schematic_value_is_part_number_test() {
        let schematic_value = SchematicValue {
            value: 35,
            bounding_box: (1..=4, 1..=3),
        };
        let symbol = PartSymbol {
            symbol: String::from('+'),
            location: (4, 2),
        };
        assert!(schematic_value.is_part_number(&[symbol]));

        let symbol = PartSymbol {
            symbol: String::from('+'),
            location: (4, 6),
        };
        assert!(!schematic_value.is_part_number(&[symbol]));
    }

    #[test]
    fn part_symbol_does_overlap_test() {
        let schematic_value = SchematicValue {
            value: 35,
            bounding_box: (1..=4, 1..=3),
        };
        let symbol = PartSymbol {
            symbol: String::from('+'),
            location: (4, 2),
        };
        assert!(symbol.does_overlap(&schematic_value));

        let symbol = PartSymbol {
            symbol: String::from('+'),
            location: (6, 8),
        };
        assert!(!symbol.does_overlap(&schematic_value));
    }

    #[test]
    fn part_symbol_gear_ratio_test() {
        let values = &TEST_SCHEMATIC.values[0..3];
        let gear_ratio = TEST_SCHEMATIC.part_symbols[0].gear_ratio(values);
        assert_eq!(gear_ratio, Some(16345));
    }

    #[test]
    fn schematic_from_str_test() {
        let schematic = Schematic::from_input_str(INPUT_STR);
        assert_eq!(schematic.values, TEST_SCHEMATIC.values);
        assert_eq!(schematic.part_symbols, TEST_SCHEMATIC.part_symbols);
    }

    #[test]
    fn schematic_sum_part_numbers_test() {
        assert_eq!(TEST_SCHEMATIC.sum_part_numbers(), 4361);
    }

    #[test]
    fn schematic_sum_gear_ratios_test() {
        assert_eq!(TEST_SCHEMATIC.sum_gear_ratios(), 467835);
    }
}
