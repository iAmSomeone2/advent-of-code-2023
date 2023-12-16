use std::fs;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
struct ParseInitSequenceError;

fn hash_str(s: &str) -> u64 {
    s.chars().fold(0u64, |hash, c| (hash + c as u64) * 17 % 256)
}

enum Operation {
    Remove,
    Insert,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Lens {
    label: String,
    focal_length: u64,
}

#[derive(Debug, Eq, PartialEq)]
struct InitSequence {
    hashes: Vec<u64>,
    lenses: Vec<String>,
}

impl FromStr for InitSequence {
    type Err = ParseInitSequenceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lenses = s
            .split(',')
            .map(|l| String::from_str(l).unwrap())
            .collect::<Vec<String>>();
        let hashes = lenses.iter().map(|l| hash_str(l)).collect();

        Ok(Self { hashes, lenses })
    }
}

impl InitSequence {
    fn sum_of_hashes(&self) -> u64 {
        self.hashes.iter().sum()
    }

    fn box_lenses(&self) -> Vec<Vec<Lens>> {
        let mut lens_boxes: Vec<Vec<Lens>> = vec![vec![]; 256];

        for lens in &self.lenses {
            let op = if lens.contains('=') {
                Operation::Insert
            } else {
                Operation::Remove
            };

            let (label, focal_length) = match op {
                Operation::Insert => {
                    let mut split = lens.split('=');
                    let label = String::from(split.next().unwrap());
                    let focal_length = split.next().unwrap().parse::<u64>().unwrap();
                    (label, Some(focal_length))
                }
                Operation::Remove => {
                    let label = lens.replace('-', "");
                    (label, None)
                }
            };

            let hash = hash_str(&label);
            let lens_box = &mut lens_boxes[hash as usize];
            match op {
                Operation::Insert => {
                    let new_lens = Lens {
                        label,
                        focal_length: focal_length.unwrap(),
                    };
                    if let Some(idx) = lens_box
                        .iter()
                        .position(|lens| lens.label == new_lens.label)
                    {
                        lens_box[idx] = new_lens;
                    } else {
                        lens_box.push(new_lens);
                    }
                }
                Operation::Remove => {
                    if let Some(idx) = lens_box.iter().position(|lens| lens.label == label) {
                        lens_box.remove(idx);
                    }
                }
            }
        }

        lens_boxes
    }
}

fn calculate_focusing_power(lens_boxes: &Vec<Vec<Lens>>) -> u64 {
    lens_boxes
        .iter()
        .enumerate()
        .fold(0u64, |acc, (box_id, lens_box)| {
            acc + lens_box
                .iter()
                .enumerate()
                .fold(0u64, |box_total, (slot, lens)| {
                    let box_id = (box_id + 1) as u64;
                    let slot = (slot + 1) as u64;
                    box_total + (box_id * slot * lens.focal_length)
                })
        })
}

fn main() {
    let init_seq = fs::read_to_string("input.txt")
        .expect("failed to read input file")
        .trim_end()
        .parse::<InitSequence>()
        .expect("failed to parse input file");

    let hash_sum = init_seq.sum_of_hashes();
    println!("Part 1 result: {hash_sum}");

    let lens_boxes = init_seq.box_lenses();
    let power = calculate_focusing_power(&lens_boxes);
    println!("Part 2 result: {power}");
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn hash_str_test() {
        let input = "HASH";
        assert_eq!(hash_str(input), 52);
    }

    #[test]
    fn init_sequence_sum_of_hashes() {
        let init_seq = TEST_INPUT.trim_end().parse::<InitSequence>().unwrap();
        let expected = 1320;

        assert_eq!(init_seq.sum_of_hashes(), expected);
    }

    #[test]
    fn box_lenses_focusing_power() {
        let init_seq = TEST_INPUT.trim_end().parse::<InitSequence>().unwrap();
        let lens_boxes = init_seq.box_lenses();

        let power = calculate_focusing_power(&lens_boxes);
        assert_eq!(power, 145);
    }
}
