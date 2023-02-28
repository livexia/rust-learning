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

    let limit = 2018;
    let mut buffer: Vec<_> = vec![0; limit];
    for i in 1..limit {
        buffer[i] = (buffer[i - 1] + steps) % i + 1;
        for j in 0..limit {
            if i == j {
                continue;
            }
            if buffer[j] >= buffer[i] {
                buffer[j] += 1;
            }
        }
    }

    let result = buffer
        .iter()
        .position(|p| *p == buffer[limit - 1] + 1)
        .unwrap();

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
    assert_eq!(part1(312).unwrap(), 772);
}
