use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::fs;
use std::rc::Rc;

type TilePtr = Rc<RefCell<MazeTile>>;

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Eq, PartialEq)]
struct Connection {
    direction: Direction,
    tile: TilePtr,
}

#[derive(Debug, Default, Eq, PartialEq)]
struct Pipe {
    connection_0: Option<Connection>,
    connection_1: Option<Connection>,
    is_start: bool,
}

#[derive(Debug, PartialEq, Eq)]
enum MazeTile {
    Vertical(Pipe),
    Horizontal(Pipe),
    NorthEastBend(Pipe),
    NorthWestBend(Pipe),
    SouthWestBend(Pipe),
    SouthEastBend(Pipe),
    Ground,
    Start,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseMazeTileError;

impl TryFrom<char> for MazeTile {
    type Error = ParseMazeTileError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Self::Vertical(Pipe::default())),
            '-' => Ok(Self::Horizontal(Pipe::default())),
            'L' => Ok(Self::NorthEastBend(Pipe::default())),
            'J' => Ok(Self::NorthWestBend(Pipe::default())),
            '7' => Ok(Self::SouthWestBend(Pipe::default())),
            'F' => Ok(Self::SouthEastBend(Pipe::default())),
            '.' => Ok(Self::Ground),
            'S' => Ok(Self::Start),
            _ => Err(ParseMazeTileError),
        }
    }
}

impl Display for MazeTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            MazeTile::Vertical(_) => '┃',
            MazeTile::Horizontal(_) => '━',
            MazeTile::NorthEastBend(_) => '┗',
            MazeTile::NorthWestBend(_) => '┛',
            MazeTile::SouthWestBend(_) => '┓',
            MazeTile::SouthEastBend(_) => '┏',
            MazeTile::Ground => '░',
            MazeTile::Start => 'S',
        };

        write!(f, "{symbol}")
    }
}

fn connect_pipes(tiles: &mut [[TilePtr]]) {
    for (i, row) in tiles.iter_mut().enumerate() {}
}

const TEST_INPUT: &str = ".....\n\
.F-7.\n\
.|.|.\n\
.L-J.\n\
.....";

fn main() {
    let tiles: Vec<Vec<TilePtr>> = fs::read_to_string("input.txt")
        .expect("failed to open input file")
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.try_into().ok())
                .map(|tile| Rc::new(RefCell::new(tile)))
                .collect()
        })
        .collect();

    let mut out_str = String::new();
    for row in tiles {
        for tile in row {
            out_str.push_str(&*format!("{tile}"));
        }
        out_str.push('\n');
    }

    print!("{out_str}");
}
