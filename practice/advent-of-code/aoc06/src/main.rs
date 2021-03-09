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

    let coordinates = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Coordinate>>>()?;
    if coordinates.is_empty() {
        return err!("no coordinates given");
    }

    let mut grid = Grid::new(coordinates);
    grid.find_finite();


    part1(&grid)?;
    part2(&grid)?;
    
    Ok(())
}

fn part1(grid: &Grid) -> Result<()>{
    let mut biggest_area = 0;
    for &loc in &grid.finite {
        let mut candidate_area = 0;
        for &loc2 in grid.table.values() {
            if loc == loc2 {
                candidate_area += 1;
            }
        }
        biggest_area = biggest_area.max(candidate_area);
    }

    writeln!(io::stdout(), "the size of the largest area that isn't infinite: {}", biggest_area)?;
    
    Ok(())
}


fn part2(grid: &Grid) -> Result<()>{
    let bound = 500;
    let mut size = 0;

    for x in -bound..=bound {
        for y in -bound..=bound {
            if grid.distance_sum(Coordinate { x, y }) < 10000 {
                size += 1
            }
        }
    }

    writeln!(io::stdout(), "the size of the region containing all locations which have a total distance to all given coordinates of less than 10000: {}", size)?;
    
    Ok(())
}

#[derive(Debug)]
struct Grid {
    locations: Vec<Coordinate>,
    finite: HashSet<Coordinate>,
    table: HashMap<Coordinate, Coordinate>,
}

impl Grid {
    fn new(locations: Vec<Coordinate>) -> Self {
        assert!(!locations.is_empty());
        Grid { locations, finite: HashSet::new(), table: HashMap::new() }
    }

    fn find_finite(&mut self) {
        for step in 0..100 {
            for loc in &self.locations {
                if self.finite.contains(&loc) {
                    continue;
                }
                for c in loc.border(step) {
                    let closest = match self.closest_location(c) {
                        None => continue,
                        Some(closest) => closest,
                    };
                    self.table.insert(c, closest);
                }
            }
            for &loc in &self.locations {
                if loc.border(step).all(|c| self.table.get(&c) != Some(&loc)) {
                    self.finite.insert(loc);
                }
            }
        }
    }

    fn distance_sum(&self, c: Coordinate) -> i32 {
        self.locations.iter().map(|&loc| loc.manhattan_distance(c)).sum()
    }

    fn closest_location(&self, c: Coordinate) -> Option<Coordinate> {
        let (mut min, mut unique) = (self.locations[0], true);
        for &loc in &self.locations[1..] {
            if loc.manhattan_distance(c) == min.manhattan_distance(c) {
                unique = false;
            } else if loc.manhattan_distance(c) < min.manhattan_distance(c) {
                min = loc;
                unique = true;
            }
        }
        if !unique {
            None
        } else {
            Some(min)
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
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

impl Coordinate {
    fn manhattan_distance(&self, other: Coordinate) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn border(self, step: i32) -> impl Iterator<Item=Coordinate> {
        (self.x - step..=self.x + step)
            .flat_map(move |x| {
                (self.y - step..=self.y + step)
                .map(move |y| Coordinate { x, y })
            })
            .filter(move |&c2| self.manhattan_distance(c2) == step)
    }
}
