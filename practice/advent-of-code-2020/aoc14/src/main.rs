use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let programs = parse_input(&input)?;

    part1(&programs)?;
    // part2()?;
    Ok(())
}

fn part1(programs: &[Program]) -> Result<u64> {
    let start = Instant::now();

    let mut mem = HashMap::new();
    for p in programs {
        p.run(&mut mem);
    }
    let result: u64 = mem.values().sum();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

struct Program {
    bitmask: (u64, u64),
    mem_writes: Vec<(u64, u64)>,
}

impl Program {
    fn new(bitmask: (u64, u64)) -> Self {
        Self {
            bitmask,
            mem_writes: vec![],
        }
    }

    fn run(&self, mem: &mut HashMap<u64, u64>) {
        let (one, zero) = self.bitmask;
        for &(index, value) in &self.mem_writes {
            let value = value | one;
            let value = value & zero;
            mem.insert(index, value);
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Program>> {
    let mut programs = vec![];
    let lines: Vec<_> = input.lines().collect();
    let mut index = 0;
    let mut p: Program;
    while index < lines.len() {
        let line = lines[index];

        if line.trim().starts_with("mask = ") {
            let mask = parse_bitmask(line)?;
            p = Program::new(mask);
            index += 1;
            while index < lines.len() && lines[index].trim().starts_with("mem") {
                p.mem_writes.push(parse_mem(lines[index])?);
                index += 1;
            }
            programs.push(p);
        }
    }

    Ok(programs)
}

fn parse_bitmask(mask_line: &str) -> Result<(u64, u64)> {
    if let Some(mask) = mask_line.trim().strip_prefix("mask = ") {
        let one_mask = u64::from_str_radix(&mask.replace('X', "0"), 2)?; // contain one bit with empty zero bit
        let zero_mask = u64::from_str_radix(&mask.replace('X', "1"), 2)?; // zero bit with empty one bit

        return Ok((one_mask, zero_mask));
    }

    err!("not a valid mask line: {mask_line}")
}

fn parse_mem(mem_line: &str) -> Result<(u64, u64)> {
    if let Some(mem) = mem_line.trim().strip_prefix("mem[") {
        if let Some((index, value)) = mem.split_once("] = ") {
            let index = index.trim().parse()?;
            let value = value.trim().parse()?;
            return Ok((index, value));
        }
    }
    err!("not a valid mem line: {mem_line}")
}

#[test]
fn example_input() {
    let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    mem[8] = 11
    mem[7] = 101
    mem[8] = 0";
    let programs = parse_input(input).unwrap();
    assert_eq!(part1(&programs).unwrap(), 165);
}
