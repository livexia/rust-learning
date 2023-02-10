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

    let result = calc_match(input.trim(), 1);

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(input: &str) -> Result<u32> {
    let start = Instant::now();

    let result = calc_match(input.trim(), input.trim().len() / 2);

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn calc_match(input: &str, step: usize) -> u32 {
    let mut result = 0;
    for (c, n) in input.bytes().zip(input.bytes().cycle().skip(step)) {
        if c == n {
            result += (c - b'0') as u32;
        }
    }
    result
}

#[test]
fn exmaple_input() {
    assert_eq!(part1("1111").unwrap(), 4);
    assert_eq!(part1("1122").unwrap(), 3);
    assert_eq!(part1("1234").unwrap(), 0);
    assert_eq!(part1("91212129").unwrap(), 9);

    assert_eq!(part2("1212").unwrap(), 6);
    assert_eq!(part2("1221").unwrap(), 0);
    assert_eq!(part2("123425").unwrap(), 4);
    assert_eq!(part2("123123").unwrap(), 12);
    assert_eq!(part2("12131415").unwrap(), 4);
}
