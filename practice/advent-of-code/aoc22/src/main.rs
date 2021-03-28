use std::io::{self, Read, Write};
use std::error::Error;
use std::result;
use std::str::FromStr;
use std::collections::BTreeMap;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    let mut cave: Cave = Cave::new(&input)?;
    
    part1(&cave)?;


    Ok(())
}

fn part1(cave: &Cave) -> Result<()> {
    let mut risk_level = 0;
    for x in 0..=cave.target.x {
        for y in 0..=cave.target.y {
            if x == cave.target.x && y == cave.target.y {
                continue;
            }
            if let Some(region) = cave.regions.get(&Coordinate { x, y }) {
                match region {
                    Region::Rocky => (),
                    Region::Narrow => risk_level += 2,
                    Region::Wet => risk_level += 1,
                }
            }
        }
    }
    writeln!(io::stdout(), "part1 answer: {}", risk_level)?;
    Ok(())
}

#[derive(Debug)]
struct Cave {
    depth: u32,
    target: Coordinate,
    regions: BTreeMap<Coordinate, Region>,
    erosion: BTreeMap<Coordinate, u32>,
    geologic: BTreeMap<Coordinate, u32>,
}

impl Cave {
    fn new(s: &str) -> Result<Self> {
        let mut cave: Cave;
        let lines: Vec<&str> = s.lines().collect();
        if let Some(depth) = lines[0].trim().strip_prefix("depth: ") {
            if let Some(target) = lines[1].trim().strip_prefix("target: ") {
                cave = Cave {
                    depth: depth.parse()?,
                    target: target.parse()?,
                    regions: BTreeMap::new(),
                    erosion: BTreeMap::new(),
                    geologic: BTreeMap::new(),
                };
                cave.init()?;
                return Ok(cave);
            } else {
                return err!("can not parse target: {}", lines[1]);
            }
        } else {
            return err!("can not parse depth: {}", lines[0]);
        }
    }

    fn init(&mut self) -> Result<()> {
        self.geologic.insert(Coordinate::new(0, 0), 0);
        self.erosion.insert(Coordinate::new(0, 0), (0 + self.depth) % 20183);
        self.geologic.insert(self.target, 0);
        self.erosion.insert(self.target, (0 + self.depth) % 20183);
        let max_x = self.target.x + 5;
        let max_y = self.target.y + 5;
        for x in 0..max_x {
            self.calc_geologic(&Coordinate::new(x, 0))?;
            self.calc_erosion(&Coordinate::new(x, 0))?;
        }
        for y in 0..max_y {
            self.calc_geologic(&Coordinate::new(0, y))?;
            self.calc_erosion(&Coordinate::new(0, y))?;
        }
        for x in 1..max_x {
            for y in 1..max_y {
                self.calc_geologic(&Coordinate::new(x, y))?;
                self.calc_erosion(&Coordinate::new(x, y))?;
            }
        }
        for (&c, e) in &self.erosion {
            match e % 3 {
                0 => self.regions.insert(c, Region::Rocky),
                1 => self.regions.insert(c, Region::Wet),
                2 => self.regions.insert(c, Region::Narrow),
                _ => return err!("wrong region kind, ersion: {}", e),
            };
        }
        Ok(())
    }

    fn calc_geologic(&mut self, c: &Coordinate) -> Result<()> {
        let geologic = if c.x == 0 {
            c.y * 48271
        } else if c.y == 0 {
            c.x * 16807
        } else {
            self.erosion.get(&Coordinate { x: c.x - 1, ..c.clone()}).unwrap()
                * self.erosion.get(&Coordinate { y: c.y - 1, ..c.clone()}).unwrap()
        };
        self.geologic.insert(c.clone(), geologic);
        Ok(())
    }

    fn calc_erosion(&mut self, c: &Coordinate) -> Result<()> {
        let erosion = if let Some(geologic) = self.geologic.get(c) {
            (geologic + self.depth) % 20183
        } else {
            return err!("can not find coordinat {:?} on the grid", c)
        };
        self.erosion.insert(c.clone(), erosion);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Region {
    Rocky,
    Narrow,
    Wet
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coordinate {
    x: u32,
    y: u32,
}

impl Coordinate {
    fn new(x: u32, y: u32) -> Self {
        Self{ x, y }
    }
}

impl FromStr for Coordinate {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {

        let coord: Vec<&str> = s.split(",").collect();
        if coord.len() != 2 {
            return err!("wrong coordinate")
        } else {
            Ok(Coordinate { x: coord[0].parse()?, y: coord[1].parse()? })
        }
    }
}