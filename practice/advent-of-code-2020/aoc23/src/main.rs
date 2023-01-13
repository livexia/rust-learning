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
    let (cups, cur, end) = parse_input(&input);

    part1(&cups, cur)?;
    part2(&cups, cur, end)?;
    Ok(())
}

fn part1(cups: &[usize], cur: usize) -> Result<String> {
    let start = Instant::now();

    let mut cups = cups.to_owned();
    nth_moves(&mut cups, cur, 100);
    let result = order_after_one(&cups);

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(cups: &[usize], cur: usize, end: usize) -> Result<usize> {
    let start = Instant::now();

    let length = 1000000;
    let times = length * 10;

    let mut cups = cups.to_owned();
    expand_cups(&mut cups, cur, end, length);

    nth_moves(&mut cups, cur, times);

    let cup1 = cups[1];
    let cup2 = cups[cup1];
    let result = cup1 * cup2;

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn nth_moves(cups: &mut [usize], cur: usize, times: usize) {
    let min = 1;
    let max = cups.len() - 1;
    let mut cur = cur;
    for _ in 0..times {
        cur = move_cup(cups, cur, min, max);
    }
}

fn move_cup(cups: &mut [usize], cur: usize, min: usize, max: usize) -> usize {
    let pick_up = [cups[cur], cups[cups[cur]], cups[cups[cups[cur]]]];
    let dest = dest(cur, &pick_up, min, max);
    cups[cur] = cups[pick_up[2]]; // cur -> p[0] -> p[1] -> p[2] -> next
    cups[pick_up[2]] = cups[dest]; // p[2] -> dest -> next
    cups[dest] = pick_up[0]; // dest -> p[0]
    cups[cur]
}

fn dest(start: usize, pick_up: &[usize; 3], min: usize, max: usize) -> usize {
    let mut cur = start;
    while cur == start || pick_up[0] == cur || pick_up[1] == cur || pick_up[2] == cur {
        cur = if cur - 1 < min { max } else { cur - 1 };
    }
    cur
}

fn order_after_one(cups: &[usize]) -> String {
    let mut cur = cups[1];
    let mut s = String::new();
    while cur != 1 {
        s.push_str(&format!("{}", cur));
        cur = cups[cur];
    }
    s
}

fn parse_input(input: &str) -> (Vec<usize>, usize, usize) {
    let mut cups: Vec<usize> = (0..=input.trim().len()).collect();
    let input: Vec<_> = input
        .trim()
        .chars()
        .map(|c| (c as u8 - b'0') as usize)
        .collect();
    for i in 0..input.len() - 1 {
        cups[input[i]] = input[i + 1];
    }
    cups[input[input.len() - 1]] = input[0];
    (cups, input[0], input[input.len() - 1])
}

fn expand_cups(cups: &mut Vec<usize>, start: usize, end: usize, length: usize) {
    let max = cups.len();
    cups.reserve_exact(length + 1);
    cups[end] = max;
    cups.extend(max + 1..=length + 1);
    cups[length] = start;
}

#[allow(dead_code)]
fn pretty_cups(cups: &[usize], start: usize) -> Vec<usize> {
    let mut result = vec![start];
    let mut cur = cups[start];
    while cur != start {
        result.push(cur);
        cur = cups[cur];
    }
    result
}

#[test]
fn example_input() {
    let input = "389125467";
    let (cups, cur, end) = parse_input(&input);

    let mut test = cups.clone();
    nth_moves(&mut test, cur, 10);
    assert_eq!("92658374", order_after_one(&test));

    assert_eq!("67384529", part1(&cups, cur).unwrap());

    assert_eq!(149245887792, part2(&cups, cur, end).unwrap());
}
