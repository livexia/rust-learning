use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type State = ((u8, i32, usize), (u8, i32, usize));

fn parse_input(input: &str) -> (usize, usize, Vec<State>) {
    let mut begin: usize = 0;
    let mut steps: usize = 0;
    let mut input = input.split("\n\n");
    if let Some(rows) = input.next() {
        let mut rows = rows.lines();
        if let Some(first) = rows.next() {
            if let Some(first) = first.strip_prefix("Begin in state ") {
                begin = (first.bytes().next().unwrap() - b'A') as usize;
            }
        }
        if let Some(second) = rows.next() {
            if let Some(second) = second.strip_prefix("Perform a diagnostic checksum after ") {
                steps = second.split(' ').next().unwrap().parse().unwrap();
            }
        }
    }
    dbg!(begin, steps);
    for rows in input {
        let mut rows = rows.lines();
        if let Some(first) = rows.next() {
            if let Some(first) = first.strip_prefix("Begin in state ") {
                begin = (first.bytes().next().unwrap() - b'A') as usize;
            }
        }
    }
    todo!()
}

fn part1() -> Result<()> {
    let start = Instant::now();

    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    todo!()
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let (begin, steps, states) = parse_input(&input);

    // part1()?;
    // part2()?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.";
    let (begin, steps, states) = parse_input(&input);
}
