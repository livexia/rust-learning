use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let report: Vec<i32> = input.lines().map(|l| l.parse().unwrap()).collect();

    part1(&report)?;
    part2(&report)?;
    Ok(())
}

fn part1(report: &[i32]) -> Result<()> {
    let sum = 2020;
    let mut set = HashSet::new();
    for entry in report {
        if set.contains(&(sum - entry)) {
            writeln!(
                io::stdout(),
                "Part1: Find the two entries that sum to 2020; what do you get if you multiply them together? {}", 
                entry * (sum - entry)
            )?;
            return Ok(());
        } else {
            set.insert(entry);
        }
    }
    err!("Part1: Can not find the two entries that sum to 2020;")
}

fn part2(report: &[i32]) -> Result<()> {
    let l = report.len();
    for i in 0..l {
        let sum = 2020 - report[i];
        let mut set = HashSet::new();
        for j in i + 1..l {
            let entry = report[j];
            if set.contains(&(sum - entry)) {
                writeln!(
                    io::stdout(),
                    "Part2: what is the product of the three entries that sum to 2020? {}",
                    entry * (sum - entry) * (2020 - sum)
                )?;
                return Ok(());
            } else {
                set.insert(entry);
            }
        }
    }
    err!("Part2: Can not find the three entries that sum to 2020;")
}
