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
    let cups = parse_input(&input);

    part1(&cups)?;
    // part2()?;
    Ok(())
}

fn part1(cups: &[u8]) -> Result<String> {
    let start = Instant::now();

    let result = order_after_one(&nth_moves(cups, 100));

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn nth_moves(cups: &[u8], times: usize) -> Vec<u8> {
    let &min = cups.iter().min().unwrap();
    let &max = cups.iter().max().unwrap();
    let mut cups = cups.to_owned();
    for _ in 0..times {
        cups = move_cup(&cups, min, max);
    }
    cups
}

fn move_cup(cups: &[u8], min: u8, max: u8) -> Vec<u8> {
    let cur = cups[0];
    let pick_up = [cups[1], cups[2], cups[3]];
    let dest = dest(cups, min, max);
    let mut new_cups = vec![];
    for &next in &cups[4..] {
        new_cups.push(next);
        if next == dest {
            new_cups.extend_from_slice(&pick_up);
        }
    }
    new_cups.push(cur);
    new_cups
}

fn dest(cups: &[u8], min: u8, max: u8) -> u8 {
    let mut cur = cups[0];
    let pick_up = [cups[1], cups[2], cups[3]];
    loop {
        cur = if cur - 1 < min { max } else { cur - 1 };
        if !pick_up.contains(&cur) {
            return cur;
        }
    }
}

fn order_after_one(cups: &[u8]) -> String {
    let mut s = String::new();
    let mut found = 0;
    for &n in cups.iter().cycle() {
        if n == 1 {
            found += 1;
            if found == 2 {
                break;
            }
        } else if found == 1 {
            s.push((n + b'0') as char)
        }
    }
    s
}

fn parse_input(input: &str) -> Vec<u8> {
    input.trim().chars().map(|c| c as u8 - b'0').collect()
}

#[test]
fn example_input() {
    let input = "389125467";
    let cups = parse_input(&input);

    assert_eq!(vec![5, 7, 4, 1, 8, 3, 9, 2, 6], nth_moves(&cups, 9));
    assert_eq!("92658374", order_after_one(&nth_moves(&cups, 10)));
    assert_eq!("67384529", order_after_one(&nth_moves(&cups, 100)));

    // part1(&cups).unwrap();
}
