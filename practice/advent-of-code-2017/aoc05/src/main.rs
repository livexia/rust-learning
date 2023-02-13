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
    let instrs = parse_input(&input);

    part1(&instrs)?;
    // part2()?;
    Ok(())
}

fn part1(instrs: &[i32]) -> Result<i32> {
    let start = Instant::now();

    let mut instrs = instrs.to_owned();
    let mut step = 0;

    let mut cur = 0;

    while cur < instrs.len() {
        let next = cur as i32 + instrs[cur];
        if next < 0 {
            break;
        }
        instrs[cur] += 1;
        cur = next as usize;
        step += 1;
    }

    writeln!(io::stdout(), "Part 1: {step}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(step)
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.trim().parse().unwrap())
        .collect()
}

#[test]
fn example_input() {
    let input = "0
        3
        0
        1
        -3";
    let instrs = parse_input(input);
    assert_eq!(part1(&instrs).unwrap(), 5);
}
