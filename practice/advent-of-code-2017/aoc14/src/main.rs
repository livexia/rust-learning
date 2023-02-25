use std::collections::VecDeque;
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

fn part1(list_size: u16, input: &str) -> Result<u32> {
    let start = Instant::now();

    let result = grid_key(input)
        .iter()
        .map(|k| knot_hash(list_size, k).count_ones())
        .sum();
    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(list_size: u16, input: &str) -> Result<u32> {
    let start = Instant::now();

    let mut grid: Vec<_> = grid_key(input)
        .iter()
        .map(|k| knot_hash(list_size, k))
        .collect();

    let mut result = 0;
    for x in 0..128 {
        for y in 0..128 {
            if grid[x] & (1 << y) != 0 {
                result += 1;
                dfs(&mut grid, x, y);
                // bfs(&mut grid, x, y);
            }
        }
    }
    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn dfs(grid: &mut [u128], x: usize, y: usize) {
    grid[x] &= !(1 << y); // set searched location with zero, avoid dead loop
    if x > 0 && grid[x - 1] & (1 << y) != 0 {
        dfs(grid, x - 1, y);
    }
    if x + 1 < 128 && grid[x + 1] & (1 << y) != 0 {
        dfs(grid, x + 1, y);
    }
    if y > 0 && grid[x] & (1 << (y - 1)) != 0 {
        dfs(grid, x, y - 1);
    }
    if y + 1 < 128 && grid[x] & (1 << (y + 1)) != 0 {
        dfs(grid, x, y + 1)
    }
}

#[allow(dead_code)]
fn bfs(grid: &mut [u128], x: usize, y: usize) {
    let mut queue = VecDeque::new();
    queue.push_back((x, y));
    while let Some((x, y)) = queue.pop_front() {
        grid[x] &= !(1 << y); // set searched location with zero, avoid dead loop
        if x > 0 && grid[x - 1] & (1 << y) != 0 {
            queue.push_back((x - 1, y));
        }
        if x + 1 < 128 && grid[x + 1] & (1 << y) != 0 {
            queue.push_back((x + 1, y));
        }
        if y > 0 && grid[x] & (1 << (y - 1)) != 0 {
            queue.push_back((x, y - 1));
        }
        if y + 1 < 128 && grid[x] & (1 << (y + 1)) != 0 {
            queue.push_back((x, y + 1));
        }
    }
}

fn knot_hash(list_size: u16, key: &[u8]) -> u128 {
    let mut list: Vec<_> = (0..list_size).collect();
    let mut current_position = 0;
    let mut skip_size = 0;
    for _ in 0..64 {
        for &length in key {
            let length = length as u16;
            for i in 0..length / 2 {
                list.swap(
                    ((current_position + i) % list_size) as usize,
                    ((current_position + length - 1 - i) % list_size) as usize,
                );
            }
            current_position = (current_position + length + skip_size) % list_size;
            skip_size += 1;
        }
    }

    let mut result = 0;
    for part in list.chunks(16) {
        result <<= 8;
        result |= part.iter().fold(0, |r, b| r ^ b) as u128;
    }

    result
}

fn grid_key(input: &str) -> Vec<Vec<u8>> {
    let mut r = Vec::with_capacity(128);
    let input: Vec<u8> = input.trim().bytes().collect();
    for i in 0..128 {
        let mut input = input.clone();
        input.push(b'-');
        input.extend(i.to_string().bytes());
        input.extend_from_slice(&[17, 31, 73, 47, 23]);
        r.push(input)
    }
    r
}

#[test]
fn example_input() {
    let input = "flqrgnkx";
    assert_eq!(part1(256, input).unwrap(), 8108);
    assert_eq!(part2(256, input).unwrap(), 1242);
    assert_eq!(part1(256, "jzgqcdpd").unwrap(), 8074);
    assert_eq!(part2(256, "jzgqcdpd").unwrap(), 1212);
}
