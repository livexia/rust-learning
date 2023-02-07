use std::collections::HashSet;
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
    let grid = parse_input(&input);

    part1(grid)?;
    // part2()?;
    Ok(())
}

fn part1(mut grid: u32) -> Result<u32> {
    let start = Instant::now();

    let mut visited = HashSet::new();
    while visited.insert(grid) {
        let mut new_grid = 0;
        for i in 0..5 {
            for j in 0..5 {
                let bug = is_bug((i, j), grid);
                let adjacent_bugs_count = adjacent((i, j))
                    .iter()
                    .filter(|&&c| fits(c, 5, 5) && is_bug(c, grid))
                    .count();
                if adjacent_bugs_count == 1 || (!bug && adjacent_bugs_count == 2) {
                    new_grid |= 1 << (i * 5 + j);
                }
            }
        }
        grid = new_grid;
    }
    let result = grid;

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn is_bug(c: Coord, grid: u32) -> bool {
    grid & (1 << (c.0 * 5 + c.1)) != 0
}

fn adjacent(c: Coord) -> [Coord; 4] {
    let (x, y) = c;
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

fn fits(c: Coord, height: i32, width: i32) -> bool {
    c.0 >= 0 && c.0 < height && c.1 >= 0 && c.1 < width
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
}
