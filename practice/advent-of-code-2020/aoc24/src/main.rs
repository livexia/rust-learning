// use std::collections::HashSet;
use hashbrown::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Coord = (i32, i32);

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let dirs = parse_input(&input);

    part1(&dirs)?;
    part2(&dirs)?;
    Ok(())
}

fn part1(dirs: &[Vec<Dir>]) -> Result<usize> {
    let start = Instant::now();
    let result = flip_by_input(dirs).len();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn flip_by_input(dirs: &[Vec<Dir>]) -> HashSet<Coord> {
    let reference = (0, 0);
    let mut black_tiles = HashSet::new();

    for dir in dirs {
        let mut cur = reference;
        for d in dir {
            cur = d.next(cur.0, cur.1);
        }
        if black_tiles.contains(&cur) {
            black_tiles.remove(&cur);
        } else {
            black_tiles.insert(cur);
        };
    }
    black_tiles
}

fn part2(dirs: &[Vec<Dir>]) -> Result<usize> {
    let start = Instant::now();

    let mut black_tiles = flip_by_input(dirs);
    for _ in 0..100 {
        black_tiles = flip_by_day(&black_tiles);
    }

    let result = black_tiles.len();

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn flip_by_day(black_tiles: &HashSet<Coord>) -> HashSet<Coord> {
    let (min, max) = range(black_tiles);
    let mut new_black_tiles = HashSet::new();
    for x in min.0..=max.0 {
        for y in min.1..=max.1 {
            if is_flipped(black_tiles, (x, y)) {
                new_black_tiles.insert((x, y));
            }
        }
    }
    new_black_tiles
}

fn range(black_tiles: &HashSet<Coord>) -> (Coord, Coord) {
    let min_x = black_tiles.iter().min_by_key(|a| a.0).unwrap().0;
    let min_y = black_tiles.iter().min_by_key(|a| a.1).unwrap().1;
    let max_x = black_tiles.iter().max_by_key(|a| a.0).unwrap().0;
    let max_y = black_tiles.iter().max_by_key(|a| a.1).unwrap().1;
    ((min_x - 1, min_y - 1), (max_x + 1, max_y + 1))
}

fn adjacent(coord: Coord) -> [Coord; 6] {
    let (x, y) = coord;
    [
        (x + 1, y),
        (x + 1, y - 1),
        (x, y + 1),
        (x - 1, y),
        (x, y - 1),
        (x - 1, y + 1),
    ]
}

fn is_flipped(black_tiles: &HashSet<Coord>, coord: Coord) -> bool {
    let is_black = black_tiles.contains(&coord);
    let mut count = 0;
    for c in adjacent(coord).iter() {
        if black_tiles.contains(c) {
            count += 1;
            if count > 2 {
                break;
            }
        }
    }
    (is_black && count != 0 && count <= 2) || count == 2
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    E,
    SE,
    NE,
    W,
    SW,
    NW,
}

impl Dir {
    fn next(&self, x: i32, y: i32) -> Coord {
        match self {
            Dir::E => (x + 1, y),
            Dir::SE => (x + 1, y - 1),
            Dir::NE => (x, y + 1),
            Dir::W => (x - 1, y),
            Dir::SW => (x, y - 1),
            Dir::NW => (x - 1, y + 1),
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Dir>> {
    use Dir::*;

    let mut dirs = vec![];
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let mut dir = vec![];
        let mut chars = line.trim().chars();
        while let Some(c) = chars.next() {
            if c == 'e' {
                dir.push(E)
            } else if c == 'w' {
                dir.push(W)
            } else if c == 's' {
                if let Some(next) = chars.next() {
                    if next == 'e' {
                        dir.push(SE)
                    } else if next == 'w' {
                        dir.push(SW)
                    } else {
                        unreachable!()
                    }
                } else {
                    unreachable!()
                }
            } else if c == 'n' {
                if let Some(next) = chars.next() {
                    if next == 'e' {
                        dir.push(NE)
                    } else if next == 'w' {
                        dir.push(NW)
                    } else {
                        unreachable!()
                    }
                } else {
                    unreachable!()
                }
            } else {
                unreachable!("char: {}", c);
            }
        }

        dirs.push(dir)
    }
    dirs
}

#[test]
fn example_input() {
    let input = "sesenwnenenewseeswwswswwnenewsewsw
    neeenesenwnwwswnenewnwwsewnenwseswesw
    seswneswswsenwwnwse
    nwnwneseeswswnenewneswwnewseswneseene
    swweswneswnenwsewnwneneseenw
    eesenwseswswnenwswnwnwsewwnwsene
    sewnenenenesenwsewnenwwwse
    wenwwweseeeweswwwnwwe
    wsweesenenewnwwnwsenewsenwwsesesenwne
    neeswseenwwswnwswswnw
    nenwswwsewswnenenewsenwsenwnesesenew
    enewnwewneswsewnwswenweswnenwsenwsw
    sweneswneswneneenwnewenewwneswswnese
    swwesenesewenwneswnwwneseswwne
    enesenwswwswneneswsenwnewswseenwsese
    wnwnesenesenenwwnenwsewesewsesesew
    nenewswnwewswnenesenwnesewesw
    eneswnwswnwsenenwnwnwwseeswneewsenese
    neswnwewnwnwseenwseesewsenwsweewe
    wseweeenwnesenwwwswnew";

    let dirs = parse_input(input);
    assert_eq!(10, part1(&dirs).unwrap());
    assert_eq!(2208, part2(&dirs).unwrap());
}
