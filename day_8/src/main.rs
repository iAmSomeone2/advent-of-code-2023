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
    index: Option<usize>,
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
            index: None,
        })
    }
}

impl Node {
    fn set_index(&mut self, index: usize) {
        self.index = Some(index);
    }
}

#[derive(Debug, Eq, PartialEq)]
struct NodeMap {
    directions: Vec<Direction>,
    nodes: HashMap<String, Node>,
    first_node: String,
    last_node: String,
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
        let mut first_node = String::new();
        let mut last_node = String::new();
        let mut node_map = HashMap::new();
        for (i, line) in lines.enumerate() {
            let mut node = line.parse::<Node>().or(Err(ParseNodeMapError))?;
            node.set_index(i);
            node_map.insert(node.name.clone(), node);
        }

        Ok(Self {
            directions,
            nodes: node_map,
            first_node: String::from("AAA"),
            last_node: String::from("ZZZ"),
        })
    }
}

impl NodeMap {
    fn count_steps(&self) -> usize {
        let final_node_name = &self.last_node;
        let mut current_node_name = &self.first_node;

        let mut step_count = 0;
        let mut dir_idx = 0;
        while current_node_name != final_node_name {
            let node = self.nodes.get(current_node_name).unwrap();
            current_node_name = match self.directions[dir_idx] {
                Direction::Left => &node.left,
                Direction::Right => &node.right,
            };
            step_count += 1;
            dir_idx += 1;
            if dir_idx >= self.directions.len() {
                dir_idx = 0;
            }
        }

        step_count
    }
}

fn main() {
    let node_map = fs::read_to_string("input.txt")
        .expect("failed to open input file. Check that it exists at 'input.txt'")
        .parse::<NodeMap>()
        .expect("failed to parse input data");

    let steps = node_map.count_steps();

    println!("Part 1 result: {steps}");
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
                index: None,
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

            assert_eq!(node_map.count_steps(), expected);

            let node_map = TEST_INPUT_2.parse::<NodeMap>().unwrap();
            let expected = 6;

            assert_eq!(node_map.count_steps(), expected);
        }
    }
}
