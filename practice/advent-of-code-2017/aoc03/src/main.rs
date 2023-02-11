use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Coord = (i32, i32);

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let num = parse_input(&input);

    part1(num)?;
    // part2()?;
    Ok(())
}

fn part1(num: i32) -> Result<i32> {
    let start = Instant::now();

    let result = dis(&num_to_dis(num), &(0, 0));

    writeln!(io::stdout(), "> Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn num_to_dis(num: i32) -> Coord {
    let mut l = (num as f64).sqrt() as i32;
    if l % 2 == 0 {
        l -= 1;
    };
    if l * l == num {
        return (l / 2, l / 2);
    }
    let c = l / 2 + 1;
    let offset = num - l * l;
    let side = offset / (l + 1);
    match side {
        0 => (c - offset % (l + 1), c),
        1 => (-c, c - offset % (l + 1)),
        2 => (-c + offset % (l + 1), -c),
        3 => (c, -c + offset % (l + 1)),
        _ => unreachable!(),
    }
}

fn dis(c1: &Coord, c2: &Coord) -> i32 {
    (c1.0.abs_diff(c2.0) + c1.1.abs_diff(c2.1)) as i32
}

fn parse_input(input: &str) -> i32 {
    input.trim().parse().unwrap()
}

#[test]
fn example_input() {
    assert_eq!(part1(23).unwrap(), 2);
    assert_eq!(part1(12).unwrap(), 3);
    assert_eq!(part1(1024).unwrap(), 31);
}
