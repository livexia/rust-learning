use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};

use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Int = u64;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input: Vec<Int> = input.lines().map(|l| l.trim().parse().unwrap()).collect();

    part1(&input, 25)?;
    part2(&input, 25)?;
    Ok(())
}

fn part1(input: &[Int], length: usize) -> Result<Int> {
    let start = Instant::now();

    let mut result = None;
    for left in 0..input.len() - length - 1 {
        if !follow_the_rule(&input[left..left + length], input[left + length]) {
            result = Some(input[left + length]);
            break;
        }
    }
    if result.is_none() {
        return err!("Not found");
    }

    writeln!(io::stdout(), "Part 1: {}", result.unwrap())?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result.unwrap())
}

fn part2(input: &[Int], length: usize) -> Result<Int> {
    let start = Instant::now();

    let target = part1(input, length)?;
    let mut result = 0;

    let mut left = 0;
    let mut right = 0;
    let mut sum = 0;
    while right < input.len() {
        match sum.cmp(&target) {
            std::cmp::Ordering::Less => {
                sum += input[right];
                right += 1;
            }
            std::cmp::Ordering::Equal => {
                result = input[left..right].iter().min().unwrap()
                    + input[left..right].iter().max().unwrap();
                break;
            }
            std::cmp::Ordering::Greater => {
                sum -= input[left];
                left += 1;
            }
        }
    }

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn follow_the_rule(preamble: &[Int], num: Int) -> bool {
    let mut visited = HashSet::new();
    for &p in preamble {
        if num >= p && visited.contains(&(num - p)) {
            return true;
        }
        visited.insert(p);
    }
    false
}

#[test]
fn example_input() {
    let input = "35
    20
    15
    25
    47
    40
    62
    55
    65
    95
    102
    117
    150
    182
    127
    219
    299
    277
    309
    576";
    let input: Vec<Int> = input.lines().map(|l| l.trim().parse().unwrap()).collect();
    assert_eq!(part1(&input, 5).unwrap(), 127);
    assert_eq!(part2(&input, 5).unwrap(), 62);
}
