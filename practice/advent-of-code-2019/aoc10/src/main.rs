use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Coord = (i32, i32);

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let map = parse_input(&input);

    part1(&map)?;
    // part2()?;
    Ok(())
}

fn part1(map: &HashSet<Coord>) -> Result<usize> {
    let start = Instant::now();

    let mut dp: HashMap<Coord, Vec<Coord>> = HashMap::new();

    for &c1 in map.iter() {
        let entry = dp.entry(c1).or_default();
        for &c2 in map {
            if c1 == c2 {
                continue;
            }
            if visible(c1, c2, map) {
                entry.push(c2);
            }
        }
    }
    let result = dp.values().map(|v| v.len()).max().unwrap();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn visible(c1: Coord, c2: Coord, map: &HashSet<Coord>) -> bool {
    for &c3 in map {
        if c3 == c1 || c3 == c2 {
            continue;
        }
        if in_between(c1, c2, c3) {
            return false;
        }
    }
    true
}

fn in_between(c1: Coord, c2: Coord, c3: Coord) -> bool {
    // https://stackoverflow.com/a/328122
    let croos = cross_product(c1, c2, c3);
    if croos != 0 {
        return false;
    }
    let dot = dot_product(c1, c2, c3);
    if dot < 0 {
        return false;
    }
    let length = squared_lenth(c1, c2);
    if dot > length {
        return false;
    }
    true
}

fn cross_product(c1: Coord, c2: Coord, c3: Coord) -> i32 {
    (c3.1 - c1.1) * (c2.0 - c1.0) - (c3.0 - c1.0) * (c2.1 - c1.1)
}

fn dot_product(c1: Coord, c2: Coord, c3: Coord) -> i32 {
    (c3.0 - c1.0) * (c2.0 - c1.0) + (c3.1 - c1.1) * (c2.1 - c1.1)
}

fn squared_lenth(c1: Coord, c2: Coord) -> i32 {
    (c1.0 - c2.0).pow(2) + (c1.1 - c2.1).pow(2)
}

fn parse_input(input: &str) -> HashSet<Coord> {
    let mut map = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.trim().char_indices() {
            if char == '#' {
                map.insert((x as i32, y as i32));
            }
        }
    }
    map
}

#[test]
fn example_input() {
    let input = ".#..#
    .....
    #####
    ....#
    ...##";
    let map = parse_input(input);
    println!("{:?}", map);
    assert_eq!(part1(&map).unwrap(), 8);
}
