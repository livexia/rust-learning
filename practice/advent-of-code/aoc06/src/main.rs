#[macro_use]
extern crate lazy_static;

use std::hash::Hash;
use std::io::{self, Read, Write};
use std::error::Error;
use std::result;
use std::str::FromStr;
use std::cmp::Eq;
use std::collections::HashMap;
use std::collections::HashSet;

use regex::Regex;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn Error>>;



fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut coordinates: Vec<Coordinate> = vec![];
    for line in input.lines() {
        let coordinate = line.parse().or_else(|err| {
            err!("failed to parse '{:?}': {}", line, err)
        })?;
        coordinates.push(coordinate);
    }
    coordinates.sort_by(|c1, c2| (c1.x+c1.y).cmp(&(c2.x + c2.y)));

    let m = (coordinates.iter().max_by_key(| &c | c.x).unwrap().x + 1) as usize;
    let n = (coordinates.iter().max_by_key(| &c | c.y).unwrap().y + 1) as usize;
    
    part1(&coordinates, m, n)?;
    part2(&coordinates, m, n)?;
    
    Ok(())
}

fn part1(coordinates: &Vec<Coordinate>, m: usize, n: usize) -> Result<()>{
    let mut grid = vec![vec![(None, std::u32::MAX); n]; m];

    for c in coordinates {
        grid[c.x as usize][c.y as usize] = (Some(c), 0u32);
    }
    for i in 0..m {
        for j in 0..n {
            for c in coordinates {
                if grid[i][j].1 == 0 {
                    continue;
                }
                let coordinate = Coordinate { x: i as i32, y: j as i32};
                let distance = c.manhattan_distance(coordinate) as u32;
                if distance == grid[i][j].1 {
                    grid[i][j].0 = None;
                } else if distance < grid[i][j].1 {
                    grid[i][j] = (Some(c), distance);
                }
            }
        }
    }

    let mut area: HashMap<&Coordinate, u32> = HashMap::new();
    let mut infinite_coordinate: HashSet<&Coordinate> = HashSet::new();
    
    for i in 0..m {
        for j in 0..n {
            if i == 0 || j == 0 || i == m - 1 || j == n -1 {
                if let Some(coordinate) = grid[i][j].0 {
                    infinite_coordinate.insert(coordinate);
                }
            }
        }
    }
    for i in 0..m {
        for j in 0..n {
            if let Some(coordinate) = grid[i][j].0 {
                if !infinite_coordinate.contains(coordinate) {
                    *area.entry(coordinate).or_insert(0) += 1;
                }
            }
        }
    }

    let max_area = area.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    writeln!(io::stdout(), "the size of the largest area that isn't infinite: {}", max_area.1)?;
    
    Ok(())
}


fn part2(coordinates: &Vec<Coordinate>, m: usize, n: usize) -> Result<()>{
    let mut grid = vec![vec![0u32; n]; m];

    let mut size = 0u32;
    for i in 0..m {
        for j in 0..n {
            for c in coordinates {
                let coordinate = Coordinate { x: i as i32, y: j as i32};
                grid[i][j] += c.manhattan_distance(coordinate) as u32;
            }
            if grid[i][j] < 10000 {
                size += 1;
            }
        }
    }

    writeln!(io::stdout(), "the size of the region containing all locations which have a total distance to all given coordinates of less than 10000: {}", size)?;
    
    Ok(())
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Coordinate>>
}

#[derive(Debug, Hash, Eq)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl FromStr for Coordinate {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Coordinate> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x)
                (?P<x>[0-9]+),\s(?P<y>[0-9]+)
            ").unwrap();
        }
        let caps = match RE.captures(s) {
            None => return err!("unrecongnized cordinate"),
            Some(caps) => caps,
        };
        Ok(Coordinate {
            x: caps["x"].parse()?,
            y: caps["y"].parse()?,
        })
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Coordinate {
    fn manhattan_distance(&self, coordinate: Coordinate) -> i32 {
        (self.x - coordinate.x).abs() + (self.y - coordinate.y).abs()
    }
}
