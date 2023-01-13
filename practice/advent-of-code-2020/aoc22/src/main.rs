use std::collections::VecDeque;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Deck = VecDeque<usize>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let (player_one, player_two) = parse_input(&input)?;

    part1(player_one.clone(), player_two.clone())?;
    // part2()?;
    Ok(())
}

fn part1(mut player_one: Deck, mut player_two: Deck) -> Result<usize> {
    let start = Instant::now();

    while !player_one.is_empty() && !player_two.is_empty() {
        let a = player_one.pop_back().unwrap();
        let b = player_two.pop_back().unwrap();
        if a > b {
            player_one.push_front(a);
            player_one.push_front(b);
        } else {
            player_two.push_front(b);
            player_two.push_front(a);
        }
    }

    let result = player_one
        .iter()
        .enumerate()
        .fold(0, |sum, (i, n)| sum + (i + 1) * n)
        + player_two
            .iter()
            .enumerate()
            .fold(0, |sum, (i, n)| sum + (i + 1) * n);

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn parse_input(input: &str) -> Result<(Deck, Deck)> {
    let mut players = (Deck::new(), Deck::new());
    let mut cur = &mut players.0;
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        } else if line.trim().starts_with("Player 1") {
            cur = &mut players.0;
        } else if line.trim().starts_with("Player 2") {
            cur = &mut players.1;
        } else {
            cur.push_front(line.trim().parse()?);
        }
    }
    Ok(players)
}

#[test]
fn example_input() {
    let input = "Player 1:
    9
    2
    6
    3
    1
    
    Player 2:
    5
    8
    4
    7
    10";

    let (player_one, player_two) = parse_input(input).unwrap();
    assert_eq!(part1(player_one.clone(), player_two.clone()).unwrap(), 306);
}
