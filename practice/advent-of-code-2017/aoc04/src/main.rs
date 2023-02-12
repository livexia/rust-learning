use std::collections::HashSet;
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
    let list = parse_input(&input);

    part1(&list)?;
    // part2()?;
    Ok(())
}

fn part1(list: &[Vec<&str>]) -> Result<usize> {
    let start = Instant::now();

    let result = list.iter().map(|r| verify(r)).filter(|&b| b).count();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn verify(password: &[&str]) -> bool {
    let set: HashSet<_> = password.iter().collect();
    set.len() == password.len()
}

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split_whitespace().collect())
        .collect()
}

#[test]
fn example_input() {
    assert!(verify(&["aa", "bb", "cc"]));
    assert!(!verify(&["aa", "bb", "cc", "aa"]));
}
