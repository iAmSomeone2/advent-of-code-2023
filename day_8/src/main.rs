use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Right,
    Left,
}

#[derive(Debug, Eq, PartialEq)]
struct ParseDirectionError;

impl TryFrom<char> for Direction {
    type Error = ParseDirectionError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let value = value.to_ascii_uppercase();
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(ParseDirectionError),
        }
    }
}

impl Direction {
    fn parse_directions(line: &str) -> Vec<Direction> {
        line.chars().filter_map(|c| c.try_into().ok()).collect()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    name: String,
    left: String,
    right: String,
}

#[derive(Debug, Eq, PartialEq)]
struct ParseNodeError;

impl FromStr for Node {
    type Err = ParseNodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" = ");

        // Get name
        let name = split.next().ok_or(ParseNodeError)?;

        // Get left and right node substring
        let lr_str = split.next().ok_or(ParseNodeError)?;
        let mut split = lr_str.split(", ");

        // Get left node name
        let left = split.next().ok_or(ParseNodeError)?;
        let left = left.replace('(', "");

        // Get right node name
        let right = split.next().ok_or(ParseNodeError)?;
        let right = right.replace(')', "");

        Ok(Self {
            name: name.to_string(),
            left,
            right,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct NodeMap {
    directions: Vec<Direction>,
    nodes: HashMap<String, Node>,
    start_keys: Vec<String>,
}

#[derive(Debug, Eq, PartialEq)]
struct ParseNodeMapError;

impl FromStr for NodeMap {
    type Err = ParseNodeMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        // Get directions from the first line
        let line = lines.next().ok_or(ParseNodeMapError)?;
        let directions = Direction::parse_directions(line);
        if directions.is_empty() {
            return Err(ParseNodeMapError);
        }

        // Skip the empty line and begin processing Nodes
        let _ = lines.next().ok_or(ParseNodeMapError)?;
        let mut start_keys = vec![];
        let mut node_map = HashMap::new();
        for line in lines {
            let node = line.parse::<Node>().or(Err(ParseNodeMapError))?;
            let node_name = node.name.clone();
            node_map.insert(node_name.clone(), node);
            if node_name.ends_with('A') {
                start_keys.push(node_name);
            }
        }

        Ok(Self {
            directions,
            nodes: node_map,
            start_keys,
        })
    }
}

impl NodeMap {
    fn count_steps(&self, start_key: &str, target_pattern: &str) -> usize {
        let mut node = self.nodes.get(start_key).unwrap();

        self.directions
            .iter()
            .cycle()
            .enumerate()
            .find_map(|(i, direction)| {
                node = match direction {
                    Direction::Left => self.nodes.get(&node.left).unwrap(),
                    Direction::Right => self.nodes.get(&node.right).unwrap(),
                };
                if node.name.ends_with(target_pattern) {
                    Some(i + 1)
                } else {
                    None
                }
            })
            .unwrap_or(0)
    }

    fn lcm_of_steps(&self, target_pattern: &str) -> usize {
        let step_counts = self
            .start_keys
            .par_iter()
            .progress()
            .map(|key| self.count_steps(key, target_pattern))
            .collect::<Vec<usize>>();

        step_counts.iter().copied().reduce(lcm).unwrap_or(0)
    }
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn main() {
    let node_map = fs::read_to_string("input.txt")
        .expect("failed to open input file. Check that it exists at 'input.txt'")
        .parse::<NodeMap>()
        .expect("failed to parse input data");

    let steps = node_map.count_steps("AAA", "ZZZ");

    println!("Part 1 result: {steps}");

    let steps = node_map.lcm_of_steps("Z");

    println!("Part 2 result: {steps}");
}

#[cfg(test)]
mod test {
    mod direction {
        use crate::Direction;

        #[test]
        fn parse_from_line() {
            let input = "LLR";
            let expected = vec![Direction::Left, Direction::Left, Direction::Right];

            assert_eq!(Direction::parse_directions(input), expected);

            assert_eq!(
                Direction::parse_directions("dsfhjla"),
                vec![Direction::Left]
            );
        }
    }
    mod node {
        use crate::{Node, ParseNodeError};

        #[test]
        fn parse_from_str() {
            let input = "AAA = (BBB, CCC)";
            let expected = Node {
                name: String::from("AAA"),
                left: String::from("BBB"),
                right: String::from("CCC"),
            };

            assert_eq!(input.parse(), Ok(expected));

            assert_eq!("AAA = s".parse::<Node>(), Err(ParseNodeError));
        }
    }
    mod node_map {
        use crate::NodeMap;

        const TEST_INPUT: &str = "RL\n\
                                  \n\
                                  AAA = (BBB, CCC)\n\
                                  BBB = (DDD, EEE)\n\
                                  CCC = (ZZZ, GGG)\n\
                                  DDD = (DDD, DDD)\n\
                                  EEE = (EEE, EEE)\n\
                                  GGG = (GGG, GGG)\n\
                                  ZZZ = (ZZZ, ZZZ)";

        const TEST_INPUT_2: &str = "LLR\n\
                                    \n\
                                    AAA = (BBB, BBB)\n\
                                    BBB = (AAA, ZZZ)\n\
                                    ZZZ = (ZZZ, ZZZ)";

        const P2_INPUT: &str = "LR\n\
                                \n\
                                11A = (11B, XXX)\n\
                                11B = (XXX, 11Z)\n\
                                11Z = (11B, XXX)\n\
                                22A = (22B, XXX)\n\
                                22B = (22C, 22C)\n\
                                22C = (22Z, 22Z)\n\
                                22Z = (22B, 22B)\n\
                                XXX = (XXX, XXX)";

        #[test]
        fn parse_from_str() {
            let direction_count = 2;
            let node_count = 7;

            let node_map = TEST_INPUT.parse::<NodeMap>();
            assert!(node_map.is_ok());
            let node_map = node_map.unwrap();

            assert_eq!(node_map.directions.len(), direction_count);
            assert_eq!(node_map.nodes.len(), node_count);
        }

        #[test]
        fn count_steps() {
            let node_map = TEST_INPUT.parse::<NodeMap>().unwrap();
            let expected = 2;

            assert_eq!(node_map.count_steps("AAA", "ZZZ"), expected);

            let node_map = TEST_INPUT_2.parse::<NodeMap>().unwrap();
            let expected = 6;

            assert_eq!(node_map.count_steps("AAA", "ZZZ"), expected);
        }

        #[test]
        fn lcm_of_steps() {
            let node_map = P2_INPUT.parse::<NodeMap>().unwrap();
            let expected = 6;

            assert_eq!(node_map.lcm_of_steps("Z"), expected);
        }
    }
}
