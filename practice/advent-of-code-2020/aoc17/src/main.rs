use hashbrown::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Coord = (i32, i32, i32);
type Grid = HashSet<Coord>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let init_grid = parse_input(&input);

    part1(&init_grid)?;
    // part2()?;
    Ok(())
}

fn part1(grid: &Grid) -> Result<usize> {
    let start = Instant::now();

    let mut grid = grid.clone();
    for _ in 0..6 {
        grid = cycle(grid);
    }

    let result = grid.len();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn cycle(grid: Grid) -> Grid {
    let mut new_grid = Grid::new();
    let (start, end) = range(&grid);
    for x in start.0..=end.0 {
        for y in start.1..=end.1 {
            for z in start.2..=end.2 {
                let count = active_counter(&(x, y, z), &grid);
                if count == 3 || (grid.contains(&(x, y, z)) && count == 2) {
                    new_grid.insert((x, y, z));
                }
            }
        }
    }
    new_grid
}

fn neighbors(src: &Coord) -> Vec<Coord> {
    let mut result = vec![];
    for dx in [1, 0, -1] {
        for dy in [1, 0, -1] {
            for dz in [1, 0, -1] {
                if dx == dy && dy == dz && dz == 0 {
                    continue;
                }
                result.push((src.0 + dx, src.1 + dy, src.2 + dz))
            }
        }
    }
    result
}

fn active_counter(src: &Coord, grid: &Grid) -> usize {
    neighbors(src).iter().filter(|&c| grid.contains(c)).count()
}

fn range(grid: &Grid) -> (Coord, Coord) {
    let (mut min_x, mut min_y, mut min_z) = (i32::MAX, i32::MAX, i32::MAX);
    let (mut max_x, mut max_y, mut max_z) = (i32::MIN, i32::MIN, i32::MIN);
    for &(x, y, z) in grid {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        min_z = min_z.min(z);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
        max_z = max_z.max(z);
    }
    (
        (min_x - 1, min_y - 1, min_z - 1),
        (max_x + 1, max_y + 1, max_z + 1),
    )
}

fn parse_input(input: &str) -> Grid {
    let mut grid: Grid = HashSet::new();
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.trim().char_indices() {
            if c == '#' {
                grid.insert((0, x as i32, y as i32));
            }
        }
    }
    grid
}

#[test]
fn example_input() {
    let input = ".#.
    ..#
    ###";
    let grid = parse_input(input);
    assert_eq!(part1(&grid).unwrap(), 112);
}
