use std::io::{self, Read, Write};
use std::result;
use std::error::Error;

type Result<T> = result::Result<T, Box<dyn Error>>;


fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input = input.trim();

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()>{
    let input_stack: Vec<char> = input.chars().collect();
    writeln!(io::stdout(), "remain units length: {}", react(&input_stack, '.').len())?;
    Ok(())
}

fn part2(input: &str) -> Result<()>{
    let input_stack: Vec<char> = input.chars().collect();
    let input_stack = react(&input_stack, '.');
    let mut polymer_length = input_stack.len();
    for c in b'a'..b'z' {
        let length = react(&input_stack, c as char);
        polymer_length = polymer_length.min(length.len());
    }
    writeln!(io::stdout(), "shortest units length: {}", polymer_length)?;
    Ok(())
}

fn react(input_stack: &[char], exclude_char: char) -> Vec<char> {
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
    stack
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
