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
    let edges = parse_input(&input);

    part1(&edges)?;
    // part2()?;
    Ok(())
}

fn part1(edges: &[(usize, usize)]) -> Result<usize> {
    let start = Instant::now();

    let &length = edges.iter().map(|(a, b)| a.max(b)).max().unwrap();
    let mut sets: Vec<_> = (0..=length).collect();
    let mut sizes: Vec<_> = vec![1; length + 1];
    for &(v1, v2) in edges {
        union(v1, v2, &mut sets, &mut sizes);
    }

    let zero = find(0, &mut sets);
    dbg!(zero);
    let result = sets
        .clone()
        .iter()
        .filter(|&&v| find(v, &mut sets) == zero)
        .count();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn union(v1: usize, v2: usize, sets: &mut [usize], sizes: &mut [usize]) {
    let root1 = find(v1, sets);
    let root2 = find(v2, sets);
    let (root1, root2, v2) = if sizes[root1] < sizes[root2] {
        (root2, root1, v1)
    } else {
        (root1, root2, v2)
    };
    if root1 != root2 {
        sets[v2] = root1;
        sizes[root1] += sizes[root2]
    }
}

fn find(mut v: usize, sets: &mut [usize]) -> usize {
    while sets[v] != v {
        sets[v] = sets[sets[v]];
        v = sets[v];
    }
    v
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let mut edges = vec![];
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if let Some((left, right)) = line.trim().split_once(" <-> ") {
            let left = left.trim().parse().unwrap();
            for right in right.split(',') {
                edges.push((left, right.trim().parse().unwrap()))
            }
        }
    }
    edges
}

#[test]
fn example_input() {
    let input = "0 <-> 2
        1 <-> 1
        2 <-> 0, 3, 4
        3 <-> 2, 4
        4 <-> 2, 3, 6
        5 <-> 6
        6 <-> 4, 5";
    let edges = parse_input(input);
    assert_eq!(part1(&edges).unwrap(), 6);
}
