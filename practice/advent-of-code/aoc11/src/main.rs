use std::io::{self, Write};
use std::error::Error;
use std::result;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let serial_number: usize = input.trim().parse()?;

    let mut grid = vec![vec![0; 301]; 301];
    for x in 1..301 {
        for y in 1..301 {
            grid[x][y] = power_level(x, y, serial_number) + grid[x-1][y] + grid[x][y-1] - grid[x-1][y-1];
        }
    }
    
    part1(&grid)?;
    part2(&grid)?;
    

    Ok(())
}

fn part1(grid: &Vec<Vec<i32>>) -> Result<()> {
    let mut answer = (1, 1);
    let mut max = 0;

    for x in 1..299 {
        for y in 1..299 {
            let temp = grid[x+2][y+2] + grid[x-1][y-1] - grid[x-1][y+2] - grid[x+2][y-1];
            if temp > max {
                max = temp;
                answer = (x, y);
            }
        }
    }

    writeln!(
        io::stdout(), 
        "part1 answer: {},{}", 
        answer.0, answer.1
    )?;
    
    Ok(())
}

fn part2(grid: &Vec<Vec<i32>>) -> Result<()> {
    let mut answer = (1, 1, 1);
    let mut max = 0;
    for x in 1..301 {
        for y in 1..301 {
            for c in 0..301 {
                if x + c > 300 || y + c > 300 {
                    continue;
                }
                let temp = grid[x+c][y+c] + grid[x-1][y-1] - grid[x-1][y+c] - grid[x+c][y-1];
                if temp > max {
                    max = temp;
                    answer = (x, y, c + 1);
                }
            }
        }
    }

    writeln!(
        io::stdout(), 
        "part2 answer: {},{},{}", 
        answer.0, answer.1, answer.2
    )?;
    
    Ok(())
}

fn power_level(x: usize, y: usize, serial_number: usize) -> i32 {
    (((x + 10) * y + serial_number) * (x + 10) / 100 % 10) as i32 - 5
}