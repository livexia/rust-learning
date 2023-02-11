use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input = parse_input(&input);

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &[Vec<u32>]) -> Result<u32> {
    let start = Instant::now();

    let result = checksum(input);

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(input: &[Vec<u32>]) -> Result<u32> {
    let start = Instant::now();

    let result = input.iter().map(|r| evenly_divide(r).unwrap()).sum();

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn checksum(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|v| v.iter().max().unwrap() - v.iter().min().unwrap())
        .sum()
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            l.trim()
                .split_ascii_whitespace()
                .map(|i| i.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn evenly_divide(row: &[u32]) -> Option<u32> {
    for (i, a) in row.iter().enumerate() {
        for b in row.iter().skip(i + 1) {
            if a % b == 0 {
                return Some(a / b);
            } else if b % a == 0 {
                return Some(b / a);
            }
        }
    }
    None
}

#[test]
fn example_input() {
    let input = "5 1 9 5
    7 5 3
    2 4 6 8";
    assert_eq!(part1(&parse_input(input)).unwrap(), 18);

    let input = "5 9 2 8
    9 4 7 3
    3 8 6 5";
    assert_eq!(part2(&parse_input(input)).unwrap(), 9);
}
