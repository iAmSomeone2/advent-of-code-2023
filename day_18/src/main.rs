use image::{ImageBuffer, Rgb, RgbImage};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fs;
use std::ops::RangeInclusive;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq)]
struct ParseDirectionError;

impl TryFrom<char> for Direction {
    type Error = ParseDirectionError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_uppercase() {
            'U' => Ok(Self::Up),
            'D' => Ok(Self::Down),
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(ParseDirectionError),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, Eq, PartialEq)]
struct ParseColorError;

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        let r = (value & 0x00FF0000) >> 16;
        let g = (value & 0x0000FF00) >> 8;
        let b = value & 0x000000FF;

        Self {
            red: r as u8,
            green: g as u8,
            blue: b as u8,
        }
    }
}

impl From<Color> for Rgb<u8> {
    fn from(value: Color) -> Self {
        Rgb([value.red, value.green, value.blue])
    }
}

impl FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('#') {
            return Err(ParseColorError);
        }

        let color_val = u32::from_str_radix(&s[1..], 16).or(Err(ParseColorError))?;
        Ok(Color::from(color_val))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct DigInstruction {
    direction: Direction,
    length: u32,
    color: Color,
}

#[derive(Debug, Eq, PartialEq)]
struct ParseDigInstructionError;

impl FromStr for DigInstruction {
    type Err = ParseDigInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let current = split
            .next()
            .ok_or(ParseDigInstructionError)?
            .chars()
            .next()
            .ok_or(ParseDigInstructionError)?;
        let direction = Direction::try_from(current).or(Err(ParseDigInstructionError))?;

        let current = split.next().ok_or(ParseDigInstructionError)?;
        let length = current.parse::<u32>().or(Err(ParseDigInstructionError))?;

        let current = split.next().ok_or(ParseDigInstructionError)?;
        let current = &current[1..current.len() - 1];

        let color = current.parse::<Color>().or(Err(ParseDigInstructionError))?;

        Ok(Self {
            direction,
            length,
            color,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct TrenchSegment {
    start: (i32, i32),
    end: (i32, i32),
    max_x: i32,
    min_x: i32,
    max_y: i32,
    min_y: i32,
    color: Color,
}

impl TrenchSegment {
    fn shift(&mut self, x_shift: i32, y_shift: i32) {
        self.start.0 += x_shift;
        self.start.1 += y_shift;
        self.end.0 += x_shift;
        self.end.1 += y_shift;

        self.min_x += x_shift;
        self.max_x += x_shift;
        self.min_y += y_shift;
        self.max_y += y_shift;
    }
}

#[derive(Debug, Eq, PartialEq)]
struct LavaductLagoon {
    width: u16,
    height: u16,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    dig_position: (i32, i32),
    trench_segments: Vec<TrenchSegment>,
}

impl Default for LavaductLagoon {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            min_x: i32::MAX,
            max_x: i32::MIN,
            min_y: i32::MAX,
            max_y: i32::MIN,
            dig_position: (0, 0),
            trench_segments: vec![],
        }
    }
}

fn absolute_range(a: u32, b: u32) -> RangeInclusive<u32> {
    match a.cmp(&b) {
        Ordering::Less | Ordering::Equal => a..=b,
        Ordering::Greater => b..=a,
    }
}

impl LavaductLagoon {
    fn dig_trench(&mut self, dig_instruction: &DigInstruction) {
        let length = dig_instruction.length as u16;
        let start_point = self.dig_position;
        let end_point;
        let max_x;
        let min_x;
        let max_y;
        let min_y;

        match dig_instruction.direction {
            Direction::Up => {
                let x = start_point.0;
                let y = start_point.1 - (length as i32);

                end_point = (x, y);
                max_x = end_point.0;
                min_x = end_point.0;
                max_y = start_point.1;
                min_y = end_point.1;
            }
            Direction::Down => {
                let x = start_point.0;
                let y = start_point.1 + (length as i32);

                end_point = (x, y);
                max_x = end_point.0;
                min_x = end_point.0;
                max_y = end_point.1;
                min_y = start_point.1;
            }
            Direction::Left => {
                let x = start_point.0 - (length as i32);
                let y = start_point.1;

                end_point = (x, y);
                max_x = start_point.0;
                min_x = end_point.0;
                max_y = end_point.1;
                min_y = end_point.1;
            }
            Direction::Right => {
                let x = start_point.0 + (length as i32);
                let y = start_point.1;

                end_point = (x, y);
                max_x = end_point.0;
                min_x = start_point.0;
                max_y = end_point.1;
                min_y = end_point.1;
            }
        }

        let segment = TrenchSegment {
            start: start_point,
            end: end_point,
            max_x,
            min_x,
            max_y,
            min_y,
            color: dig_instruction.color.clone(),
        };

        self.dig_position = end_point;
        self.trench_segments.push(segment);
    }

    fn dig_trenches(&mut self, dig_instructions: &[DigInstruction]) {
        // Dig trenches
        println!("Digging...");
        for instruction in dig_instructions {
            self.dig_trench(instruction);
        }

        // Determine dimensions
        println!("Determining dimensions...");
        let min_x = self
            .trench_segments
            .iter()
            .min_by(|&a, &b| a.min_x.cmp(&b.min_x))
            .unwrap()
            .min_x;
        let max_x = self
            .trench_segments
            .iter()
            .max_by(|&a, &b| a.max_x.cmp(&b.max_x))
            .unwrap()
            .max_x;
        let min_y = self
            .trench_segments
            .iter()
            .min_by(|&a, &b| a.min_y.cmp(&b.min_y))
            .unwrap()
            .min_y;
        let max_y = self
            .trench_segments
            .iter()
            .max_by(|&a, &b| a.max_y.cmp(&b.max_y))
            .unwrap()
            .max_y;

        let width = (max_x - min_x) + 1;
        let height = (max_y - min_y) + 1;

        self.width = width as u16;
        self.height = height as u16;
        self.min_x = min_x;
        self.max_x = max_x;
        self.min_y = min_y;
        self.max_y = max_y;

        println!("Remapping origin...");
        self.update_origin();
    }

    /// Updates the [LavaductLagoon] so that the origin is in the upper-left corner and all coordinates
    /// of all contained elements are positive.
    fn update_origin(&mut self) {
        let x_add = self.min_x.abs();
        let y_add = self.min_y.abs();
        for trench in &mut self.trench_segments {
            trench.shift(x_add, y_add);
        }

        self.min_x += x_add;
        self.max_x += x_add;
        self.min_y += y_add;
        self.max_y += y_add;
    }

    fn make_grid(&self) -> ColorGrid {
        let rows = self.height as usize;
        let cols = self.width as usize;
        let mut grid = Vec::with_capacity(rows);
        for _ in 0..rows {
            let mut row = Vec::with_capacity(cols);
            for _ in 0..cols {
                let color = Rc::new(RefCell::new(Color::from(0xFFFFFF)));
                row.push(color);
            }
            grid.push(row);
        }

        for trench in &self.trench_segments {
            let trench_color = &trench.color;
            if trench.start.0 == trench.end.0 {
                // trench is vertical
                let x = trench.start.0 as usize;
                let y_range = absolute_range(trench.start.1 as u32, trench.end.1 as u32);
                for y in y_range {
                    let mut target_color = grid[y as usize][x].borrow_mut();
                    *target_color = trench_color.clone();
                }
            }
            if trench.start.1 == trench.end.1 {
                // trench is horizontal
                let x_range = absolute_range(trench.start.0 as u32, trench.end.0 as u32);
                let y = trench.start.1 as usize;
                for x in x_range {
                    let mut target_color = grid[y][x as usize].borrow_mut();
                    *target_color = trench_color.clone();
                }
            }
        }

        grid
    }

    fn draw_to_image(&self) -> RgbImage {
        let mut img: RgbImage = ImageBuffer::from_pixel(
            self.width as u32,
            self.height as u32,
            Rgb([255u8, 255, 255]),
        );

        for trench in &self.trench_segments {
            let color: Rgb<u8> = Rgb::from(trench.color.clone());
            if trench.start.0 == trench.end.0 {
                // trench is vertical
                let x = trench.start.0 as u32;
                let y_range = absolute_range(trench.start.1 as u32, trench.end.1 as u32);
                for y in y_range {
                    img.put_pixel(x, y, color);
                }
            }
            if trench.start.1 == trench.end.1 {
                // trench is horizontal
                let x_range = absolute_range(trench.start.0 as u32, trench.end.0 as u32);
                let y = trench.start.1 as u32;
                for x in x_range {
                    img.put_pixel(x, y, color);
                }
            }
        }

        img
    }
}

type ColorGrid = Vec<Vec<Rc<RefCell<Color>>>>;

fn flood_fill(color_grid: &mut ColorGrid, fill_color: Color) {
    let width = color_grid[0].len();
    let height = color_grid.len();
    let start_x = width / 2;
    let start_y = height / 2;
    let inside_color = color_grid[start_y][start_x].borrow().clone();

    let mut fill_queue = VecDeque::new();
    {
        let node = color_grid[start_y][start_x].clone();
        fill_queue.push_front((node, (start_x, start_y)));
    }
    while !fill_queue.is_empty() {
        let (node, (node_x, node_y)) = fill_queue.pop_front().unwrap();
        let mut node = node.borrow_mut();
        if *node == inside_color {
            *node = fill_color.clone();
            {
                // North node
                let x = node_x;
                let (y, did_overflow) = node_y.overflowing_sub(1);
                if !did_overflow {
                    let node = color_grid[y][x].clone();
                    fill_queue.push_front((node, (x, y)));
                }
            }
            {
                // South node
                let x = node_x;
                let y = node_y + 1;
                if y < height {
                    let node = color_grid[y][x].clone();
                    fill_queue.push_front((node, (x, y)));
                }
            }
            {
                // East node
                let (x, did_overflow) = node_x.overflowing_sub(1);
                let y = node_y;
                if !did_overflow {
                    let node = color_grid[y][x].clone();
                    fill_queue.push_front((node, (x, y)));
                }
            }
            {
                // West node
                let x = node_x + 1;
                let y = node_y;
                if x < width {
                    let node = color_grid[y][x].clone();
                    fill_queue.push_front((node, (x, y)));
                }
            }
        }
    }
}

const TEST_INPUT: &str = "R 6 (#70c710)\n\
                          D 5 (#0dc571)\n\
                          L 2 (#5713f0)\n\
                          D 2 (#d2c081)\n\
                          R 2 (#59c680)\n\
                          D 2 (#411b91)\n\
                          L 5 (#8ceee2)\n\
                          U 2 (#caa173)\n\
                          L 1 (#1b58a2)\n\
                          U 2 (#caa171)\n\
                          R 2 (#7807d2)\n\
                          U 3 (#a77fa3)\n\
                          L 2 (#015232)\n\
                          U 2 (#7a21e3)";

fn main() {
    // let instructions = TEST_INPUT
    //     .lines()
    //     .filter_map(|line| DigInstruction::from_str(line).ok())
    //     .collect::<Vec<_>>();
    let instructions = fs::read_to_string("input.txt")
        .expect("failed to open input file")
        .lines()
        .filter_map(|line| DigInstruction::from_str(line).ok())
        .collect::<Vec<_>>();
    let mut lavaduct_lagoon = LavaductLagoon::default();

    lavaduct_lagoon.dig_trenches(&instructions);

    println!("Creating color grid...");
    let mut color_grid = lavaduct_lagoon.make_grid();
    println!("Filling lagoon...");
    flood_fill(&mut color_grid, Color::from(0xFF0000));

    println!("Writing image to file...");
    let out_img: RgbImage = ImageBuffer::from_fn(
        lavaduct_lagoon.width as u32,
        lavaduct_lagoon.height as u32,
        |x, y| {
            let color = color_grid[y as usize][x as usize].borrow().clone();
            Rgb::from(color)
        },
    );
    out_img
        .save("out.png")
        .expect("failed to write output image");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_color_from_u32() {
        let input = 0x00ABCDEFu32;
        let expected = Color {
            red: 0xAB,
            green: 0xCD,
            blue: 0xEF,
        };

        assert_eq!(Color::from(input), expected);
    }

    #[test]
    fn parse_color_from_str() {
        let input = "#abcdef";
        let expected = Color {
            red: 0xAB,
            green: 0xCD,
            blue: 0xEF,
        };

        assert_eq!(input.parse(), Ok(expected));

        assert_eq!("abcdef".parse::<Color>(), Err(ParseColorError));
    }

    #[test]
    fn parse_dig_instruction_from_str() {
        let input = "R 6 (#70c710)";
        let expected = DigInstruction {
            direction: Direction::Right,
            length: 6,
            color: Color {
                red: 0x70,
                green: 0xC7,
                blue: 0x10,
            },
        };

        assert_eq!(input.parse(), Ok(expected));

        assert_eq!(
            "R 6".parse::<DigInstruction>(),
            Err(ParseDigInstructionError)
        );
    }

    #[test]
    fn dig_trenches() {
        let instructions = TEST_INPUT
            .lines()
            .filter_map(|line| DigInstruction::from_str(line).ok())
            .collect::<Vec<_>>();
        let expected_width = 7;
        let expected_height = 10;
        let mut lavaduct_lagoon = LavaductLagoon::default();

        lavaduct_lagoon.dig_trenches(&instructions);
        assert_eq!(lavaduct_lagoon.width, expected_width);
        assert_eq!(lavaduct_lagoon.height, expected_height);
    }
}
