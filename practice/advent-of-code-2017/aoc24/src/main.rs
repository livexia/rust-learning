use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn parse_input(input: &str) -> Result<Vec<(usize, usize)>> {
    let mut r = vec![];
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if let Some((left, right)) = line.trim().split_once('/') {
            let a: usize = left.parse()?;
            let b: usize = right.parse()?;
            r.push((a, b));
        }
    }
    Ok(r)
}

fn dfs(
    cur: (usize, usize),
    components: &[(usize, usize)],
    visited: &mut HashSet<(usize, usize)>,
) -> usize {
    let mut max_strength = cur.0 + cur.1;
    for &(a, b) in components {
        if a == cur.1 && visited.insert((a, b)) {
            max_strength = max_strength.max(dfs((a, b), components, visited) + cur.0 + cur.1);
            visited.remove(&(a, b));
        } else if b == cur.1 && visited.insert((a, b)) {
            max_strength = max_strength.max(dfs((b, a), components, visited) + cur.0 + cur.1);
            visited.remove(&(a, b));
        }
    }
    max_strength
}

fn part1(components: &[(usize, usize)]) -> Result<usize> {
    let start = Instant::now();

    let result = dfs((0, 0), components, &mut HashSet::new());

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let components = parse_input(&input)?;

    part1(&components)?;
    // part2()?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "0/2
        2/2
        2/3
        3/4
        3/5
        0/1
        10/1
        9/10";
    let components = parse_input(input).unwrap();
    assert_eq!(part1(&components).unwrap(), 31);
}
