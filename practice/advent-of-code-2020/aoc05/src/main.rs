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

fn part1(input: &str) -> Result<usize> {
    let start = Instant::now();

    let result = input.lines().map(|l| seat_id(l.trim())).max().unwrap();

    writeln!(io::stdout(), "Part1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(input: &str) -> Result<usize> {
    let start = Instant::now();

    use std::collections::HashSet;
    let seat_ids: HashSet<_> = input.lines().map(|l| seat_id(l.trim())).collect();
    let result = *seat_ids
        .iter()
        .find(|&&id| !seat_ids.contains(&(id + 1)) && seat_ids.contains(&(id + 2)))
        .unwrap()
        + 1;

    writeln!(io::stdout(), "Part2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn seat_id(s: &str) -> usize {
    let (row, column) = binary_search(s);
    row * 8 + column
}

fn binary_search(s: &str) -> (usize, usize) {
    let mut left = 0;
    let mut right = 127;
    let mut column = 0;
    for (i, c) in s.char_indices() {
        match c {
            'F' => right = (left + right) / 2,
            'B' => left = (left + right) / 2,
            'L' | 'R' => {
                let mut left = 0;
                let mut right = 7;
                for c in s.chars().skip(i) {
                    match c {
                        'L' => right = (left + right) / 2,
                        'R' => left = (left + right) / 2,
                        _ => unreachable!(),
                    }
                }
                column = right;
                break;
            }
            _ => unreachable!(),
        }
    }
    (right, column)
}

#[test]
fn example_input() {
    let input = "BFFFBBFRRR";
    assert_eq!(seat_id(input), 567);
    let input = "FFFBBBFRRR";
    assert_eq!(seat_id(input), 119);
    let input = "BBFFBBFRLL";
    assert_eq!(seat_id(input), 820);
}
