use std::{collections::HashMap, io::{self, Read, Write}};
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

    let points = input.lines().
        map(|l| l.parse()).collect::<Result<Vec<Point>>>()?;

    let n = points.len();
    let mut connections: Vec<usize> = (0..n).collect();

    for i in 0..n {
        for j in 0..n {
            if find(i, &mut connections) == find(j, &mut connections) {
                continue;
            }
            if points[i].is_same_constellation(&points[j]) {
                union(i, j, &mut connections)
            }
        }
    }

    let mut constellation = HashSet::new();
    for &i in &connections {
        if !constellation.contains(&i) {
            constellation.insert(i);
        }
    }

    println!("part1 answer: {:?}", constellation.len());

    Ok(())
}

fn union(i: usize, j: usize, connections: &mut Vec<usize>){
    let root_i = find(i, connections);
    let root_j = find(j, connections);
    connections[root_j] = root_i;
}

fn find(i: usize, connections: &mut Vec<usize>) -> usize {
    if connections[i] != i {
        connections[i] = find(connections[i], connections)
    }
    connections[i]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    t: i32
}

impl Point {
    fn distance(&self, other: &Self) -> i32 {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        let dz = (self.z - other.z).abs();
        let dt = (self.t - other.t).abs();
        dx + dy + dz + dt
    }

    fn is_same_constellation(&self, other: &Self) -> bool {
        if self.distance(other) < 4 {
            true
        } else {
            false
        }
    }
}

impl FromStr for Point {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let p: Vec<&str> = s.trim().split(",").collect();
        if p.len() != 4 {
            return err!("unrecognized point: {}", s)
        }
        Ok(Self {
            x: p[0].parse()?,
            y: p[1].parse()?,
            z: p[2].parse()?,
            t: p[3].parse()?,
        })
    }
}

