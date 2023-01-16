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

    dbg!(run_with_seq(program, &[4, 3, 2, 1, 0]));
    let mut output = Int::MIN;
    for a in 0..5 {
        let mut seq = vec![a];
        for b in 0..5 {
            if !seq.contains(&b) {
                seq.push(b);
                for c in 0..5 {
                    if !seq.contains(&c) {
                        seq.push(c);
                        for d in 0..5 {
                            if !seq.contains(&d) {
                                seq.push(d);
                                for e in 0..5 {
                                    if !seq.contains(&e) {
                                        seq.push(e);
                                        output = output.max(run_with_seq(program, &seq));
                                        seq.pop();
                                    }
                                }
                                seq.pop();
                            }
                        }
                        seq.pop();
                    }
                }
                seq.pop();
            }
        }
    }

    writeln!(io::stdout(), "Part 1: {output}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(output)
}

fn part2(program: &[Int]) -> Result<Int> {
    let start = Instant::now();

    let output = 0;

    writeln!(io::stdout(), "Part 2: {output}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(output)
}

fn run_with_seq(program: &[Int], seq: &[Int]) -> Int {
    let mut input = 0;
    for &i in seq {
        let mut n_p = program.to_owned();
        input = run_program(&mut n_p, &mut vec![input, i])
    }
    input
}

fn run_program(program: &mut [Int], input: &mut Vec<Int>) -> Int {
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
                program[op1] = input.pop().unwrap();
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
    assert_eq!(
        part1(&vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0
        ])
        .unwrap(),
        43210
    );
    assert_eq!(
        part1(&vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0
        ])
        .unwrap(),
        54321
    );
    assert_eq!(
        part1(&vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
        ])
        .unwrap(),
        65210
    );
}
