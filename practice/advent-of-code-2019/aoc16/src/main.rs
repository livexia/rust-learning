use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Int = i32;

#[allow(dead_code)]
const BASE: [Int; 4] = [0, 1, 0, -1];

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

    let offset = input.chars().take(7).collect::<String>().parse()?;
    let input = input.repeat(10000);
    let result = get_eight_digit_message(&input, 100, offset)?;

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn get_eight_digit_message(input: &str, phase_count: usize, offset: usize) -> Result<String> {
    let mut input = str_to_int(input);
    if offset * 3 > input.len() {
        // start from offset, only pattern with 1 will apply
        let mut sum = input[offset..(offset * 2 + 1).min(input.len())]
            .iter()
            .sum();
        for _ in 0..phase_count {
            sum = simplify_fft(&mut input, offset, sum);
        }
    } else {
        for _ in 0..phase_count {
            fft(&mut input, offset);
        }
    }
    int_to_str(&input[offset..offset + 8])
}

fn fft(input: &mut [Int], offset: usize) {
    for i in offset..input.len() {
        input[i] = ones_digit(
            input[i..]
                .chunks(i + 1)
                .step_by(2)
                .zip((-1..=1).step_by(2).cycle())
                .map(|(it, i)| i * it.iter().sum::<Int>())
                .sum::<Int>(),
        );
    }
}

fn simplify_fft(input: &mut [Int], offset: usize, mut cur_sum: Int) -> Int {
    if input.len() > offset * 3 - 1 {
        unimplemented!("unimplemented simply fft for input length big than offset * 3 - 1")
    }
    let length = (2 * offset + 1).min(input.len());

    let mut i = offset;
    let mut temp = input[i];
    input[i] = ones_digit(cur_sum);
    let mut next_sum = input[i];
    while i + 1 < length {
        cur_sum -= temp;
        if i + offset + 1 < length {
            cur_sum += input[i + offset + 1];
            if i + offset + 2 < length {
                cur_sum += input[i + offset + 2]
            }
        }
        temp = input[i + 1];
        input[i + 1] = ones_digit(cur_sum);
        next_sum += input[i + 1];
        i += 1;
    }
    next_sum
}

fn ones_digit(n: Int) -> Int {
    (n % 10).abs()
}

fn str_to_int(s: &str) -> Vec<Int> {
    s.bytes().map(|b| (b - b'0') as Int).collect()
}

fn int_to_str(v: &[Int]) -> Result<String> {
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
