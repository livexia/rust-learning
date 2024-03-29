use std::collections::HashMap;
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
    let moves = parse_input(&input);

    part1(&moves, 16)?;
    part2(&moves, 16)?;
    Ok(())
}

fn part1(moves: &[Dance], length: usize) -> Result<String> {
    let start = Instant::now();

    let mut programs: Vec<_> = (0..length).collect();
    let mut offset = 0;
    dance_with_moves(&mut programs, &mut offset, length, moves);
    let result = reoder(&programs, offset, length);

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(moves: &[Dance], length: usize) -> Result<String> {
    let start = Instant::now();

    let mut programs: Vec<_> = (0..length).collect();
    let mut offset = 0;

    let mut visited = HashMap::new();
    visited.insert(programs.clone(), 0);
    let mut loop_size = 0;

    for t in 0..1_000_000_000 {
        dance_with_moves(&mut programs, &mut offset, length, moves);
        if visited.contains_key(&programs) {
            loop_size = t - visited.get(&programs).unwrap();
            break;
        }
        visited.insert(programs.clone(), t);
    }
    for _ in 0..1_000_000_000 % loop_size {
        dance_with_moves(&mut programs, &mut offset, length, moves);
    }
    let result = reoder(&programs, offset, length);

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn reoder(programs: &[usize], offset: usize, length: usize) -> String {
    let mut list: Vec<_> = programs.iter().enumerate().map(|(i, a)| (a, i)).collect();
    list.sort();
    list.iter()
        .cycle()
        .skip(offset)
        .take(length)
        .map(|(_, b)| (*b as u8 + b'a') as char)
        .collect()
}

fn dance_with_moves(programs: &mut [usize], offset: &mut usize, length: usize, moves: &[Dance]) {
    for m in moves {
        m.dance(programs, offset, length);
    }
}

#[derive(Debug)]
enum Dance {
    Spin(usize),
    Exchange((usize, usize)),
    Partner((usize, usize)),
}

impl Dance {
    fn dance(&self, programs: &mut [usize], offset: &mut usize, length: usize) {
        match self {
            Dance::Spin(x) => *offset = (*offset + length - x) % length,
            Dance::Exchange((i, j)) => {
                let a = programs
                    .iter()
                    .position(|k| (k + length - *offset) % length == *i)
                    .unwrap();
                let b = programs
                    .iter()
                    .position(|k| (k + length - *offset) % length == *j)
                    .unwrap();
                programs.swap(a, b);
            }
            Dance::Partner((a, b)) => programs.swap(*a, *b),
        }
    }
}

fn parse_input(input: &str) -> Vec<Dance> {
    let mut moves = vec![];
    for dance in input.trim().split(',') {
        if dance.trim().is_empty() {
            continue;
        }
        let bytes: Vec<_> = dance.trim().bytes().collect();
        if b's' == bytes[0] {
            let mut a = 0usize;
            for &i in &bytes[1..] {
                a *= 10;
                a += (i - b'0') as usize;
            }
            moves.push(Dance::Spin(a));
        } else if b'x' == bytes[0] {
            let mut a = 0;
            let mut b = 0;
            let mut flag = true;
            for &i in &bytes[1..] {
                if i == b'/' {
                    flag = false;
                } else if flag {
                    a *= 10;
                    a += (i - b'0') as usize;
                } else if !flag {
                    b *= 10;
                    b += (i - b'0') as usize;
                }
            }
            moves.push(Dance::Exchange((a, b)));
        } else if b'p' == bytes[0] {
            let a = bytes[1] - b'a';
            let b = bytes[3] - b'a';
            moves.push(Dance::Partner((a as usize, b as usize)));
        } else {
            unimplemented!("Wrong move: {dance}");
        }
    }
    moves
}

#[test]
fn example_input() {
    let input = "s1,x3/4,pe/b";
    let moves = parse_input(input);
    assert_eq!(&part1(&moves, 5).unwrap(), "baedc");
}
