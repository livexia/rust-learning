#[macro_use]
extern crate lazy_static;

use std::io::{self, Read, Write};
use std::error::Error;
use std::result;
use std::fmt;
use std::str::FromStr;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::iter::FromIterator;

use regex::Regex;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut nanobots: Vec<Nanobot> = input.lines().
        map(|l| l.parse()).collect::<Result<Vec<Nanobot>>>()?;

    nanobots.sort_by(|a, b| b.radius.cmp(&a.radius));

    part1(&nanobots)?;
    part2(&nanobots)?;

    Ok(())
}

fn part1(nanobots: &Vec<Nanobot>) -> Result<()> {
    let mut sum = 0;
    let strongest_nanobot = &nanobots[0];
    for nanobot in nanobots {
        if strongest_nanobot.in_range(&nanobot.pos) {
            sum += 1;
        }
    }
    writeln!(io::stdout(), "part1 answer: {}", sum)?;
    Ok(())
}

fn part2(nanobots: &Vec<Nanobot>) -> Result<()> {
    let my_pos = Position{ x: 0, y: 0, z: 0 };
    let (mut min_x, mut min_y, mut min_z, mut max_x, mut max_y, mut max_z) = (0, 0, 0, 0 ,0 ,0);
    for bot in nanobots {
        min_x = min_x.min(bot.pos.x);
        min_y = min_y.min(bot.pos.y);
        min_z = min_z.min(bot.pos.z);
        max_x = max_x.max(bot.pos.x);
        max_y = max_y.max(bot.pos.y);
        max_z = max_z.max(bot.pos.z);
    }

    let max_nanobot = &nanobots[0];
    println!("{:?}", max_nanobot);


    min_x = max_nanobot.pos.x - max_nanobot.radius;
    min_y = max_nanobot.pos.y - max_nanobot.radius;
    min_z = max_nanobot.pos.z - max_nanobot.radius;
    max_x = max_nanobot.pos.x + max_nanobot.radius;
    max_y = max_nanobot.pos.y + max_nanobot.radius;
    max_z = max_nanobot.pos.z + max_nanobot.radius;

    let mut max = 0;
    for x in min_x..max_x {
        for y in min_y..max_y {
            for z in min_z..max_z {
                let mut sum = 0;
                let pos = Position { x, y, z };
                if !max_nanobot.in_range(&pos) {
                    continue;
                }
                for bot in nanobots {
                    if bot.in_range(&pos) {
                        sum += 1;
                    }
                }
                if max < sum {
                    println!("{:?}", pos);
                    max = sum;
                }
            }
        }
    }    
    writeln!(io::stdout(), "part2 answer: {}", max)?;
    Ok(())
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    fn distance(&self, other: &Self) -> i32 {
        let mut d = (self.x - other.x).abs();
        d += (self.y - other.y).abs();
        d += (self.z - other.z).abs();
        d
    }
}

#[derive(Debug)]
struct Nanobot {
    pos: Position,
    radius: i32,
}

impl Nanobot {
    fn in_range(&self, pos: &Position) -> bool {
        if self.pos.distance(pos) <= self.radius {
            true
        } else {
            false
        }
    }
}

impl FromStr for Nanobot {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x)
                pos=<(?P<x>[-0-9]+),(?P<y>[-0-9]+),(?P<z>[-0-9]+)>,\sr=(?P<r>[0-9]+)
            ").unwrap();
        }
        let caps = match RE.captures(s) {
            None => return err!("unrecognized nannobot"),
            Some(caps) => caps,
        };
        let pos = Position {
            x: caps["x"].parse()?,
            y: caps["y"].parse()?,
            z: caps["z"].parse()?,
        };
        Ok(Nanobot { pos, radius: caps["r"].parse()? })
    }
}
