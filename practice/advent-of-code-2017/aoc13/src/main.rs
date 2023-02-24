use rayon::prelude::*;
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
    let firewall = parse_input(&input);

    part1(&firewall)?;
    part2(&firewall)?;
    Ok(())
}

fn part1(firewall: &[usize]) -> Result<usize> {
    let start = Instant::now();

    let result = firewall
        .iter()
        .enumerate()
        .filter(|(_, &d)| d != 0)
        .fold(0, |s, (i, d)| {
            s + if i % (2 * d - 2) == 0 { i * d } else { 0 }
        });

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(firewall: &[usize]) -> Result<usize> {
    let start = Instant::now();

    let result = (0..1000000000)
        .into_par_iter()
        .find_first(|delay| {
            firewall
                .iter()
                .enumerate()
                .filter(|(_, &d)| d != 0)
                .all(|(i, d)| (i + delay) % (2 * d - 2) != 0)
        })
        .unwrap();

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn parse_input(input: &str) -> Vec<usize> {
    let mut firewall = vec![];
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if let Some((left, right)) = line.trim().split_once(':') {
            let left = left.trim().parse().unwrap();
            let right = right.trim().parse().unwrap();
            firewall.resize(left, 0);
            firewall.push(right);
        }
    }
    firewall
}

#[test]
fn example_input() {
    let input = "0: 3
        1: 2
        4: 4
        6: 4";
    let firewall = parse_input(input);
    assert_eq!(part1(&firewall).unwrap(), 24);
    assert_eq!(part2(&firewall).unwrap(), 10);
}
