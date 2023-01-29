use hashbrown::HashSet;
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
    let mut grid = parse_input(&input);

    part1(&grid)?;

    let entrance = find_entrance(&grid).unwrap();
    update_map(&mut grid, entrance);
    part2(&grid)?;
    Ok(())
}

fn part1(grid: &[Vec<char>]) -> Result<usize> {
    let start = Instant::now();

    let entrance = find_entrance(grid).unwrap();
    let complete_keys = find_keys(grid);
    let result = bfs_all_keys(grid, entrance, complete_keys).unwrap();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(grid: &[Vec<char>]) -> Result<usize> {
    let start = Instant::now();

    let entrances = find_four_entrance(grid);
    let four_complete_keys = bfs_four_complete_keys(grid, &entrances);
    println!("{:b}", four_complete_keys[1]);
    let mut result = 0;
    for (entrance, complete_keys) in entrances.into_iter().zip(four_complete_keys.into_iter()) {
        result += bfs_all_keys(grid, entrance, complete_keys).unwrap()
    }

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn bfs_four_complete_keys(grid: &[Vec<char>], entrances: &[Coord]) -> [u32; 4] {
    let height = grid.len();
    let width = grid[0].len();
    let mut keys = [0; 4];

    for (i, &src) in entrances.iter().enumerate() {
        let mut queue = VecDeque::new();
        queue.push_back((coord_to_hash(src), 0));
        let mut visited = HashSet::new();
        visited.insert((coord_to_hash(src), 0));
        while let Some((cur, owned_keys)) = queue.pop_front() {
            let (x, y) = hash_to_coord(cur);
            for next in [
                (x.saturating_sub(1), y),
                (x + 1, y),
                (x, y.saturating_sub(1)),
                (x, y + 1),
            ] {
                if valid_coord(next.0, next.1, height, width) && grid[next.0][next.1] != '#' {
                    let kind = grid[next.0][next.1];
                    let temp = if is_key(kind) {
                        keys[i] |= key_hash(kind);
                        owned_keys | key_hash(kind)
                    } else {
                        owned_keys
                    };
                    if visited.insert((coord_to_hash(next), temp)) {
                        queue.push_back((coord_to_hash(next), temp));
                    }
                }
            }
        }
    }

    keys
}

fn update_map(grid: &mut [Vec<char>], entrance: Coord) {
    let (x, y) = entrance;
    grid[x][y] = '#';
    grid[x - 1][y] = '#';
    grid[x][y - 1] = '#';
    grid[x + 1][y] = '#';
    grid[x][y + 1] = '#';
    grid[x - 1][y - 1] = '@';
    grid[x - 1][y + 1] = '@';
    grid[x + 1][y - 1] = '@';
    grid[x + 1][y + 1] = '@';
}

fn bfs_all_keys(grid: &[Vec<char>], src: Coord, complete_keys: u32) -> Option<usize> {
    let height = grid.len();
    let width = grid[0].len();

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((coord_to_hash(src), 0));
    visited.insert((coord_to_hash(src), 0));
    let mut depth = 0;
    while !queue.is_empty() {
        let size = queue.len();
        for _ in 0..size {
            let (cur, owned_keys) = queue.pop_front().unwrap();
            if owned_keys == complete_keys {
                return Some(depth);
            }
            let (x, y) = hash_to_coord(cur);
            for next in [
                (x.saturating_sub(1), y),
                (x + 1, y),
                (x, y.saturating_sub(1)),
                (x, y + 1),
            ] {
                if valid_coord(next.0, next.1, height, width)
                    && is_accessible(grid[next.0][next.1], owned_keys, complete_keys)
                {
                    let kind = grid[next.0][next.1];
                    let temp = if is_key(kind) {
                        owned_keys | key_hash(kind)
                    } else {
                        owned_keys
                    };
                    if visited.insert((coord_to_hash(next), temp)) {
                        queue.push_back((coord_to_hash(next), temp));
                    }
                }
            }
        }
        depth += 1;
    }
    None
}

fn coord_to_hash(c: Coord) -> u64 {
    let (x, y) = (c.0 as u64, c.1 as u64);
    x << 8 | y
}

fn hash_to_coord(h: u64) -> Coord {
    ((h >> 8) as usize, (h & 0xff) as usize)
}

fn key_hash(key: char) -> u32 {
    1 << (key as u8 - b'a')
}

fn find_keys(grid: &[Vec<char>]) -> u32 {
    let mut keys = 0;

    for x in 1..grid.len() - 1 {
        for y in 1..grid[0].len() - 1 {
            if is_key(grid[x][y]) {
                keys |= key_hash(grid[x][y]);
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

fn find_four_entrance(grid: &[Vec<char>]) -> Vec<Coord> {
    let mut r = vec![];
    for x in 1..grid.len() - 1 {
        for y in 1..grid[0].len() - 1 {
            if grid[x][y] == '@' {
                r.push((x, y));
            }
        }
    }
    r
}

fn is_accessible(kind: char, owned_keys: u32, complete_keys: u32) -> bool {
    kind == '.'
        || kind == '@'
        || is_key(kind)
        || (is_door(kind) && owned_keys & key_hash(door_to_key(kind)) != 0)
        || (is_door(kind) && complete_keys & key_hash(door_to_key(kind)) == 0)
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

    // part2
    let input = "#######
    #a.#Cd#
    ##@#@##
    #######
    ##@#@##
    #cB#Ab#
    #######";
    let grid = parse_input(input);
    assert_eq!(part2(&grid).unwrap(), 8);

    let input = "###############
    #d.ABC.#.....a#
    ######@#@######
    ###############
    ######@#@######
    #b.....#.....c#
    ###############";
    let grid = parse_input(input);
    assert_eq!(part2(&grid).unwrap(), 24);

    let input = "#############
    #DcBa.#.GhKl#
    #.###@#@#I###
    #e#d#####j#k#
    ###C#@#@###J#
    #fEbA.#.FgHi#
    #############";
    let grid = parse_input(input);
    assert_eq!(part2(&grid).unwrap(), 32);

    let input = "#############
    #g#f.D#..h#l#
    #F###e#E###.#
    #dCba@#@BcIJ#
    #############
    #nK.L@#@G...#
    #M###N#H###.#
    #o#m..#i#jk.#
    #############";
    let grid = parse_input(input);
    assert_eq!(part2(&grid).unwrap(), 72);
}
