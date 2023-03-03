use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

// ##../#.../..../..#. => 1100100000000010
fn parse_pattern(input: &str) -> u128 {
    let mut bit_map = 0;
    for c in input.trim().chars() {
        if c == '/' {
            continue;
        }
        bit_map <<= 1;
        bit_map |= if c == '#' { 1 } else { 0 };
    }
    bit_map
}

fn parse_input(input: &str) -> HashMap<(usize, u128), u128> {
    let mut rules = HashMap::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if let Some((left, right)) = line.trim().split_once(" => ") {
            let size = if left.trim().len() == 3 * 3 + 2 { 3 } else { 4 };
            let left = parse_pattern(left);
            let right = parse_pattern(right);
            rules.insert((size, left), right);
        }
    }
    rules
}

struct Image {
    raw: Vec<u128>,
    size: usize,
}

impl Image {
    fn new() -> Self {
        Self {
            raw: vec![
                parse_pattern(".#."),
                parse_pattern("..#"),
                parse_pattern("###"),
            ],
            size: 3,
        }
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // part1()?;
    // part2()?;
    Ok(())
}

fn part1() -> Result<()> {
    let start = Instant::now();

    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    todo!()
}
