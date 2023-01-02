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

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<u32> {
    let start = Instant::now();

    let groups = parse_input(input);
    let result: u32 = groups
        .iter()
        .map(|g| bit_count(g.iter().fold(0, |bits, b| bits | b)))
        .sum();

    writeln!(io::stdout(), "Part1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(input: &str) -> Result<u32> {
    let start = Instant::now();

    let groups = parse_input(input);
    let result: u32 = groups
        .iter()
        .map(|g| bit_count(g.iter().fold(0xffffffff, |bits, b| bits & b)))
        .sum();

    writeln!(io::stdout(), "Part2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn bit_count(mut num: u32) -> u32 {
    let mut counter = 0;
    while num != 0 {
        if num & 1 == 1 {
            counter += 1;
        }
        num >>= 1;
    }
    counter
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut groups = vec![vec![]];
    for line in input.lines() {
        if line.trim().is_empty() {
            groups.push(vec![])
        } else {
            groups.last_mut().unwrap().push(
                line.trim()
                    .bytes()
                    .fold(0, |bits, b| bits | 1 << (b - b'a')),
            )
        }
    }
    groups
}

#[test]
fn example_input() {
    let input = "abc

    a
    b
    c
    
    ab
    ac
    
    a
    a
    a
    a
    
    b";
    assert_eq!(part1(input).unwrap(), 11);
    assert_eq!(part2(input).unwrap(), 6);
}
