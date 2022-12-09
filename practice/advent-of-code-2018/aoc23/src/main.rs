#[macro_use]
extern crate lazy_static;

use std::io::{self, Read, Write};
use std::error::Error;
use std::result;
use std::str::FromStr;
use std::collections::BinaryHeap;

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

    // let mut queue: BinaryHeap<(u64, i32)> = BinaryHeap::new();
    let mut queue = vec![];
    for bot in nanobots {
        let d: u64 = vec![bot.pos.x, bot.pos.y, bot.pos.z].iter().map(|x| x.abs() as u64).sum();
        queue.push((d.saturating_sub(bot.radius), 1));
        queue.push((d + bot.radius + 1, -1));
    }
    queue.sort();
    queue.reverse();
    let mut count = 0;
    let mut max_count = 0;
    let mut answer = 0;
    while !queue.is_empty() {
        let (dist, e )= queue.pop().unwrap();
        count += e;
        println!("{}, {}", dist, e);
        if count > max_count {
            answer = dist;
            max_count = count;
        }
    }
    writeln!(io::stdout(), "part2 answer: {}", answer)?;
    Ok(())
}

#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn distance(&self, other: &Self) -> u64 {
        let mut d = (self.x - other.x).abs() as u64;
        d += (self.y - other.y).abs() as u64;
        d += (self.z - other.z).abs() as u64;
        d
    }
}

#[derive(Debug)]
struct Nanobot {
    pos: Coordinate,
    radius: u64,
}

impl Nanobot {
    fn in_range(&self, pos: &Coordinate) -> bool {
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
        let pos = Coordinate {
            x: caps["x"].parse()?,
            y: caps["y"].parse()?,
            z: caps["z"].parse()?,
        };
        Ok(Nanobot { pos, radius: caps["r"].parse()? })
    }
}
