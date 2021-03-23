use std::{fmt::write, io::{self, Read, Write}};
use std::error::Error;
use std::result;
use std::fmt;
use std::str::FromStr;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::iter::FromIterator;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut caves: Caves = input.parse()?;

    writeln!(io::stdout(), "{}", caves)?;

    Ok(())
}

#[derive(Debug, Default)]
struct Caves {
    grid: BTreeMap<Coordinate, Cell>,
    units: BTreeMap<Coordinate, Unit>,
    max: Coordinate,
}

impl FromStr for Caves {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        if !s.is_ascii() {
            return err!("only ASCII caves are supportes");
        }
        let mut caves = Caves::default();
        caves.max.x = s.lines().next().unwrap_or("").len() - 1;
        caves.max.y = s.lines().count() - 1;
        for (y, line) in s.lines().enumerate() {
            for (x, _) in line.char_indices() {
                let c = Coordinate { x, y };
                let cell = &line[x..x+1];
                if ["E", "G"].contains(&cell) {
                    let unit: Unit = cell.parse()?;
                    caves.grid.insert(c, Cell::Open);
                    caves.units.insert(c, unit);
                } else {
                    caves.grid.insert(c, cell.parse()?);
                }
            }
        }

        Ok(caves)
    }
}

impl fmt::Display for Caves {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (c, cell) in &self.grid {
            if let Some(ref unit) = self.units.get(c) {
                write!(f, "{}", unit)?;
            } else {
                write!(f, "{}", cell)?;
            }
            if c.x == self.max.x {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}



#[derive(Clone, Copy, Debug)]
enum Cell {
    Open,
    Wall,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Open => write!(f, "."),
            Cell::Wall => write!(f, "#")
        }
    }
}

impl FromStr for Cell {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        match s.as_bytes().get(0) {
            Some(&b'#') => Ok(Cell::Wall),
            Some(&b'.') => Ok(Cell::Open),
            Some(&b) => err!("unrecognized cell: 0x{:x}", b),
            None => err!("cannot deserialize empty string into cell"),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize, 
    y: usize
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.y, self.x).cmp(&(other.y, other.x)))
    }
}

#[derive(Clone, Copy, Debug)]
enum UnitKind {
    Elf,
    Goblin,
}

#[derive(Debug)]
struct Unit {
    attack: i32,
    hp: i32,
    kind: UnitKind,
}

impl FromStr for Unit {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let kind = match s.as_bytes().get(0) {
            Some(&b'E') => UnitKind::Elf,
            Some(&b'G') => UnitKind::Goblin,
            Some(&b) => return err!("unrecognized unit kind: 0x{:x}", b),
            None => return err!("cannot deserialize empty string into unit"),
        };
        Ok(Self{ attack: 3, hp: 200, kind})
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            UnitKind::Elf => write!(f, "E"),
            UnitKind::Goblin => write!(f, "G"),
        }
    }
}
