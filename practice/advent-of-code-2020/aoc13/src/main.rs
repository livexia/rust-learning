use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Int = i32;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let (timestamp, buses) = parse_input(&input)?;

    part1(&timestamp, &buses)?;
    // part2()?;
    Ok(())
}

fn part1(timestamp: &Int, buses: &[Option<Int>]) -> Result<Int> {
    let start = Instant::now();

    let bus_id = buses
        .iter()
        .filter_map(|&i| i)
        .min_by_key(|id| id - timestamp % id)
        .unwrap();
    let result = (bus_id - timestamp % bus_id) * bus_id;

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn parse_input(input: &str) -> Result<(Int, Vec<Option<Int>>)> {
    let lines: Vec<_> = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();
    if lines.len() != 2 {
        return err!("not a valid input");
    }
    let timestamp = lines[0].parse()?;
    let buses = lines[1].split(',').map(|s| s.trim().parse().ok()).collect();

    Ok((timestamp, buses))
}

#[test]
fn example_input() {
    let input = "939
    7,13,x,x,59,x,31,19";
    let (timestamp, buses) = parse_input(input).unwrap();
    assert_eq!(part1(&timestamp, &buses).unwrap(), 295);
}
