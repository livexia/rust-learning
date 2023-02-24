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
    // part2()?;
    Ok(())
}

fn part1(firewall: &[Layer]) -> Result<usize> {
    let start = Instant::now();

    let mut result = 0;
    let mut firewall = firewall.to_owned();
    let mut init = true;
    let mut packet_layer = 0;
    while packet_layer < firewall.len() {
        if !init {
            packet_layer += 1;
        } else {
            init = false;
        }

        for (i, layer) in firewall.iter_mut().enumerate() {
            if layer.depth == 0 {
                continue;
            }
            if layer.scanner == 0 && packet_layer == i {
                result += i * layer.depth;
            }
            layer.next();
        }
    }

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

#[derive(Debug, Clone)]
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
        if self.scanner == 0 {
            self.dir = true
        } else if self.scanner + 1 == self.depth {
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
}
