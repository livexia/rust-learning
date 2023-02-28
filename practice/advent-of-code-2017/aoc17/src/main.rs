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
    let steps = parse_input(&input);

    part1(steps)?;
    // part2()?;
    Ok(())
}

fn part1(steps: usize) -> Result<usize> {
    let start = Instant::now();

    let mut buffer: Vec<_> = (0..=2017).collect();
    for i in 1..=10 {
        buffer[i] = (buffer[i - 1] + steps) % (i + 1);
        dbg!(buffer[i]);
    }

    let result = buffer.iter().position(|p| *p == buffer[2017] + 1).unwrap();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn parse_input(input: &str) -> usize {
    input.trim().parse().unwrap()
}

#[test]
fn example_input() {
    assert_eq!(part1(3).unwrap(), 638);
}
