use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Int = u64;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let (k1, k2) = parse_input(&input)?;

    part1(k1, k2)?;
    // part2()?;
    Ok(())
}

fn part1(k1: Int, k2: Int) -> Result<Int> {
    let start = Instant::now();

    let (l1, l2) = loop_size_test(k1, k2);
    let e1 = transform(k1, l2);
    let e2 = transform(k2, l1);
    assert_eq!(e1, e2);

    writeln!(io::stdout(), "Part 1: {e1}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(e1)
}

fn loop_size_test(k1: Int, k2: Int) -> (Int, Int) {
    let mut loop_size = (0, 0);
    let mut value = 1;
    for i in 1.. {
        value = value * 7 % 20201227;
        if value == k1 {
            loop_size.0 = i;
        } else if value == k2 {
            loop_size.1 = i;
        }
        if loop_size.0 != 0 && loop_size.1 != 0 {
            break;
        }
    }
    loop_size
}

fn transform(num: Int, loop_size: Int) -> Int {
    let mut value = 1;
    for _ in 0..loop_size {
        value = value * num % 20201227
    }
    value
}

fn parse_input(input: &str) -> Result<(Int, Int)> {
    let lines: Vec<_> = input.lines().collect();
    if lines.len() != 2 {
        return err!("input not contains two public key");
    }
    Ok((lines[0].trim().parse()?, lines[1].trim().parse()?))
}

#[test]
fn example_input() {
    assert_eq!(17807724, transform(7, 11));
    assert_eq!((11, 8), loop_size_test(17807724, 5764801));
    assert_eq!(5764801, transform(7, 8));
    assert_eq!(14897079, transform(17807724, 8));
    assert_eq!(14897079, transform(5764801, 11));

    assert_eq!(part1(17807724, 5764801).unwrap(), 14897079)
}
