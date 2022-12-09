use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let grid_pattern: Vec<Vec<bool>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| if c == '.' { false } else { true })
                .collect()
        })
        .collect();

    part1(&grid_pattern)?;
    part2(&grid_pattern)?;
    Ok(())
}

fn part1(grid: &[Vec<bool>]) -> Result<()> {
    let count = slopes(grid, 3, 1);
    writeln!(
        io::stdout(),
        "Part1: how many trees would you encounter? {}",
        count
    )?;
    Ok(())
}

fn part2(grid: &[Vec<bool>]) -> Result<()> {
    let result = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .fold(1, |r, &(right, down)| {
            dbg!(slopes(grid, right, down));
            r * slopes(grid, right, down)
        });
    writeln!(
        io::stdout(),
        "Part2: What do you get if you multiply together the number of trees encountered on each of the listed slopes? {}",
        result
    )?;
    Ok(())
}

fn slopes(grid: &[Vec<bool>], right: usize, down: usize) -> usize {
    // let (mut x, mut y) = (0, 0);
    // let mut count = 0;
    // while x < grid.len() {
    //     if tree_at_location(grid, x, y) {
    //         count += 1;
    //     }
    //     (x, y) = (x + down, y + right);
    // }
    // count
    (0..grid.len())
        .step_by(down)
        .filter(|&x| tree_at_location(grid, x, x * right / down))
        .count()
}

fn tree_at_location(grid: &[Vec<bool>], x: usize, y: usize) -> bool {
    if x >= grid.len() {
        return false;
    }
    grid[x][y % grid[0].len()]
}
