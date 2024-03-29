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

    let output = run_program(&mut program.to_owned(), 1);

    writeln!(io::stdout(), "Part 1: {output}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(output)
}

fn part2(program: &[Int]) -> Result<Int> {
    let start = Instant::now();

    let output = run_program(&mut program.to_owned(), 5);

    writeln!(io::stdout(), "Part 2: {output}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(output)
}

fn run_program(program: &mut [Int], input: Int) -> Int {
    let mut output = 0;
    let mut pc = 0;
    while pc < program.len() {
        let (opcode, f1, f2, f3) = parse_opcode(program[pc]);
        let op1 = addr_lookup(program, pc + 1, f1);
        match opcode {
            1 | 2 | 7 | 8 => {
                instr_with_four(program, pc, opcode, f1, f2, f3);
                pc += 4;
            }
            3 => {
                program[op1] = input;
                pc += 2
            }
            4 => {
                output = program[op1];
                pc += 2
            }
            5 => {
                let op2 = addr_lookup(program, pc + 2, f2);
                if program[op1] != 0 {
                    pc = program[op2] as usize;
                } else {
                    pc += 3
                }
            }
            6 => {
                let op2 = addr_lookup(program, pc + 2, f2);
                if program[op1] == 0 {
                    pc = program[op2] as usize;
                } else {
                    pc += 3
                }
            }
            99 => return output,
            _ => unreachable!(
                "Encountering an unknown opcode means something went wrong: {}",
                opcode
            ),
        };
    }
    output
}

fn instr_with_four(program: &mut [Int], pc: Addr, opcode: Int, f1: bool, f2: bool, f3: bool) {
    let op1 = addr_lookup(program, pc + 1, f1);
    let op2 = addr_lookup(program, pc + 2, f2);
    let dest = addr_lookup(program, pc + 3, f3);
    program[dest] = match opcode {
        1 => program[op1] + program[op2],
        2 => program[op1] * program[op2],
        7 => (program[op1] < program[op2]) as Int,
        8 => (program[op1] == program[op2]) as Int,
        _ => unreachable!(),
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
        assert!(program[pc] >= 0);
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
