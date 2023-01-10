use hashbrown::HashMap;
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
    // part2()?;
    Ok(())
}

fn part1(numbers: &[usize]) -> Result<usize> {
    let start = Instant::now();

    let mut last_spoken: HashMap<usize, usize> = numbers
        .iter()
        .enumerate()
        .map(|(i, &n)| (n, i + 1))
        .collect();
    let mut cur = 0;
    for turn in numbers.len() + 1..2020 {
        let temp = cur;
        if let Some(&last_turn) = &last_spoken.get(&cur) {
            cur = turn - last_turn;
        } else {
            cur = 0;
        }
        last_spoken.insert(temp, turn);
    }

    writeln!(io::stdout(), "Part 1: {cur}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(cur)
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
}
