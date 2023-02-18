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
    let input = parse_input(&input);

    part1(256, &input)?;
    // part2()?;
    Ok(())
}

fn part1(list_size: usize, input: &[usize]) -> Result<usize> {
    let start = Instant::now();

    let list = knot_hash(list_size, input);
    let result =
        list.iter().position(|&i| i == 0).unwrap() * list.iter().position(|&i| i == 1).unwrap();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn knot_hash(list_size: usize, input: &[usize]) -> Vec<usize> {
    let mut list: Vec<_> = (0..list_size).collect();
    let mut current_position = 0;
    for (skip_size, length) in input.iter().enumerate() {
        // reverse
        for item in list.iter_mut() {
            if (current_position..current_position + length).contains(item)
                || (current_position + length >= list_size
                    && (0..(current_position + length) % list_size).contains(item))
            {
                let offset = ((*item + list_size - current_position) % list_size) * 2 + 1;
                *item = (*item + list_size + length - offset) % list_size;
            }
        }
        // update current_position
        current_position = (current_position + length + skip_size) % list_size;
        // update skip_size with loop
    }
    list
}

#[allow(dead_code)]
fn debug_list(list: &[usize]) {
    let mut l: Vec<(usize, usize)> = list.iter().copied().enumerate().collect();
    l.sort_by_key(|&(_, index)| index);
    let l = l.iter().map(|(i, _)| i).collect::<Vec<_>>();
    println!("{:?}", l);
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect()
}

#[test]
fn example_input() {
    let input = "3, 4, 1, 5";
    assert_eq!(part1(5, &parse_input(input)).unwrap(), 12);
}
