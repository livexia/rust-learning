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
    let numbers = parse_input(&input);

    part1(&numbers)?;
    part2(&numbers)?;
    Ok(())
}

fn part1(numbers: &[usize]) -> Result<usize> {
    let start = Instant::now();

    let cur = nth(numbers, 2020);
    writeln!(io::stdout(), "Part 1: {cur}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(cur)
}

fn part2(numbers: &[usize]) -> Result<usize> {
    let start = Instant::now();

    let cur = nth(numbers, 30000000);
    writeln!(io::stdout(), "Part 2: {cur}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(cur)
}

fn nth(numbers: &[usize], max_turn: usize) -> usize {
    let mut last_spoken = vec![
        0;
        max_turn
            .max(*numbers.iter().max().unwrap())
            .max(numbers.len())
    ];
    for (i, &n) in numbers.iter().enumerate() {
        last_spoken[n] = i + 1;
    }
    let mut cur = 0;
    for turn in numbers.len() + 1..max_turn {
        let last_turn = last_spoken[cur];
        last_spoken[cur] = turn;
        if last_turn == 0 {
            cur = 0;
        } else {
            cur = turn - last_turn;
        }
    }
    cur
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|n| n.parse().unwrap())
        .collect()
}

#[test]
fn example_input() {
    let input = "0,3,6";
    let numbers = parse_input(input);
    assert_eq!(part1(&numbers).unwrap(), 436);
    assert_eq!(part1(&parse_input("1,3,2")).unwrap(), 1);
    assert_eq!(part1(&parse_input("2,1,3")).unwrap(), 10);
    assert_eq!(part1(&parse_input("1,2,3")).unwrap(), 27);
    assert_eq!(part1(&parse_input("3,1,2")).unwrap(), 1836);

    assert_eq!(part2(&parse_input("0,3,6")).unwrap(), 175594);
    assert_eq!(part2(&parse_input("3,1,2")).unwrap(), 362);
}
