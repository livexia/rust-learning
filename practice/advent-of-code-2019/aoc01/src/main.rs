use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Int = u32;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mods = parse_input(&input)?;

    part1(&mods)?;
    part2(&mods)?;
    Ok(())
}

fn part1(mods: &[Int]) -> Result<Int> {
    let start = Instant::now();

    let result: Int = mods.iter().cloned().map(calc_fuel).sum();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(mods: &[Int]) -> Result<Int> {
    let start = Instant::now();

    let result: Int = mods.iter().cloned().map(calc_total_fuel).sum();

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn calc_total_fuel(mass: Int) -> Int {
    let mut total = 0;
    let mut mass = mass;
    while mass != 0 {
        mass = calc_fuel(mass);
        total += mass;
    }
    total
}

fn calc_fuel(mass: Int) -> Int {
    (mass / 3).saturating_sub(2)
}

fn parse_input(input: &str) -> Result<Vec<Int>> {
    let mut r = vec![];

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        r.push(line.trim().parse()?);
    }
    Ok(r)
}
