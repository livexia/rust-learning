use std::collections::hash_map::DefaultHasher;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::hash::{Hash, Hasher};
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
    part2(player_one, player_two)?;
    Ok(())
}

fn part1(mut player_one: Deck, mut player_two: Deck) -> Result<usize> {
    let start = Instant::now();

    while !player_one.is_empty() && !player_two.is_empty() {
        let a = player_one.pop_front().unwrap();
        let b = player_two.pop_front().unwrap();
        let player_one_win = a > b;
        wins(&mut player_one, &mut player_two, player_one_win, a, b);
    }

    let result = score(&player_one) + score(&player_two);

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(mut player_one: Deck, mut player_two: Deck) -> Result<usize> {
    let start = Instant::now();

    let result = if game(&mut player_one, &mut player_two, &mut HashMap::new()) {
        score(&player_one)
    } else {
        score(&player_two)
    };

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn game(
    player_one: &mut Deck,
    player_two: &mut Deck,
    cache: &mut HashMap<(u64, u64), bool>,
) -> bool {
    let h1 = calculate_hash(&player_one);
    let h2 = calculate_hash(&player_two);
    if let Some(r) = cache.get(&(h1, h2)) {
        return *r;
    }
    let mut player_one_decks = HashSet::new();
    let mut player_two_decks = HashSet::new();
    while !player_one.is_empty() && !player_two.is_empty() {
        let h1 = calculate_hash(&player_one);
        let h2 = calculate_hash(&player_two);
        if !player_one_decks.insert(h1) || !player_two_decks.insert(h2) {
            return true;
        }
        let a = player_one.pop_front().unwrap();
        let b = player_two.pop_front().unwrap();
        if a > player_one.len() || b > player_two.len() {
            let player_one_win = a > b;
            wins(player_one, player_two, player_one_win, a, b);
        } else {
            let mut player_one_copy = player_one.clone();
            player_one_copy.truncate(a);
            let mut player_two_copy = player_two.clone();
            player_two_copy.truncate(b);
            let player_one_win = game(&mut player_one_copy, &mut player_two_copy, cache);
            wins(player_one, player_two, player_one_win, a, b);
        }
    }
    let r = !player_one.is_empty();
    cache.insert((h1, h2), r);
    r
}

fn wins(player_one: &mut Deck, player_two: &mut Deck, player_one_win: bool, a: usize, b: usize) {
    if player_one_win {
        player_one.push_back(a);
        player_one.push_back(b);
    } else {
        player_two.push_back(b);
        player_two.push_back(a);
    }
}

fn score(player: &Deck) -> usize {
    player
        .iter()
        .rev()
        .enumerate()
        .fold(0, |sum, (i, n)| sum + (i + 1) * n)
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
            cur.push_back(line.trim().parse()?);
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
    assert_eq!(part2(player_one, player_two).unwrap(), 291);
}
#[test]
fn loop_input() {
    let input = "Player 1:
    43
    19
    
    Player 2:
    2
    29
    14";

    let (player_one, player_two) = parse_input(input).unwrap();
    part2(player_one, player_two).unwrap();
}
