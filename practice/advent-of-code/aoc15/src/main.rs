#[cfg(test)]
mod test;

use std::io::{self, Read, Write};
use std::error::Error;
use std::result;
use std::fmt;
use std::str::FromStr;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::HashSet;
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

impl Caves {
    fn step(&mut self) {
        todo!()
    }

    fn is_wall(&self, c: &Coordinate) -> bool {
        if let Some(c) = self.grid.get(&c) {
            c == &Cell::Wall
        } else {
            true
        }
    }

    fn is_open(&self, c: &Coordinate) -> bool {
        !self.is_wall(c)
    }

    fn near(&self, c: &Coordinate) -> Vec<Coordinate> {
        let mut near = vec![];
        let x = c.x;
        let y = c.y;
        if c.x < self.max.x {
            near.push(Coordinate { x: x + 1, y});
        }
        if c.y < self.max.y {
            near.push(Coordinate { x, y: y + 1 });
        }
        if c.y > 0 {
            near.push(Coordinate { x, y: y - 1 });
        }
        if c.x > 0 {
            near.push(Coordinate { x: x - 1, y});
        }
        near.into_iter().filter(|c| self.is_open(c)).collect()
    }

    fn any_targets(&self, c: &Coordinate) -> bool {
        let u = &self.units[c];
        for unit in self.units.values() {
            if u.is_target(&unit) {
                return true
            }
        }
        false
    }
    
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
        let mut units = vec![];
        for (c, cell) in &self.grid {
            if let Some(ref unit) = self.units.get(c) {
                write!(f, "{}", unit)?;
                units.push(format!("{:?}", unit));
            } else {
                write!(f, "{}", cell)?;
            }
            if c.x == self.max.x {
                write!(f, "    {}", units.join(", "))?;
                units.clear();
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}




#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl Coordinate {
    fn is_adjacent(&self, other: &Coordinate) -> bool {
        match self.distance(other) {
            1 => true,
            _ => false,
        }
    }

    fn distance(&self, other: &Coordinate) -> usize {
        let dx = (self.x as i32 - other.x as i32).abs() as usize;
        let dy = (self.y as i32 - other.y as i32).abs() as usize;
        dx + dy
    }
    
    fn up(&self) -> Self {
        Self { x: self.x - 1, y: self.y }
    }
    
    fn down(&self) -> Self {
        Self { x: self.x + 1, y: self.y }
    }
    
    fn left(&self) -> Self {
        Self { x: self.x, y: self.y - 1 }
    }
    
    fn right(&self) -> Self {
        Self { x: self.x, y: self.y + 1 }
    }
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

#[derive(Clone)]
struct Unit {
    attack: i32,
    hp: i32,
    kind: UnitKind,
}

impl Unit {
    fn is_target(&self, other: &Unit) -> bool {
        use crate::UnitKind::*;
        match (self.kind, other.kind) {
            (Elf, Goblin) => true,
            (Goblin, Elf) => true,
            _ => false
        }
    }

    fn attack(&mut self, other: &mut Unit) {
        other.hp -= self.attack;
    }

    fn is_dead(&mut self) -> bool {
        if self.hp <= 0 {
            true
        } else {
            false
        }
    }
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

impl fmt::Debug for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            UnitKind::Elf => write!(f, "E({})", self.hp),
            UnitKind::Goblin => write!(f, "G({})", self.hp),
        }
    }
}
