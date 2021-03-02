use std::io::{self, Read, Write};
use std::result;
use std::error::Error;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}

type Result<T> = result::Result<T, Box<dyn Error>>;


fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()>{
    let input_stack = input.chars().collect::<Vec<char>>();
    writeln!(io::stdout(), "remain units length: {}", reaction(&input_stack, '.').unwrap_or(vec![]).len())?;
    Ok(())
}

fn part2(input: &str) -> Result<()>{
    let input_stack = input.chars().collect::<Vec<char>>();
    let input_stack = reaction(&input_stack, '.').unwrap_or(vec![]);
    let mut polymer_length = input_stack.len();
    for c in 0..26 {
        let length = reaction(&input_stack, (c + b'a') as char);
        polymer_length = polymer_length.min(length.unwrap_or(vec![]).len());
    }
    writeln!(io::stdout(), "shortest units length: {}", polymer_length)?;
    Ok(())
}

fn reaction(input_stack: &[char], exclude_char: char) -> Option<Vec<char>> {
    if input_stack.is_empty() {
        return None;
    }
    let mut stack = vec![];
    for &c in input_stack {
        if c.to_ascii_lowercase() == exclude_char {
            continue;
        }
        if stack.is_empty() {
            stack.push(c);
            continue;
        }
        if is_opposite_polarity(c, stack[stack.len()-1]) {
            stack.pop();
        } else {
            stack.push(c)
        }
    }
    Some(stack)
}

fn is_opposite_polarity(c1: char, c2: char) -> bool {
    if c1 == c2 {
        return false;
    }
    if c1.to_ascii_lowercase() == c2.to_ascii_lowercase() { 
        return true 
    };
    false
}
