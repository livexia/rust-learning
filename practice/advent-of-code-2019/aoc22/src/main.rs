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

    let deck = shuffle(10007, techniques, 1);

    let result = deck[2019];

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(techniques: &[Technique]) -> Result<usize> {
    let start = Instant::now();

    // let deck = shuffle(119315717514047, techniques, 101741582076661);

    // let v = vec![0; 119315717514047];

    let deck = shuffle(10009, techniques, 101741582076661);

    let result = 0;

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn shuffle(length: usize, techniques: &[Technique], times: usize) -> Vec<usize> {
    let mut deck: Vec<_> = (0..length).collect(); // index: card number, value: card index

    let mut visited = std::collections::HashMap::new();
    let mut loop_size = usize::MAX;

    for _i in 0..times {
        for t in techniques {
            match t {
                Technique::DealNew => {
                    for i in deck.iter_mut() {
                        *i = length - 1 - *i;
                    }
                }
                Technique::Cut(n) => {
                    let index = if n < &0 {
                        length - n.abs() as usize
                    } else {
                        *n as usize
                    };
                    for i in 0..length {
                        if deck[i] < index {
                            deck[i] += length - index
                        } else {
                            deck[i] -= index
                        }
                    }
                }
                Technique::DealIncrement(n) => {
                    for i in 0..length {
                        deck[i] = (deck[i] * *n as usize) % length;
                    }
                }
            }
        }
        // println!("{:?}", deck);
        let v = deck.iter().position(|i| i == &5752).unwrap();
        if let Some(prev) = visited.get(&v) {
            println!("{}: {} {}, {}", v, prev, _i, _i - prev);
            loop_size = _i - prev;
            break;
        }
        visited.insert(v, _i);
    }
    dbg!(visited.iter().find(|(_, &i)| i == times % loop_size));

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
    let input = "deal with increment 7
    deal into new stack
    deal into new stack";
    let ts = paesr_input(input).unwrap();
    let deck = shuffle(10, &ts, 1);
    assert_eq!(&show_deck(&deck), "0 3 6 9 2 5 8 1 4 7");

    let input = "deal into new stack";
    let ts = paesr_input(input).unwrap();
    let deck = shuffle(10, &ts, 1);
    assert_eq!(&show_deck(&deck), "9 8 7 6 5 4 3 2 1 0");

    let input = "cut 3";
    let ts = paesr_input(input).unwrap();
    let deck = shuffle(10, &ts, 1);
    assert_eq!(&show_deck(&deck), "3 4 5 6 7 8 9 0 1 2");

    let input = "cut -4";
    let ts = paesr_input(input).unwrap();
    let deck = shuffle(10, &ts, 1);
    assert_eq!(&show_deck(&deck), "6 7 8 9 0 1 2 3 4 5");

    let input = "deal with increment 3";
    let ts = paesr_input(input).unwrap();
    let deck = shuffle(10, &ts, 1);
    assert_eq!(&show_deck(&deck), "0 7 4 1 8 5 2 9 6 3");

    let input = "deal with increment 9";
    let ts = paesr_input(input).unwrap();
    let deck = shuffle(10, &ts, 1);
    assert_eq!(&show_deck(&deck), "0 9 8 7 6 5 4 3 2 1");

    let input = "cut 6
    deal with increment 7
    deal into new stack";
    let ts = paesr_input(input).unwrap();
    let deck = shuffle(10, &ts, 1);
    assert_eq!(&show_deck(&deck), "3 0 7 4 1 8 5 2 9 6");

    let input = "deal with increment 7
    deal with increment 9
    cut -2";
    let ts = paesr_input(input).unwrap();
    let deck = shuffle(10, &ts, 1);
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
    let deck = shuffle(10, &ts, 1);
    assert_eq!(&show_deck(&deck), "9 2 5 8 1 4 7 0 3 6");
}
