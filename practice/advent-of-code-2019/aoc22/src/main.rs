use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let ts = paesr_input(&input)?;

    part1(&ts)?;
    part2(&ts)?;
    Ok(())
}

fn part1(techniques: &[Technique]) -> Result<usize> {
    let start = Instant::now();

    let mut deck = [2019];
    shuffle(&mut deck, 10007, techniques);

    let result = deck[0];
    let (offset, increment) = get_offset_and_increment(techniques, 10007);

    assert_eq!(get_nth_card(result as i128, offset, increment, 10007), 2019);

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

// part 2 https://old.reddit.com/r/adventofcode/comments/ee0rqi/2019_day_22_solutions/fbnkaju/
fn part2(techniques: &[Technique]) -> Result<i128> {
    let start = Instant::now();

    let length = 119315717514047;
    let times = 101741582076661;

    let (offset_diff, increment_mul) = get_offset_and_increment(techniques, length);
    let (offset, increment) = clac_for_iterations(times, offset_diff, increment_mul, length);

    let result = get_nth_card(2020, offset, increment, length);

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn get_nth_card(n: i128, offset: i128, increment: i128, length: i128) -> i128 {
    (offset + increment * n).rem_euclid(length)
}

fn clac_for_iterations(
    times: i128,
    offset_diff: i128,
    increment_mul: i128,
    length: i128,
) -> (i128, i128) {
    let increment = modular_pow(increment_mul, times, length);
    let offset = ((offset_diff * (1 - increment)).rem_euclid(length)
        * inv((1 - increment_mul).rem_euclid(length), length))
    .rem_euclid(length);
    (offset, increment)
}

fn get_offset_and_increment(techniques: &[Technique], length: i128) -> (i128, i128) {
    let (mut offset, mut increment): (i128, i128) = (0, 1);
    for t in techniques {
        match t {
            Technique::DealNew => {
                increment *= -1;
                offset += increment;
            }
            Technique::Cut(n) => {
                offset += *n as i128 * increment;
            }
            Technique::DealIncrement(n) => {
                increment *= inv(*n as i128, length);
            }
        }
        increment = increment.rem_euclid(length);
        offset = offset.rem_euclid(length);
    }
    (offset, increment)
}

// https://en.wikipedia.org/wiki/Modular_exponentiation#Right-to-left_binary_method
fn modular_pow(mut base: i128, mut exponent: i128, modulus: i128) -> i128 {
    if modulus == 1 {
        return 0;
    }
    let _ = (modulus - 1).checked_mul(modulus - 1);
    let mut result = 1;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base).rem_euclid(modulus);
        }
        exponent >>= 1;
        base = (base * base).rem_euclid(modulus);
    }
    result
}

fn inv(n: i128, length: i128) -> i128 {
    modular_pow(n, length - 2, length)
}

#[allow(dead_code)]
fn reverse_shuffle(mut dest: usize, length: usize, techniques: &[Technique]) -> usize {
    for t in techniques.iter().rev() {
        match t {
            Technique::DealNew => {
                dest = length - 1 - dest;
            }
            Technique::Cut(n) => {
                let offset = ((*n as i128).rem_euclid(length as i128)) as usize;
                dest = (dest + offset) % length;
            }
            Technique::DealIncrement(n) => {
                let n = *n as usize;
                let mut temp = dest;
                while temp % n != 0 {
                    temp += length;
                }
                dest = (temp / n) % length;
            }
        }
    }
    dest
}

fn shuffle(deck: &mut [usize], length: usize, techniques: &[Technique]) {
    for t in techniques {
        match t {
            Technique::DealNew => {
                for i in deck.iter_mut() {
                    *i = length - 1 - *i;
                }
            }
            Technique::Cut(n) => {
                let offset = length - ((*n as i128).rem_euclid(length as i128)) as usize;
                for i in deck.iter_mut() {
                    *i = (*i + offset) % length;
                }
            }
            Technique::DealIncrement(n) => {
                for i in deck.iter_mut() {
                    *i = (*i * *n as usize) % length;
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Technique {
    DealNew,
    Cut(i32),
    DealIncrement(i32),
}

impl FromStr for Technique {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        if s.trim() == "deal into new stack" {
            Ok(Self::DealNew)
        } else if let Some(n) = s.trim().strip_prefix("cut ") {
            Ok(Self::Cut(n.trim().parse()?))
        } else if let Some(n) = s.trim().strip_prefix("deal with increment ") {
            Ok(Self::DealIncrement(n.trim().parse()?))
        } else {
            err!("Not a valid technique: {s}")
        }
    }
}

fn paesr_input(input: &str) -> Result<Vec<Technique>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse())
        .collect::<Result<Vec<Technique>>>()
}

#[test]
fn example_input() {
    fn part1_helper(length: usize, techniques: &[Technique]) -> Vec<usize> {
        let mut deck: Vec<usize> = (0..length).collect();
        shuffle(&mut deck, length, techniques);
        deck
    }

    fn show_deck(deck: &[usize]) -> String {
        let mut s = String::new();
        let mut deck: Vec<(usize, &usize)> = deck.iter().enumerate().collect();
        deck.sort_by_key(|(_, &i)| i);
        for (i, _) in deck {
            s.push_str(&format!("{i} "));
        }

        s.trim().to_string()
    }

    let input = "deal with increment 7
    deal into new stack
    deal into new stack";
    let ts = paesr_input(input).unwrap();
    let deck = part1_helper(10, &ts);
    assert_eq!(&show_deck(&deck), "0 3 6 9 2 5 8 1 4 7");

    let input = "deal into new stack";
    let ts = paesr_input(input).unwrap();
    let deck = part1_helper(10, &ts);
    assert_eq!(&show_deck(&deck), "9 8 7 6 5 4 3 2 1 0");

    let input = "cut 3";
    let ts = paesr_input(input).unwrap();
    let deck = part1_helper(10, &ts);
    assert_eq!(&show_deck(&deck), "3 4 5 6 7 8 9 0 1 2");

    let input = "cut -4";
    let ts = paesr_input(input).unwrap();
    let deck = part1_helper(10, &ts);
    assert_eq!(&show_deck(&deck), "6 7 8 9 0 1 2 3 4 5");

    let input = "deal with increment 3";
    let ts = paesr_input(input).unwrap();
    let deck = part1_helper(10, &ts);
    assert_eq!(&show_deck(&deck), "0 7 4 1 8 5 2 9 6 3");

    let input = "deal with increment 9";
    let ts = paesr_input(input).unwrap();
    let deck = part1_helper(10, &ts);
    assert_eq!(&show_deck(&deck), "0 9 8 7 6 5 4 3 2 1");

    let input = "cut 6
    deal with increment 7
    deal into new stack";
    let ts = paesr_input(input).unwrap();
    let deck = part1_helper(10, &ts);
    assert_eq!(&show_deck(&deck), "3 0 7 4 1 8 5 2 9 6");

    let input = "deal with increment 7
    deal with increment 9
    cut -2";
    let ts = paesr_input(input).unwrap();
    let deck = part1_helper(10, &ts);
    assert_eq!(&show_deck(&deck), "6 3 0 7 4 1 8 5 2 9");

    let input = "deal into new stack
    cut -2
    deal with increment 7
    cut 8
    cut -4
    deal with increment 7
    cut 3
    deal with increment 9
    deal with increment 3
    cut -1";
    let ts = paesr_input(input).unwrap();
    let deck = part1_helper(10, &ts);
    assert_eq!(&show_deck(&deck), "9 2 5 8 1 4 7 0 3 6");
}
