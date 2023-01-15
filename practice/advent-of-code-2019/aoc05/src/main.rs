use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Int = isize;
type Addr = usize;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let program = parse_input(&input)?;

    part1(&program)?;
    part2(&program)?;
    Ok(())
}

fn part1(program: &[Int]) -> Result<Int> {
    let start = Instant::now();

    let mut output = 0;
    let mut program = program.to_owned();
    run_program(&mut program, 1, &mut output);

    writeln!(io::stdout(), "Part 1: {output}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(output)
}

fn part2(program: &[Int]) -> Result<Int> {
    let start = Instant::now();

    let result = 0;

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn run_program(program: &mut [Int], input: Int, output: &mut Int) {
    let mut pc = 0;
    while pc < program.len() {
        let (opcode, f1, f2, f3) = parse_opcode(program[pc]);
        let op1 = addr_lookup(program, pc + 1, f1);
        let op2 = addr_lookup(program, pc + 2, f2);
        let dest = addr_lookup(program, pc + 3, f3);
        match opcode {
            1 => {
                program[dest] = program[op1] + program[op2];
                pc += 4;
            }
            2 => {
                program[dest] = program[op1] * program[op2];
                pc += 4;
            }
            3 => {
                program[op1] = input;
                pc += 2
            }
            4 => {
                *output = program[op1];
                pc += 2
            }
            99 => return,
            _ => unreachable!(
                "Encountering an unknown opcode means something went wrong: {}",
                opcode
            ),
        };
    }
}

fn parse_opcode(mut opcode: Int) -> (Int, bool, bool, bool) {
    let mut r = (0, false, false, false);
    r.0 = opcode % 100;
    opcode /= 100;
    r.1 = opcode % 10 == 1;
    opcode /= 10;
    r.2 = opcode % 10 == 1;
    opcode /= 10;
    r.3 = opcode % 10 == 1;
    r
}

fn addr_lookup(program: &[Int], pc: Addr, flag: bool) -> Addr {
    if flag {
        pc
    } else {
        if program[pc] < 0 {
            unreachable!()
        }
        program[pc] as usize
    }
}

fn parse_input(input: &str) -> Result<Vec<Int>> {
    let mut program = vec![];
    for num in input.trim().split(',') {
        program.push(num.parse()?);
    }
    Ok(program)
}

#[test]
fn example_input() {
    assert_eq!(parse_opcode(1002), (2, false, true, false));
}
