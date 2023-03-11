use nom::{
    bytes::complete::{tag, take, take_till, take_until},
    sequence::preceded,
    IResult,
};
use std::io::{self, Read, Write};
use std::time::Instant;
use std::{collections::HashSet, error::Error};

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

fn parse_begin(input: &str) -> IResult<&str, &str> {
    preceded(tag("Begin in state "), take(1usize))(input)
}

fn parse_checksum(input: &str) -> IResult<&str, &str> {
    preceded(
        take_till(|c: char| c.is_numeric()),
        take_till(|c: char| c == ' '),
    )(input)
}

fn parse_input(input: &str) -> Result<(usize, usize, Vec<State>)> {
    let (input, begin) = match parse_begin(input) {
        Ok((i, b)) => (i, state_to_id(b)),
        _ => return err!("unable to parse the start state"),
    };
    let (input, steps) = match parse_checksum(input) {
        Ok((i, s)) => (i, s.parse()?),
        _ => return err!("unable to parse the checksum"),
    };

    let mut states = vec![];
    let mut input = input;
    while let Ok((n_input, (id, s))) = parse_state(input) {
        input = n_input;
        assert_eq!(states.len(), id);
        states.push(s);
    }
    Ok((begin, steps, states))
}

fn part1(begin: usize, steps: usize, states: &[State]) -> Result<usize> {
    let start = Instant::now();

    let mut tape: HashSet<i32> = HashSet::new();
    let mut cur_state = begin;
    let mut cur_pos = 0;
    for _ in 0..steps {
        let (value, dir, next_state) = if !tape.contains(&cur_pos) {
            states[cur_state].0
        } else {
            states[cur_state].1
        };
        if value == 0 {
            tape.remove(&cur_pos);
        } else {
            tape.insert(cur_pos);
        }
        cur_pos += dir;
        cur_state = next_state;
    }

    let result = tape.len();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let (begin, steps, states) = parse_input(&input)?;

    part1(begin, steps, &states)?;
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
    assert_eq!(part1(begin, steps, &states).unwrap(), 3);
}
