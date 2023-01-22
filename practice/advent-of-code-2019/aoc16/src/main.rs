use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

#[allow(dead_code)]
const BASE: [i32; 4] = [0, 1, 0, -1];

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input, 100)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str, phase_count: usize) -> Result<String> {
    let start = Instant::now();

    let result = get_eight_digit_message(input, phase_count, 0)?;

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(input: &str) -> Result<String> {
    let start = Instant::now();

    let input = input.repeat(10000);
    let offset = input.chars().take(7).collect::<String>().parse()?;
    let result = get_eight_digit_message(&input, 100, offset)?;

    writeln!(io::stdout(), "Part 2: {result}")?;
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
            input[i..]
                .chunks(i + 1)
                .map(|it| it.iter().sum::<i32>())
                .step_by(2)
                .enumerate()
                .fold(0, |sum, (i, w)| if i % 2 == 0 { sum + w } else { sum - w }),
        ));
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

    assert_eq!(
        part2("03036732577212944063491565474664").unwrap(),
        "84462026".to_string()
    );
}
