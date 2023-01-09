use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Int = i64;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let (timestamp, buses) = parse_input(&input)?;

    part1(&timestamp, &buses)?;
    part2(&buses)?;
    Ok(())
}

fn part1(timestamp: &Int, buses: &[Int]) -> Result<Int> {
    let start = Instant::now();

    let bus_id = buses
        .iter()
        .filter(|&&i| i != 1)
        .min_by_key(|&&id| id - timestamp % id)
        .unwrap();
    let result = (bus_id - timestamp % bus_id) * bus_id;

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(buses: &[Int]) -> Result<Int> {
    let start = Instant::now();

    let buses: Vec<_> = buses
        .iter()
        .enumerate()
        .filter(|(_, id)| id != &&1)
        .collect();
    let mut t = *buses[0].1;
    let mut step = 1;
    let mut cur = 0;
    loop {
        for (i, &(offset, id)) in buses.iter().enumerate().skip(cur) {
            if (t + offset as Int) % id != 0 {
                t += step;
                break;
            } else {
                cur = i + 1;
                step *= id;
            }
        }
        if cur == buses.len() {
            break;
        }
    }

    writeln!(io::stdout(), "Part 2: {t}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(t)
}

fn parse_input(input: &str) -> Result<(Int, Vec<Int>)> {
    let lines: Vec<_> = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();
    if lines.len() != 2 {
        return err!("not a valid input");
    }
    let timestamp = lines[0].parse()?;
    let buses = lines[1]
        .split(',')
        .map(|s| s.trim().parse().unwrap_or(1))
        .collect();

    Ok((timestamp, buses))
}

#[test]
fn example_input() {
    let input = "939
    7,13,x,x,59,x,31,19";
    let (timestamp, buses) = parse_input(input).unwrap();
    assert_eq!(part1(&timestamp, &buses).unwrap(), 295);
    assert_eq!(part2(&buses).unwrap(), 1068781);

    let input = "939
    17,x,13,19";
    let (_, buses) = parse_input(input).unwrap();
    assert_eq!(part2(&buses).unwrap(), 3417);

    let input = "939
    67,7,59,61";
    let (_, buses) = parse_input(input).unwrap();
    assert_eq!(part2(&buses).unwrap(), 754018);

    let input = "939
    67,x,7,59,61";
    let (_, buses) = parse_input(input).unwrap();
    assert_eq!(part2(&buses).unwrap(), 779210);

    let input = "939
    67,7,x,59,61";
    let (_, buses) = parse_input(input).unwrap();
    assert_eq!(part2(&buses).unwrap(), 1261476);

    let input = "939
    1789,37,47,1889";
    let (_, buses) = parse_input(input).unwrap();
    assert_eq!(part2(&buses).unwrap(), 1202161486);
}
