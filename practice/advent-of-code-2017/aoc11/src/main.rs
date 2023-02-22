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
    let dirs = parse_input(&input);

    part1(&dirs)?;
    // part2()?;
    Ok(())
}

fn part1(dirs: &[&str]) -> Result<i32> {
    let start = Instant::now();

    let mut cur = (Dis::H(0), Dis::V(0));
    for dir in dirs {
        cur = next(cur.0, cur.1, dir);
    }
    let (b, a) = cur.0.dis(&cur.1);

    let result = b / 3 + (a - b / 3) / 2;

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

#[derive(Clone, Copy, Debug)]
enum Dis {
    H(i32),
    V(i32),
}

impl Dis {
    fn add_h(self, i: i32) -> Self {
        match self {
            Dis::H(a) => Dis::H(a + i),
            _ => self,
        }
    }

    fn add_v(self, i: i32) -> Self {
        match self {
            Dis::V(b) => Dis::V(b + i),
            _ => self,
        }
    }

    fn dis(&self, &other: &Dis) -> (i32, i32) {
        let a = match self {
            Dis::H(a) => a,
            Dis::V(b) => b,
        };
        let b = match other {
            Dis::H(a) => a,
            Dis::V(b) => b,
        };
        (a.abs(), b.abs())
    }
}

fn next(x: Dis, y: Dis, dir: &str) -> (Dis, Dis) {
    match dir {
        "n" => (x, y.add_v(2)),
        "ne" => (x.add_h(3), y.add_v(1)),
        "se" => (x.add_h(3), y.add_v(-1)),
        "s" => (x, y.add_v(-2)),
        "sw" => (x.add_h(-3), y.add_v(-1)),
        "nw" => (x.add_h(-3), y.add_v(1)),
        _ => unreachable!("wrong dir: {dir}"),
    }
}

fn parse_input(input: &str) -> Vec<&str> {
    input.trim().split(',').collect()
}

#[test]
fn example_input() {
    assert_eq!(part1(&parse_input("ne,ne,ne")).unwrap(), 3);
    assert_eq!(part1(&parse_input("ne,ne,sw,sw")).unwrap(), 0);
    assert_eq!(part1(&parse_input("ne,ne,s,s")).unwrap(), 2);
    assert_eq!(part1(&parse_input("se,sw,se,sw,sw")).unwrap(), 3);
}
