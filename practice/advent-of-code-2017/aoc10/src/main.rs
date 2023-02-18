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

    part1(256, &input)?;
    part2(256, &input)?;
    Ok(())
}

fn part1(list_size: usize, input: &str) -> Result<usize> {
    let start = Instant::now();

    let input = parse_input(input);
    let mut list: Vec<_> = (0..list_size).collect();
    knot_hash(&mut list, &mut 0, &mut 0, &input);
    // let result =
    // list.iter().position(|&i| i == 0).unwrap() * list.iter().position(|&i| i == 1).unwrap();
    let result = list[0] * list[1];

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(list_size: usize, input: &str) -> Result<String> {
    let start = Instant::now();

    let input = parse_input_with_ascii(input);
    let mut list: Vec<usize> = (0..list_size).collect();
    let mut current_position = 0;
    let mut skip_size = 0;
    for _ in 0..64 {
        knot_hash(&mut list, &mut current_position, &mut skip_size, &input);
    }

    // let list = list_with_order(&list);

    let mut result = String::new();
    for i in (0..256).step_by(16) {
        result.push_str(&format!(
            "{:02x}",
            list[i..i + 16].iter().skip(1).fold(list[i], |r, b| r ^ b)
        ));
    }

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn knot_hash(
    list: &mut [usize],
    current_position: &mut usize,
    skip_size: &mut usize,
    input: &[usize],
) {
    let list_size = list.len();
    for length in input {
        // reverse with value as list index and vec index as list value, this method is slow
        // for item in list.iter_mut() {
        // if (*current_position..*current_position + length).contains(item)
        // || (*current_position + length >= list_size
        // && (0..(*current_position + length) % list_size).contains(item))
        // {
        // let offset = ((*item + list_size - *current_position) % list_size) * 2 + 1;
        // *item = (*item + list_size + length - offset) % list_size;
        // }
        // }
        for i in 0..length / 2 {
            list.swap(
                (*current_position + i) % list_size,
                (*current_position + length - 1 - i) % list_size,
            );
        }
        // update current_position
        *current_position = (*current_position + length + *skip_size) % list_size;
        // update skip_size
        *skip_size += 1;
    }
}

#[allow(dead_code)]
fn list_with_order(list: &[usize]) -> Vec<usize> {
    // Vec index as list value, and Vec value as list index, vec to list
    let mut l: Vec<(usize, usize)> = list.iter().copied().enumerate().collect();
    l.sort_by_key(|&(_, index)| index);
    l.into_iter().map(|(i, _)| i).collect()
}

fn parse_input_with_ascii(input: &str) -> Vec<usize> {
    input
        .trim()
        .bytes()
        .chain([17, 31, 73, 47, 23])
        .map(|b| b as usize)
        .collect()
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
    assert_eq!(part1(5, input).unwrap(), 12);
    assert_eq!(
        &part2(256, "1,2,3").unwrap(),
        "3efbe78a8d82f29979031a4aa0b16a9d"
    );
    assert_eq!(
        &part2(256, "1,2,4").unwrap(),
        "63960835bcdc130f0b66d7ff4f6a5a8e"
    );
    assert_eq!(
        &part2(256, "AoC 2017").unwrap(),
        "33efeb34ea91902bb2f59c9920caa6cd"
    );
    assert_eq!(&part2(256, "").unwrap(), "a2582a3a0e66e6e86e3812dcb672a272");
}
