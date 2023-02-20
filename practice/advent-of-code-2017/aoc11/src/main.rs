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
    let dirs = parse_input(&input);

    part1(&dirs)?;
    // part2()?;
    Ok(())
}

fn part1(dirs: &[&str]) -> Result<i32> {
    let start = Instant::now();

    let mut cur = (0, 0);
    for dir in dirs {
        cur = next(cur.0, cur.1, dir);
    }
    let result = cur.0.abs() + cur.1.abs();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn next(x: i32, y: i32, dir: &str) -> (i32, i32) {
    match dir {
        "n" => (x - 1, y + 1),
        "ne" => (x, y + 1),
        "se" => (x + 1, y + 1),
        "s" => (x + 1, y - 1),
        "sw" => (x, y - 1),
        "nw" => (x - 1, y - 1),
        _ => unreachable!("wrong dir: {dir}"),
    }
}

fn parse_input(input: &str) -> Vec<&str> {
    input.trim().split(',').collect()
}

#[test]
fn example_input() {
    assert_eq!(part1(&parse_input("ne,ne,ne")).unwrap(), 3);
    assert_eq!(part1(&parse_input("ne,ne,sw,sw")).unwrap(), 0);
    assert_eq!(part1(&parse_input("ne,ne,s,s")).unwrap(), 2);
    assert_eq!(part1(&parse_input("se,sw,se,sw,sw")).unwrap(), 3);
}
