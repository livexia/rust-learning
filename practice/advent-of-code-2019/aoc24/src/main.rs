use std::collections::{HashMap, HashSet};
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
    let grid = parse_input(&input);

    part1(grid)?;
    part2(grid, 200)?;
    Ok(())
}

fn part1(mut grid: u32) -> Result<u32> {
    let start = Instant::now();

    let mut visited = HashSet::new();
    while visited.insert(grid) {
        let mut new_grid = 0;
        for i in 0..25 {
            let bug = is_bug(i, grid);
            let adjacent_bugs_count = adjacent(i).iter().filter(|&&c| is_bug(c, grid)).count();
            if adjacent_bugs_count == 1 || (!bug && adjacent_bugs_count == 2) {
                new_grid |= 1 << i;
            }
        }
        grid = new_grid;
    }
    let result = grid;

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(grid: u32, minutes: usize) -> Result<u32> {
    let start = Instant::now();

    let mut grids = HashMap::new();
    grids.insert(0, grid);
    for _ in 0..minutes {
        let mut new_grids = HashMap::new();
        for &level in grids.keys() {
            for level in level - 1..=level + 1 {
                if new_grids.contains_key(&level) {
                    continue;
                };
                let &cur = grids.get(&(level)).unwrap_or(&0);
                let &outer = grids.get(&(level - 1)).unwrap_or(&0);
                let &inner = grids.get(&(level + 1)).unwrap_or(&0);
                let mut new_grid = 0;
                for i in 0..25 {
                    let bug = is_bug(i, cur);
                    let adjacent_bugs_count = plutonian_adjacent(i, level)
                        .iter()
                        .filter(|&&(id, l)| {
                            if l == level {
                                is_bug(id, cur)
                            } else if l == level - 1 {
                                is_bug(id, outer)
                            } else if l == level + 1 {
                                is_bug(id, inner)
                            } else {
                                unreachable!()
                            }
                        })
                        .count();
                    if adjacent_bugs_count == 1 || (!bug && adjacent_bugs_count == 2) {
                        new_grid |= 1 << i;
                    }
                }
                if new_grid != 0 {
                    new_grids.insert(level, new_grid);
                }
            }
        }
        grids = new_grids;
    }

    let result = grids.values().map(|g| g.count_ones()).sum::<u32>();
    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn plutonian_adjacent(id: i32, level: i32) -> Vec<(i32, i32)> {
    if id == 12 {
        return vec![];
    }
    let mut r = vec![];

    if id < 5 {
        r.push((7, level - 1));
    } else if id >= 20 {
        r.push((17, level - 1));
    }
    if id % 5 == 0 {
        r.push((11, level - 1));
    } else if id % 5 == 4 {
        r.push((13, level - 1));
    }
    if id == 7 {
        for i in (0..).step_by(1).take(5) {
            r.push((i, level + 1));
        }
    } else if id == 11 {
        for i in (0..).step_by(5).take(5) {
            r.push((i, level + 1));
        }
    } else if id == 13 {
        for i in (4..).step_by(5).take(5) {
            r.push((i, level + 1));
        }
    } else if id == 17 {
        for i in (20..).step_by(1).take(5) {
            r.push((i, level + 1));
        }
    }
    if id - 5 >= 0 && id - 5 != 12 {
        r.push((id - 5, level));
    }
    if id + 5 < 25 && id + 5 != 12 {
        r.push((id + 5, level));
    }
    if id > 0 && id - 1 != 12 && (id - 1) / 5 == id / 5 {
        r.push((id - 1, level));
    }
    if id + 1 < 25 && id + 1 != 12 && (id + 1) / 5 == id / 5 {
        r.push((id + 1, level));
    }
    r
}

fn is_bug(id: i32, grid: u32) -> bool {
    grid & (1 << id) != 0
}

fn adjacent(id: i32) -> Vec<i32> {
    let mut r = vec![];
    if id - 5 >= 0 {
        r.push(id - 5);
    }
    if id + 5 < 25 {
        r.push(id + 5);
    }
    if id > 0 && (id - 1) / 5 == id / 5 {
        r.push(id - 1);
    }
    if id + 1 < 25 && (id + 1) / 5 == id / 5 {
        r.push(id + 1);
    }
    r
}

fn parse_input(input: &str) -> u32 {
    let mut grid = 0u32;
    for (i, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        for (j, c) in line.trim().char_indices() {
            if c == '#' {
                grid |= 1 << (i * 5 + j)
            }
        }
    }
    grid
}

#[test]
fn example_input() {
    let input = "....#
    #..#.
    #..##
    ..#..
    #....";
    let grid = parse_input(input);
    assert_eq!(part1(grid).unwrap(), 2129920);
    assert_eq!(part2(grid, 10).unwrap(), 99);
}
