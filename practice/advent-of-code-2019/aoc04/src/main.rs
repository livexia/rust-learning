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

    let (start, end) = input.split_once('-').unwrap();
    let (start, end) = (start.parse()?, end.parse()?);

    part1(start, end)?;
    part2(start, end)?;
    Ok(())
}

fn part1(start: u32, end: u32) -> Result<usize> {
    let start_time = Instant::now();

    let result = (start..=end)
        .filter(|&num| number_test(&num_to_str(num)))
        .count();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(
        io::stdout(),
        "> Time elapsed is: {:?}",
        start_time.elapsed()
    )?;
    Ok(result)
}

fn part2(start: u32, end: u32) -> Result<usize> {
    let start_time = Instant::now();

    let result = (start..=end)
        .filter(|&num| number_test2(&num_to_str(num)))
        .count();

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(
        io::stdout(),
        "> Time elapsed is: {:?}",
        start_time.elapsed()
    )?;
    Ok(result)
}

fn num_to_str(num: u32) -> String {
    format!("{num}")
}

fn number_test(num: &str) -> bool {
    let mut flag = false;
    !num.contains('0')
        && num.len() == 6
        && num.chars().zip(num.chars().skip(1)).all(|(c1, c2)| {
            if c1 == c2 {
                flag = true;
            }
            c1 <= c2 && c1.is_numeric()
        })
        && flag
}

fn number_test2(num: &str) -> bool {
    let mut flag = false;
    let mut count = 0;
    !num.contains('0')
        && num.len() == 6
        && num.chars().zip(num.chars().skip(1)).all(|(c1, c2)| {
            if c1 == c2 {
                count += 1;
            } else {
                if count == 1 {
                    flag = true;
                }
                count = 0;
            }
            c1 <= c2
        })
        && (count == 1 || flag)
}

#[test]
fn example_input() {
    assert_eq!(number_test("111111"), true);
    assert_eq!(number_test("223450"), false);
    assert_eq!(number_test("123789"), false);

    assert_eq!(number_test2("112233"), true);
    assert_eq!(number_test2("123444"), false);
    assert_eq!(number_test2("111122"), true);
}
