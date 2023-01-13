use std::collections::HashMap;
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
    let dirs = parse_input(&input);

    part1(&dirs)?;
    // part2()?;
    Ok(())
}

fn part1(dirs: &[Vec<Dir>]) -> Result<usize> {
    let start = Instant::now();
    let reference = (0, 0);
    let mut tiles = HashMap::new();

    for dir in dirs {
        let mut cur = reference;
        for d in dir {
            cur = d.next(cur.0, cur.1);
        }
        let b = if let Some(b) = tiles.get(&cur) {
            !b
        } else {
            true
        };
        tiles.insert(cur, b);
    }

    let result = tiles.values().filter(|&&b| b).count();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
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
    fn next(&self, x: i32, y: i32) -> (i32, i32) {
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
}
