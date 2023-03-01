use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn find_start(grid: &[Vec<char>]) -> Option<(usize, usize)> {
    for (i, &c) in grid[0].iter().enumerate() {
        if c == '|' {
            return Some((0, i));
        }
    }
    None
}

// 0 down, 1 left, 2 up, 3 right, clockwise
fn move_with_dir(
    x: usize,
    y: usize,
    dir: usize,
    grid: &[Vec<char>],
    path: &mut String,
) -> Option<(usize, usize, usize)> {
    let (nx, ny) = if dir == 0 && x + 1 < grid.len() {
        (x + 1, y)
    } else if dir == 1 && y > 0 {
        (x, y - 1)
    } else if dir == 2 && x > 0 {
        (x - 1, y)
    } else if dir == 3 && y + 1 < grid[0].len() {
        (x, y + 1)
    } else {
        // unreachable!("Wrong direction: {dir}");
        return None;
    };
    if grid[nx][ny].is_whitespace() {
        return None;
    } else if grid[nx][ny].is_alphabetic() {
        path.push(grid[nx][ny]);
    } else if grid[nx][ny] == '+' {
        let nd = [
            (nx + 1, ny),
            (nx, ny.saturating_sub(1)),
            (nx.saturating_sub(1), ny),
            (nx, ny + 1),
        ]
        .into_iter()
        .enumerate()
        .find(|&(_, (i, j))| {
            (i, j) != (x, y)
                && (i, j) != (nx, ny)
                && if let Some(c) = get(grid, i, j) {
                    c != ' '
                } else {
                    false
                }
        })
        .unwrap()
        .0;
        return Some((nx, ny, nd));
    }
    Some((nx, ny, dir))
}

fn get(grid: &[Vec<char>], x: usize, y: usize) -> Option<char> {
    if x < grid.len() && y < grid[0].len() {
        return Some(grid[x][y]);
    }
    None
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let grid = parse_input(&input);

    part1(&grid)?;
    // part2()?;
    Ok(())
}

fn part1(grid: &[Vec<char>]) -> Result<String> {
    let start = Instant::now();

    let ((mut x, mut y), mut d) = (find_start(grid).unwrap(), 0);
    let mut path = String::new();

    while let Some((nx, ny, nd)) = move_with_dir(x, y, d, grid, &mut path) {
        (x, y, d) = (nx, ny, nd);
    }

    writeln!(io::stdout(), "Part 1: {path}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(path)
}

#[test]
fn example_input() {
    let input = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
                ";
    assert_eq!(&part1(&parse_input(input)).unwrap(), "ABCDEF");
}
