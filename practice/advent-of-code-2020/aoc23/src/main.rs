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
    part2(&cups)?;
    Ok(())
}

fn part1(cups: &[u32]) -> Result<String> {
    let start = Instant::now();

    let mut cups = cups.to_owned();
    nth_moves(&mut cups, 100);
    let result = order_after_one(&cups);

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(cups: &[u32]) -> Result<u64> {
    let start = Instant::now();

    let times = 1000000;

    let mut cups: Vec<u32> = cups.iter().map(|&n| n as u32).collect();
    let max = cups.iter().max().unwrap() + 1;
    cups.extend(max..=times);
    nth_moves(&mut cups, times);

    let one_pos = cups.iter().position(|&n| n == 1).unwrap();
    let r1 = cups[(one_pos + 1) % times as usize];
    let r2 = cups[(one_pos + 2) % times as usize];
    dbg!(r1, r2);
    let result = r1 as u64 * r2 as u64;

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn nth_moves(cups: &mut Vec<u32>, times: u32) {
    let &min = cups.iter().min().unwrap();
    let &max = cups.iter().max().unwrap();
    for t in 0..times {
        if t % 10000 == 0 {
            println!("t: {}/100", t / 10000);
        }
        move_cup(cups, min, max);
    }
}

fn move_cup(cups: &mut Vec<u32>, min: u32, max: u32) {
    let cur = cups.remove(0);
    let pick_up = [cups.remove(0), cups.remove(0), cups.remove(0)];
    let dest = dest(cur, &pick_up, min, max);
    let dest_pos = cups.iter().position(|&n| n == dest).unwrap() + 1;
    for (offset, n) in pick_up.into_iter().enumerate() {
        cups.insert(dest_pos + offset, n);
    }
    cups.push(cur);
}

fn dest(cur: u32, pick_up: &[u32; 3], min: u32, max: u32) -> u32 {
    let mut cur = cur;
    loop {
        cur = if cur - 1 < min { max } else { cur - 1 };
        if !pick_up.contains(&cur) {
            return cur;
        }
    }
}

fn order_after_one(cups: &[u32]) -> String {
    let mut s = String::new();
    let mut found = 0;
    for &n in cups.iter().cycle() {
        if n == 1 {
            found += 1;
            if found == 2 {
                break;
            }
        } else if found == 1 {
            s.push((n as u8 + b'0') as char)
        }
    }
    s
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .trim()
        .chars()
        .map(|c| (c as u8 - b'0') as u32)
        .collect()
}

#[test]
fn example_input() {
    let input = "389125467";
    let cups = parse_input(&input);

    let mut test = cups.clone();
    nth_moves(&mut test, 10);
    println!("{:?}", test);
    println!("{:?}", pos(&test, 6, 10));
    assert_eq!("92658374", order_after_one(&test));

    assert_eq!("67384529", part1(&cups).unwrap());

    assert_eq!(149245887792, part2(&cups).unwrap());
}
