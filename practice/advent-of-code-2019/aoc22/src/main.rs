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
    // part2()?;
    Ok(())
}

fn part1(techniques: &[Technique]) -> Result<usize> {
    let start = Instant::now();

    let (deck, first) = shuffle(10007, techniques);
    println!("{}", show_deck(&deck, first));

    let result = 0;

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn shuffle(length: usize, techniques: &[Technique]) -> (Vec<usize>, usize) {
    let mut deck: Vec<_> = (1..length).collect(); // index: card number, value: next card
    deck.push(0);

    let mut first = 0;
    for t in techniques {
        match t {
            Technique::DealNew => {
                let mut cur = first;
                let mut next = deck[cur];

                while next != first {
                    let temp = deck[next];
                    deck[next] = cur;
                    cur = next;
                    next = temp;
                }
                deck[first] = cur;
                first = cur;
            }
            Technique::Cut(n) => {
                first = (first
                    + if n < &0 {
                        length - n.abs() as usize
                    } else {
                        *n as usize
                    })
                    % length;
            }
            Technique::DealIncrement(n) => {
                println!("n:{n}, {}", show_deck(&deck, first));
                assert!(n > &0);
                let mut new_decks = deck.clone();
                let mut cur = first;
                for i in 0..length {
                    let temp = new_decks[cur];
                    let x = (i + 1 + length - *n as usize) % length;

                    println!("{} jump {} to {}", cur, x, get_n_card(x, &new_decks, first));
                    new_decks[cur] = get_n_card(x % length, &new_decks, first) + 1;
                    cur = temp;
                }
                deck = new_decks;
                println!("new {}", show_deck(&deck, first));
            }
        }
    }
    (deck, first)
}

fn get_n_card(n: usize, deck: &[usize], first: usize) -> usize {
    let mut cur = first;
    for _ in 0..n {
        cur = deck[cur];
    }
    cur
}

fn show_deck(deck: &[usize], first: usize) -> String {
    let mut s = String::new();
    let mut cur = first;
    s.push_str(&format!("{cur} "));
    while deck[cur] != first {
        s.push_str(&format!("{} ", deck[cur]));
        cur = deck[cur];
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
    // let input = "deal with increment 7
    // deal into new stack
    // deal into new stack";
    // let ts = paesr_input(input).unwrap();
    // let (deck, first) = shuffle(10, &ts);
    // assert_eq!(&show_deck(&deck, first), "0 3 6 9 2 5 8 1 4 7");

    // let input = "deal into new stack";
    // let ts = paesr_input(input).unwrap();
    // let (deck, first) = shuffle(10, &ts);
    // assert_eq!(&show_deck(&deck, first), "9 8 7 6 5 4 3 2 1 0");

    // let input = "cut 3";
    // let ts = paesr_input(input).unwrap();
    // let (deck, first) = shuffle(10, &ts);
    // assert_eq!(&show_deck(&deck, first), "3 4 5 6 7 8 9 0 1 2");

    // let input = "cut -4";
    // let ts = paesr_input(input).unwrap();
    // let (deck, first) = shuffle(10, &ts);
    // assert_eq!(&show_deck(&deck, first), "6 7 8 9 0 1 2 3 4 5");

    // let input = "deal with increment 3";
    // let ts = paesr_input(input).unwrap();
    // let (deck, first) = shuffle(10, &ts);
    // assert_eq!(&show_deck(&deck, first), "0 7 4 1 8 5 2 9 6 3");

    let input = "deal with increment 9";
    let ts = paesr_input(input).unwrap();
    let (deck, first) = shuffle(10, &ts);
    assert_eq!(&show_deck(&deck, first), "0 9 8 7 6 5 4 3 2 1");

    // let input = "cut 6
    // deal with increment 7
    // deal into new stack";
    // let ts = paesr_input(input).unwrap();
    // let (deck, first) = shuffle(10, &ts);
    // assert_eq!(&show_deck(&deck, first), "3 0 7 4 1 8 5 2 9 6");

    let input = "deal with increment 7
    deal with increment 9
    cut -2";
    let ts = paesr_input(input).unwrap();
    let (deck, first) = shuffle(10, &ts);
    assert_eq!(&show_deck(&deck, first), "6 3 0 7 4 1 8 5 2 9");

    // let input = "deal into new stack
    // cut -2
    // deal with increment 7
    // cut 8
    // cut -4
    // deal with increment 7
    // cut 3
    // deal with increment 9
    // deal with increment 3
    // cut -1";
    // let ts = paesr_input(input).unwrap();
    // let (deck, first) = shuffle(10, &ts);
    // assert_eq!(&show_deck(&deck, first), "9 2 5 8 1 4 7 0 3 6");
}
