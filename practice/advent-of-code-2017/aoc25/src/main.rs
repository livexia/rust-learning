use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till, take_until},
    character::is_alphabetic,
    sequence::preceded,
    Finish, IResult,
};
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Rule = (u8, i32, usize);
type State = (Rule, Rule);

fn state_to_id(input: &str) -> usize {
    (input.bytes().next().unwrap() - b'A') as usize
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, write_value) = preceded(
        take_until("Write the value "),
        preceded(take_till(|c: char| c.is_numeric()), take(1usize)),
    )(input)?;
    let (input, move_dir) = preceded(
        take_until("Move one slot to the "),
        preceded(tag("Move one slot to the "), take(1usize)),
    )(input)?;
    let move_dir = if move_dir == "r" { 1 } else { -1 };
    let (input, next_state) = preceded(
        take_until("Continue with state "),
        preceded(tag("Continue with state "), take(1usize)),
    )(input)?;
    println!("{:?} {:?} {:?}", write_value, move_dir, next_state);
    Ok((
        input,
        (
            write_value.parse().unwrap(),
            move_dir,
            state_to_id(next_state),
        ),
    ))
}

fn parse_state(input: &str) -> IResult<&str, (usize, State)> {
    let (input, state) = preceded(
        take_until("In state "),
        preceded(tag("In state "), take(1usize)),
    )(input)?;
    let state = state_to_id(state);
    let (input, zero_rule) = parse_rule(input)?;
    let (input, one_rule) = parse_rule(input)?;

    Ok((input, (state, (zero_rule, one_rule))))
}

fn parse_input(input: &str) -> IResult<&str, (usize, usize, Vec<State>)> {
    let (input, begin) = preceded(tag("Begin in state "), take(1usize))(input)?;
    let (input, steps) = preceded(
        take_till(|c: char| c.is_numeric()),
        take_till(|c: char| c == ' '),
    )(input)?;
    let begin = state_to_id(begin);
    let steps = steps.parse().unwrap();
    dbg!(begin, steps);

    println!("{:?}", parse_state(input));
    Ok(("", (begin, steps, vec![])))
}

fn part1() -> Result<()> {
    let start = Instant::now();

    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    todo!()
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let (_, (begin, steps, states)) = parse_input(&input)?;

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
    let (begin, steps, states) = parse_input(&input).unwrap();
}
