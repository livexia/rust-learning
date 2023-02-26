use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err{
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Int = u64;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let (a, b) = parse_input(&input);

    part1(a, b)?;
    part2(a, b)?;
    Ok(())
}

fn part1(a: Int, b: Int) -> Result<Int> {
    let start = Instant::now();

    let mut result = 0;
    let (mut a, mut b) = (a, b);
    for _ in 0..40_000_000 {
        (a, b) = (next_value(a, 16807), next_value(b, 48271));
        if matched(a, b) {
            result += 1;
        }
    }

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(a: Int, b: Int) -> Result<Int> {
    let start = Instant::now();

    let mut result = 0;
    let (mut a, mut b) = (a, b);
    for _ in 0..5_000_000 {
        (a, b) = (
            next_value_with_criteria(a, 16807, 4),
            next_value_with_criteria(b, 48271, 8),
        );
        if matched(a, b) {
            result += 1;
        }
    }

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn matched(a: Int, b: Int) -> bool {
    a & 0xffff == b & 0xffff
}

fn next_value(n: Int, factor: Int) -> Int {
    (n * factor) % 2147483647
}

fn next_value_with_criteria(mut n: Int, factor: Int, criteria: Int) -> Int {
    n = next_value(n, factor);
    while n % criteria != 0 {
        n = next_value(n, factor)
    }
    n
}

fn parse_input(input: &str) -> (Int, Int) {
    let lines: Vec<_> = input.lines().filter(|l| !l.is_empty()).collect();
    (
        lines[0].split_whitespace().last().unwrap().parse().unwrap(),
        lines[1].split_whitespace().last().unwrap().parse().unwrap(),
    )
}

#[test]
fn exaple_input() {
    assert_eq!(part1(65, 8921).unwrap(), 588);
    assert_eq!(part2(65, 8921).unwrap(), 309);
}
