use std::io::{self, Read, Write};
use std::error::Error;
use std::fmt;
use std::result;
use std::str::FromStr;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::collections::BinaryHeap;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    let cave: Cave = Cave::new(&input)?;
    
    part1(&cave)?;
    part2(&cave)?;

    Ok(())
}

fn part1(cave: &Cave) -> Result<()> {
    let mut risk_level = 0;
    for x in 0..=cave.target.x {
        for y in 0..=cave.target.y {
            if x == cave.target.x && y == cave.target.y {
                continue;
            }
            match cave.regions[x][y] {
                Region::Rocky => (),
                Region::Narrow => risk_level += 2,
                Region::Wet => risk_level += 1,
            }
        }
    }
    writeln!(io::stdout(), "part1 answer: {}", risk_level)?;
    Ok(())
}

fn part2(cave: &Cave) -> Result<()> {
    writeln!(io::stdout(), "part2 answer: {}", cave.shortest_time()?)?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Tool {
    Torch,
    ClimbingGear,
    Neither,
}

#[derive(Debug)]
struct Cave {
    depth: u64,
    target: Coordinate,
    bound: Coordinate,
    regions: Vec<Vec<Region>>,
    erosion: Vec<Vec<u64>>,
    geologic: Vec<Vec<u64>>,
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
                    bound: target.parse()?,
                    regions: Vec::new(),
                    erosion: Vec::new(),
                    geologic: Vec::new(),
                };
                cave.bound.x *= 15;
                cave.bound.y *= 15;
                cave.regions = vec![vec![Region::Rocky; cave.bound.y + 1]; cave.bound.x + 1];
                cave.erosion = vec![vec![0; cave.bound.y + 1]; cave.bound.x + 1];
                cave.geologic = vec![vec![0; cave.bound.y + 1]; cave.bound.x + 1];
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
        self.geologic[0][0] = 0;
        self.erosion[0][0] = self.depth % 20183;
        self.geologic[self.target.x][self.target.y] = 0;
        self.erosion[self.target.x][self.target.y] = self.depth % 20183;
        for x in 0..=self.bound.x {
            self.calc_geologic(x, 0)?;
            self.calc_erosion(x, 0)?;
        }
        for y in 0..=self.bound.y {
            self.calc_geologic(0, y)?;
            self.calc_erosion(0, y)?;
        }
        for x in 1..=self.bound.x {
            for y in 1..=self.bound.y {
                self.calc_geologic(x ,y)?;
                self.calc_erosion(x, y)?;
            }
        }
        for x in 0..=self.bound.x {
            for y in 0..=self.bound.y {
                self.regions[x][y] = match self.erosion[x][y] % 3 {
                    0 => Region::Rocky,
                    1 => Region::Wet,
                    2 => Region::Narrow,
                    _ => return err!("wrong region kind, ersion: {}", self.erosion[x][y]),
                };
            }
        }
        Ok(())
    }

    fn calc_geologic(&mut self, x: usize, y: usize) -> Result<()> {
        self.geologic[x][y] = if x == 0 {
            y as u64 * 48271
        } else if y == 0 {
            x  as u64 * 16807
        } else {
            self.erosion[x - 1][y] * self.erosion[x][y - 1]
        };
        Ok(())
    }

    fn calc_erosion(&mut self, x: usize, y: usize) -> Result<()> {
        self.erosion[x][y] = (self.geologic[x][y] + self.depth) % 20183;
        Ok(())
    }

    fn string(&self, me: &Coordinate) -> String {
        let mut result = String::new();
        for y in 0..=self.bound.y {
            for x in 0..=self.bound.x {
                let cur = Coordinate::new(x, y);
                if self.target == cur {
                    result.push_str("T");
                } else if x == 0 && y == 0 {
                    result.push_str("M");
                } else if me == &cur {
                    result.push_str("X");
                } else {
                    result.push_str(&format!("{}", self.regions[x][y]));
                }
            }
            result.push_str("\n");
        }
        result
    }

    fn shortest_time(&self) -> Result<usize> {
        type Time = usize;
        type PriorityQueue = BinaryHeap<Reverse<(Time, Coordinate, Tool)>>;
        let mut queue: PriorityQueue = BinaryHeap::new();
        let mut best: HashMap<(Coordinate, Tool), Time> = HashMap::new();

        queue.push(Reverse((0, Coordinate::new(0, 0), Tool::Torch)));
        while let Some(Reverse((time, c, tool))) = queue.pop() {
            if best.contains_key(&(c, tool)) && best[&(c, tool)] <= time {
                continue;
            }
            best.insert((c, tool), time);
            if c == self.target && tool == Tool::Torch {
                return Ok(time)
            }
            
            for &e in &[Tool::Torch, Tool::ClimbingGear, Tool::Neither] {
                if self.regions[c.x][c.y].can_equip(e) {
                    queue.push(Reverse((time + 7, c, e)))
                }
            }
            for &(x, y) in &[(0, -1), (1, 0), (0, 1), (-1, 0)] {
                if (x < 0 && c.x == 0) || (y <= 0 && c.y == 0) {
                    continue;
                }

                let x = (c.x as i64 + x) as usize;
                let y = (c.y as i64 + y) as usize;
                if x > self.bound.x || y > self.bound.y {
                    continue;
                }
                if self.regions[x][y].can_equip(tool) {
                    let neighbor = Coordinate::new(x, y);
                    queue.push(Reverse((time + 1, neighbor, tool)));
                }
            }
        }
        err!("could not find a path to {:?}", self.target)
    }
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string(&Coordinate::new(0, 0)))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Region {
    Rocky,
    Narrow,
    Wet
}

impl Region {
    fn can_equip(&self, tool: Tool) -> bool {
        use crate::Region::*;
        use crate::Tool::*;

        match (self, tool) {
            (Rocky, Torch) | (Rocky, ClimbingGear) => true,
            (Wet, ClimbingGear) | (Wet, Neither) => true,
            (Narrow, Torch) | (Narrow, Neither) => true,
            _ => false
        }
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Region::Rocky => write!(f, ".")?,
            Region::Narrow => write!(f, "|")?,
            Region::Wet => write!(f, "=")?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
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