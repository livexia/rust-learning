use hashbrown::{HashMap, HashSet};
use std::collections::VecDeque;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Coord = (usize, usize);

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let grid = parse_input(&input);

    part1(&grid)?;
    // part2()?;
    Ok(())
}

fn part1(grid: &[Vec<char>]) -> Result<usize> {
    let start = Instant::now();

    let entrance = find_entrance(grid).unwrap();
    let keys = find_keys(grid);
    if keys.len() > 32 {
        unimplemented!("for {} keys", keys.len())
    }
    let complete_keys = keys.keys().fold(0, |h, &c| h | key_hash(c));
    let result = bfs_all_keys(grid, entrance, complete_keys).unwrap();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn bfs_all_keys(grid: &[Vec<char>], src: Coord, complete_keys: u32) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((src, 0));
    let mut depth = 0;
    while !queue.is_empty() {
        let size = queue.len();
        for _ in 0..size {
            let (cur, owned_keys) = queue.pop_front().unwrap();
            if owned_keys == complete_keys {
                return Some(depth);
            }
            for next in accessible_adjacent(cur.0, cur.1, grid, owned_keys) {
                let knid = grid[next.0][next.1];
                let temp = if is_key(knid) {
                    owned_keys | key_hash(knid)
                } else {
                    owned_keys
                };
                if visited.insert((next, temp)) {
                    queue.push_back((next, temp));
                }
            }
        }
        depth += 1;
    }
    None
}

fn key_hash(key: char) -> u32 {
    1 << (key as u8 - b'a')
}

fn find_keys(grid: &[Vec<char>]) -> HashMap<char, Coord> {
    let mut keys = HashMap::new();

    for x in 1..grid.len() - 1 {
        for y in 1..grid[0].len() - 1 {
            if is_key(grid[x][y]) {
                keys.insert(grid[x][y], (x, y));
            }
        }
    }

    keys
}

fn find_entrance(grid: &[Vec<char>]) -> Option<Coord> {
    for x in 1..grid.len() - 1 {
        for y in 1..grid[0].len() - 1 {
            if grid[x][y] == '@' {
                return Some((x, y));
            }
        }
    }
    None
}

fn accessible_adjacent(x: usize, y: usize, grid: &[Vec<char>], owned_keys: u32) -> Vec<Coord> {
    let mut r = Vec::with_capacity(4);
    for (x, y) in [
        (x.saturating_sub(1), y),
        (x + 1, y),
        (x, y.saturating_sub(1)),
        (x, y + 1),
    ] {
        if is_accessible(x, y, grid, owned_keys) {
            r.push((x, y));
        }
    }
    r
}

fn is_accessible(x: usize, y: usize, grid: &[Vec<char>], owned_keys: u32) -> bool {
    valid_coord(x, y, grid.len(), grid[0].len())
        && (grid[x][y] == '.'
            || (is_door(grid[x][y]) && owned_keys & key_hash(door_to_key(grid[x][y])) != 0)
            || is_key(grid[x][y]))
}

fn valid_coord(x: usize, y: usize, height: usize, width: usize) -> bool {
    x > 0 && y > 0 && x < height - 1 && y < width - 1
}

fn is_door(c: char) -> bool {
    ('A'..='Z').contains(&c)
}

fn door_to_key(c: char) -> char {
    (c as u8 - b'A' + b'a') as char
}

fn is_key(c: char) -> bool {
    ('a'..='z').contains(&c)
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.trim().chars().collect()).collect()
}

#[test]
fn example_input() {
    let input = "#########
    #b.A.@.a#
    #########";
    let grid = parse_input(input);
    assert_eq!(part1(&grid).unwrap(), 8);

    let input = "########################
    #f.D.E.e.C.b.A.@.a.B.c.#
    ######################.#
    #d.....................#
    ########################";
    let grid = parse_input(input);
    assert_eq!(part1(&grid).unwrap(), 86);

    let input = "########################
    #...............b.C.D.f#
    #.######################
    #.....@.a.B.c.d.A.e.F.g#
    ########################";
    let grid = parse_input(input);
    assert_eq!(part1(&grid).unwrap(), 132);

    let input = "#################
    #i.G..c...e..H.p#
    ########.########
    #j.A..b...f..D.o#
    ########@########
    #k.E..a...g..B.n#
    ########.########
    #l.F..d...h..C.m#
    #################";
    let grid = parse_input(input);
    assert_eq!(part1(&grid).unwrap(), 136);

    let input = "########################
    #@..............ac.GI.b#
    ###d#e#f################
    ###A#B#C################
    ###g#h#i################
    ########################";
    let grid = parse_input(input);
    assert_eq!(part1(&grid).unwrap(), 81);
}
