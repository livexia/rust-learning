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

    let gen_a = Gen::new(a, 16807);
    let gen_b = Gen::new(b, 48271);
    let result = gen_a
        .zip(gen_b)
        .take(40_000_000)
        .filter(|&(a, b)| matched(a, b))
        .count() as u64;

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(a: Int, b: Int) -> Result<Int> {
    let start = Instant::now();

    let gen_a = Gen::new(a, 16807);
    let gen_b = Gen::new(b, 48271);
    let result = gen_a
        .filter(|a| a % 4 == 0)
        .zip(gen_b.filter(|b| b % 8 == 0))
        .take(5_000_000)
        .filter(|&(a, b)| matched(a, b))
        .count() as u64;

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn matched(a: Int, b: Int) -> bool {
    a & 0xffff == b & 0xffff
}

#[allow(dead_code)]
fn next_value(n: Int, factor: Int) -> Int {
    (n * factor) % 2147483647
}

#[allow(dead_code)]
fn next_value_with_criteria(mut n: Int, factor: Int, criteria: Int) -> Int {
    n = next_value(n, factor);
    while n % criteria != 0 {
        n = next_value(n, factor)
    }
    n
}

struct Gen {
    prev: Int,
    factor: Int,
}

impl Gen {
    fn new(n: Int, factor: Int) -> Self {
        Self { prev: n, factor }
    }
}

impl Iterator for Gen {
    type Item = Int;

    fn next(&mut self) -> Option<Int> {
        self.prev = (self.prev * self.factor) % 2147483647;
        Some(self.prev)
    }
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
