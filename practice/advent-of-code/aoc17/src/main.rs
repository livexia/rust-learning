use std::io::{self, Read, Write};
use std::error::Error;
use std::result;
use std::fmt;
use std::str::FromStr;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::iter::FromIterator;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    let mut ground: Groud = input.parse()?;

    ground.flow()?;
    // println!("{}", ground);
    println!("part1, reachable tiles: {}", ground.reach_able());
    println!("part2, remaining water: {}", ground.remain_water());

    Ok(())
}

impl Groud {
    fn reach_able(&self) -> i32 {
        self.grid.iter()
            .filter(|(&c, _)| !self.is_illegal(c) && c.y <= self.y_range.1 && c.y >= self.y_range.0 )
            .fold(0, |sum, (_, &k)| {
                if k == Kind::FlowWater || k == Kind::RestWater {
                    sum + 1
                } else {
                    sum
                }
            })
    }
    fn remain_water(&self) -> i32 {
        self.grid.iter()
            .filter(|(&c, _)| !self.is_illegal(c) && c.y <= self.y_range.1 && c.y >= self.y_range.0 )
            .fold(0, |sum, (_, &k)| {
                if k == Kind::RestWater {
                    sum + 1
                } else {
                    sum
                }
            })
    }

    fn flow(&mut self) -> Result<()> {
        use crate::Kind::*;
       
        let source = Coordinate::new(500, 0);
        let mut down: Vec<Coordinate> = Vec::new();
        let mut vertical: Vec<Coordinate> = vec![];
        match self.get(&source.down()) {
            Sand => {
                self.grid.insert(source.down(), FlowWater);
                down.push(source.down())
            },
            _ => return err!("water no where to go"),
        }
        let mut count = 1;
        while !down.is_empty() || !vertical.is_empty() {
            if let Some(cur) = down.pop() {
                if self.is_illegal(cur) {
                    continue;
                }
                match self.get(&cur.down()) {
                    Sand => {
                        if self.is_illegal(cur.down()) {
                            continue;
                        }
                        self.grid.insert(cur.down(), FlowWater);
                        down.push(cur.down())
                    },
                    Clay | RestWater => vertical.push(cur),
                    FlowWater => continue,
                    _ => {
                        println!("{}\n", self);
                        println!("cur: {:?}, cur_cown: {:?}", cur, cur.down());
                        println!("down: {:?}\nvertical: {:?}", down, vertical);
                        return err!("no where to go");
                    },
                }
            }
            if let Some(cur) = vertical.pop() {
                self.make_rest_water(cur, &mut down, &mut vertical);
            }
            count += 1;
            if count % 1000000 == 0 {
                // println!("{}\n", self);
                println!("down: {:?}\nvertical: {:?}", down, vertical);
                println!("water: {:?}", self.reach_able());
            }
        }
        Ok(())

    }

    fn make_rest_water(&mut self, start: Coordinate, down: &mut Vec<Coordinate>, vertical: &mut Vec<Coordinate>) {
        use crate::Kind::*;

        let mut open = vec![];
        let mut left_open = false;
        let mut right_open = false;
        let mut cur = start; 
        loop {
            let left = cur.left();
            let left_down = left.down();
            if self.is_sand(left) || self.is_flow_water(left) {
                cur = left;
                open.push(left);
            } else {
                break;
            }
            if self.is_sand(left_down) || self.is_flow_water(left_down) {
                left_open = true;
                break;
            }
        }
        open.reverse();
        open.push(start);
        let mut cur = start; 
        loop {
            let right = cur.right();
            let right_down = right.down();
            if self.is_sand(right) || self.is_flow_water(right) {
                cur = right;
                open.push(right);
            } else {
                break;
            }
            if self.is_sand(right_down) || self.is_flow_water(right_down) {
                right_open = true;
                break;
            }
        }
        let n = open.len();
        if left_open {
            down.push(open[0]);
        }
        if right_open {
            down.push(open[n-1]);
        }
        if !left_open && !right_open {
            vertical.push(start.up());
            open.iter()
                .for_each(|&c| { self.grid.insert(c, RestWater); });
        } else {
            open.iter()
                .for_each(|&c| { self.grid.insert(c, FlowWater); });
        }
    }

    fn is_sand(&self, coord: Coordinate) -> bool {
        self.get(&coord) == Kind::Sand
    }

    fn is_flow_water(&self, coord: Coordinate) -> bool {
        self.get(&coord) == Kind::FlowWater
    }

    fn get(&self, coord: &Coordinate) -> Kind {
        if let Some(&kind) = self.grid.get(coord) {
            kind
        } else {
            Kind::Sand
        }
    }

    fn is_illegal(&self, coord: Coordinate) -> bool {
        if coord.x < self.min_coord.x || coord.y < self.min_coord.y {
            true
        } else if coord.x > self.max_coord.x || coord.y > self.max_coord.y {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct Groud {
    min_coord: Coordinate,
    max_coord: Coordinate,
    y_range: (i32, i32),
    grid: BTreeMap<Coordinate, Kind>
}

impl fmt::Display for Groud {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let start_x = self.min_coord.x;
        let start_y = self.min_coord.y;
        let end_x = self.max_coord.x;
        let end_y = self.max_coord.y;
        for y in start_y..=end_y {
            write!(f, "{:4} ", y)?;
            for x in start_x..=end_x {
                if let Some(kind) = self.grid.get(&Coordinate::new(x, y)) {
                    match kind {
                        Kind::Clay => write!(f, "#")?,
                        Kind::Sand => (),
                        Kind::Source => write!(f, "+")?,
                        Kind::RestWater => write!(f, "~")?,
                        Kind::FlowWater => write!(f, "|")?,
                    }
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Groud {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut min_coord = Coordinate::new(500, 0);
        let mut max_coord = Coordinate::new(500, 0);
        let mut grid = BTreeMap::new();
        for line in s.lines() {
            let parts: Vec<&str> = line.trim().split(", ").collect();
            if parts.len() != 2 {
                return err!("input error: {}", line);
            }
            let mut part1 = parts[0].split("=");
            let fix = part1.next().unwrap();
            let value: i32 = part1.next().unwrap().parse()?;
            let mut part2 = parts[1].split("=").skip(1);
            let range: Vec<&str> = part2.next().unwrap()
                .split("..")
                .collect();
            let start = range[0].parse()?;
            let end = range[1].parse()?;
            match fix {
                "x" => {
                    min_coord.x = min_coord.x.min(value - 2);
                    max_coord.x = max_coord.x.max(value + 2);
                    min_coord.y = min_coord.y.min(start);
                    max_coord.y = max_coord.y.max(end);
                    for y in start..=end {
                        grid.insert(Coordinate::new(value, y), Kind::Clay);
                    }
                },
                "y" => {
                    min_coord.x = min_coord.x.min(start - 2);
                    max_coord.x = max_coord.x.max(end + 2);
                    min_coord.y = min_coord.y.min(value);
                    max_coord.y = max_coord.y.max(value);
                    for x in start..=end {
                        grid.insert(Coordinate::new(x, value), Kind::Clay);
                    }
                }
                _ => ()
            }
        }
        let min_y = grid.iter()
            .map(|(c,_)| c.y).min().unwrap();
        let max_y = grid.iter()
                .map(|(c,_)| c.y).max().unwrap();
        let y_range = (min_y, max_y);
        let source = Coordinate::new(500, 0);
        grid.insert(source, Kind::Source);
        Ok(Self { min_coord, max_coord, y_range, grid })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coordinate {
    x: i32,
    y: i32
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Self{ x, y }
    }

    fn up(&self) -> Self {
        Self { x: self.x, y: self.y - 1 }
    }

    fn down(&self) -> Self {
        Self { x: self.x, y: self.y + 1 }
    }

    fn left(&self) -> Self {
        Self { x: self.x - 1, y: self.y }
    }

    fn right(&self) -> Self {
        Self { x: self.x + 1, y: self.y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Clay,
    Sand,
    Source,
    RestWater,
    FlowWater
}