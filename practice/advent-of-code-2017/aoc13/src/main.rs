use std::collections::HashSet;
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
    let firewall = parse_input(&input);

    part1(&firewall)?;
    part2(&firewall)?;
    Ok(())
}

fn part1(firewall: &[Layer]) -> Result<usize> {
    let start = Instant::now();

    let firewall = init_firewall(firewall);
    let result = firewall.iter().enumerate().fold(0, |s, (i, l)| {
        s + if l.scanner == 0 { i * l.depth } else { 0 }
    });

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(firewall: &[Layer]) -> Result<usize> {
    let start = Instant::now();

    let mut result = 0;
    let mut trimed_firewall = vec![];
    let mut dedup = HashSet::new();
    for layer in init_firewall(firewall) {
        if dedup.insert(layer.clone()) {
            trimed_firewall.push(layer);
        }
    }
    for delay in 0.. {
        if trimed_firewall
            .iter()
            .all(|l| l.depth == 0 || l.scanner != 0)
        {
            result = delay;
            break;
        }
        trimed_firewall.iter_mut().for_each(|l| l.next());
    }

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn init_firewall(firewall: &[Layer]) -> Vec<Layer> {
    let mut firewall = firewall.to_owned();
    for i in 0..firewall.len() {
        for _ in 0..i {
            firewall[i].next();
        }
    }
    firewall
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Layer {
    depth: usize,
    scanner: usize,
    dir: bool,
}

impl Layer {
    fn new(depth: usize) -> Self {
        Self {
            depth,
            scanner: 0,
            dir: true,
        }
    }

    fn next(&mut self) {
        if self.depth == 0 {
            return;
        }
        if self.scanner == 0 {
            self.dir = true
        } else if self.scanner + 1 >= self.depth {
            self.dir = false
        }

        if self.dir {
            self.scanner += 1;
        } else {
            self.scanner -= 1;
        }
    }
}

fn parse_input(input: &str) -> Vec<Layer> {
    let mut firewall = vec![];
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if let Some((left, right)) = line.trim().split_once(':') {
            let left = left.trim().parse().unwrap();
            let right = right.trim().parse().unwrap();
            for _ in firewall.len()..left {
                firewall.push(Layer::new(0));
            }
            firewall.push(Layer::new(right))
        }
    }
    firewall
}

#[test]
fn example_input() {
    let input = "0: 3
        1: 2
        4: 4
        6: 4";
    let firewall = parse_input(input);
    assert_eq!(part1(&firewall).unwrap(), 24);
    assert_eq!(part2(&firewall).unwrap(), 10);
}
