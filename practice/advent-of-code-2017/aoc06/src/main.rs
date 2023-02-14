use std::collections::{HashMap, HashSet};
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
    let memory = parse_input(&input);

    part1(&memory)?;
    part2(&memory)?;
    Ok(())
}

fn part1(memory: &[usize]) -> Result<usize> {
    let start = Instant::now();

    let mut memory = memory.to_owned();
    let mut result = 0;

    let mut seen = HashSet::new();
    while seen.insert(memory.clone()) {
        result += 1;
        reallocation(&mut memory);
    }

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(memory: &[usize]) -> Result<usize> {
    let start = Instant::now();

    let mut memory = memory.to_owned();
    let result;

    let mut last_seen: HashMap<Vec<usize>, usize> = HashMap::new();
    let mut cur = 0;
    loop {
        cur += 1;
        if let Some(last) = last_seen.get(&memory) {
            result = cur - *last;
            break;
        }
        last_seen.insert(memory.clone(), cur);
        reallocation(&mut memory);
    }

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn reallocation(memory: &mut [usize]) {
    let (index, mut max) = get_max(memory);
    memory[index] = 0;
    for i in (0..memory.len()).cycle().skip(index + 1) {
        max -= 1;
        memory[i] += 1;
        if max == 0 {
            break;
        }
    }
}

fn get_max(memory: &[usize]) -> (usize, usize) {
    let (mut index, mut max) = (0, 0);
    for (i, &b) in memory.iter().enumerate() {
        if b > max {
            index = i;
            max = b;
        }
    }
    (index, max)
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

#[test]
fn example_input() {
    let input = "0 2 7 0";
    assert_eq!(part1(&parse_input(input)).unwrap(), 5);
    assert_eq!(part2(&parse_input(input)).unwrap(), 4);
}
