use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let list: Vec<Line> = input.lines().map(|l| l.parse()).collect::<Result<_>>()?;

    part1(&list)?;
    part2(&list)?;
    Ok(())
}

fn part1(list: &[Line]) -> Result<()> {
    let count = list.iter().filter(|l| l.is_valid_part1()).count();
    writeln!(
        io::stdout(),
        "Part1: How many passwords are valid according to their policies? {}",
        count
    )?;
    Ok(())
}

fn part2(list: &[Line]) -> Result<()> {
    let count = list.iter().filter(|l| l.is_valid_part2()).count();
    writeln!(
        io::stdout(),
        "Part2: How many passwords are valid according to their policies? {}",
        count
    )?;
    Ok(())
}

struct Line {
    range: (usize, usize),
    letter: char,
    password: String,
}

impl Line {
    fn is_valid_part1(&self) -> bool {
        let mut shown = HashMap::new();
        for c in self.password.chars() {
            *shown.entry(c).or_insert(0) += 1;
        }
        let &count = shown.get(&self.letter).unwrap_or(&0);
        self.range.0 <= count && self.range.1 >= count
    }

    fn is_valid_part2(&self) -> bool {
        let password: Vec<_> = self.password.chars().collect();
        (password[self.range.0 - 1] == self.letter) ^ (password[self.range.1 - 1] == self.letter)
    }
}

impl FromStr for Line {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((policy, password)) = s.split_once(": ") {
            let password = password.to_string();
            if let Some((range, letter)) = policy.split_once(' ') {
                if let Some((start, end)) = range.split_once('-') {
                    let range: (usize, usize) = (start.parse()?, end.parse()?);
                    if let Some(letter) = letter.chars().next() {
                        return Ok(Self {
                            range,
                            letter,
                            password,
                        });
                    }
                }
            }
        }
        err!("This is not a valid line: {}", s)
    }
}
