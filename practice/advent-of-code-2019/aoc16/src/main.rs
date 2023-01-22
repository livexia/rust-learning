use std::error::Error;
use std::io::{self, Read, Write};
use std::iter::repeat;
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

const BASE: [i32; 4] = [0, 1, 0, -1];

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input, 100)?;
    // part2()?;
    Ok(())
}

fn part1(input: &str, phase_count: usize) -> Result<String> {
    let start = Instant::now();

    let result = get_eight_digit_message(input, phase_count, 0)?;

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn get_eight_digit_message(input: &str, phase_count: usize, offset: usize) -> Result<String> {
    let mut input = str_to_int(input);
    for _ in 0..phase_count {
        input = fft(&input);
    }
    int_to_str(&input[offset..offset + 8])
}

fn fft(input: &[i32]) -> Vec<i32> {
    let mut v = vec![];
    for i in 0..input.len() {
        v.push(ones_digit(
            input
                .iter()
                .zip(
                    BASE.into_iter()
                        .flat_map(|p| repeat(p).take(i + 1))
                        .cycle()
                        .skip(1),
                )
                .map(|(n, p)| n * p)
                .sum(),
        ))
    }
    v
}

fn ones_digit(n: i32) -> i32 {
    n.abs() % 10
}

fn str_to_int(s: &str) -> Vec<i32> {
    s.bytes().map(|b| (b - b'0') as i32).collect()
}

fn int_to_str(v: &[i32]) -> Result<String> {
    let mut s = String::new();
    for &n in v {
        if n == 0 && s.is_empty() {
            continue;
        }
        if (0..=9).contains(&n) {
            s.push_str(&format!("{n}"))
        } else {
            return err!("not possible to convert a vec of int to str: {:?}", v);
        }
    }
    Ok(s)
}

#[test]
fn example_input() {
    assert_eq!(ones_digit(-17), 7);
    assert_eq!(ones_digit(38), 8);

    assert_eq!(part1("12345678", 4).unwrap(), "1029498".to_string());

    assert_eq!(
        part1("80871224585914546619083218645595", 100).unwrap(),
        "24176176".to_string()
    );
}
