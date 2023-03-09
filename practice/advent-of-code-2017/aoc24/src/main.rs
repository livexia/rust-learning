use std::collections::{HashSet, VecDeque};
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

#[allow(dead_code)]
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

#[derive(Debug, Clone, Copy)]
struct Component {
    id: usize,
    port: (usize, usize),
}

impl Component {
    fn new(id: usize, port: (usize, usize)) -> Component {
        Self { id, port }
    }

    fn rev(&self) -> Self {
        Self {
            id: self.id,
            port: (self.port.1, self.port.0),
        }
    }

    fn hash(&self) -> u128 {
        1 << self.id
    }

    fn contained_in(&self, h: u128) -> bool {
        self.hash() & h != 0
    }

    fn strength(&self) -> usize {
        self.port.0 + self.port.1
    }
}

fn ports_to_components(components: &[(usize, usize)]) -> Vec<Component> {
    components
        .iter()
        .enumerate()
        .map(|(i, p)| Component::new(i, *p))
        .collect()
}

fn bfs(components: &[(usize, usize)]) -> (usize, usize) {
    let components = ports_to_components(components);
    let src = Component::new(components.len(), (0, 0));
    assert!(components.len() < 128);

    let mut queue = VecDeque::new();
    queue.push_back((src, 0, 0));

    let mut max_strength = 0;
    let mut max_depth_strength = 0;
    while !queue.is_empty() {
        let size = queue.len();
        max_depth_strength = 0;
        for _ in 0..size {
            let (cur, path, s) = queue.pop_front().unwrap();
            max_strength = s.max(max_strength);
            max_depth_strength = s.max(max_depth_strength);
            for next in &components {
                if !next.contained_in(path) {
                    if cur.port.1 == next.port.0 {
                        queue.push_back((*next, path | next.hash(), s + next.strength()))
                    } else if cur.port.1 == next.port.1 {
                        queue.push_back((next.rev(), path | next.hash(), s + next.strength()))
                    }
                }
            }
        }
    }
    (max_strength, max_depth_strength)
}

fn part1(components: &[(usize, usize)]) -> Result<usize> {
    let start = Instant::now();

    let result = bfs(components).0;

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(components: &[(usize, usize)]) -> Result<usize> {
    let start = Instant::now();

    let result = bfs(components).1;

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let components = parse_input(&input)?;

    part1(&components)?;
    part2(&components)?;
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
    assert_eq!(part2(&components).unwrap(), 19);
}
