use itertools::Itertools;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Int = i128;
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

    let mut output = Int::MIN;
    for seq in (0..5).permutations(5).unique() {
        output = output.max(run_with_seq(program, &seq, true));
    }

    writeln!(io::stdout(), "Part 1: {output}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(output)
}

fn part2(program: &[Int]) -> Result<Int> {
    let start = Instant::now();

    let mut output = 0;
    for seq in (5..10).permutations(5).unique() {
        output = output.max(run_with_seq(program, &seq, false));
    }

    writeln!(io::stdout(), "Part 2: {output}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(output)
}

fn run_with_seq(program: &[Int], seq: &[Int], restart: bool) -> Int {
    let mut last_output = 0;
    let mut amps = vec![];
    let mut status = 0;
    for &i in seq {
        amps.push(Amp::new(program, i))
    }
    loop {
        for p in amps.iter_mut() {
            p.add_input(last_output);
            status = p.run_program();
            last_output = p.output;
        }
        if status == 99 {
            break;
        }
        if restart {
            break;
        }
    }
    last_output
}

struct Amp {
    program: Vec<Int>,
    pc: Addr,
    input: Vec<Int>,
    output: Int,
}

impl Amp {
    fn new(program: &[Int], setting: Int) -> Self {
        Amp {
            program: program.to_owned(),
            pc: 0,
            input: vec![setting],
            output: 0,
        }
    }

    fn add_input(&mut self, i: Int) {
        self.input.push(i);
        self.input.reverse();
    }

    fn run_program(&mut self) -> Int {
        while self.pc < self.program.len() {
            let (opcode, f1, f2, f3) = parse_opcode(self.program[self.pc]);
            if opcode == 99 {
                return 99;
            }
            let op1 = addr_lookup(&self.program, self.pc + 1, f1);
            match opcode {
                1 | 2 | 7 | 8 => {
                    instr_with_four(&mut self.program, self.pc, opcode, f1, f2, f3);
                    self.pc += 4;
                }
                3 => {
                    if let Some(i) = self.input.pop() {
                        self.program[op1] = i;
                        self.pc += 2
                    } else {
                        return 3;
                    }
                }
                4 => {
                    self.output = self.program[op1];
                    self.pc += 2;
                    return 4;
                }
                5 => {
                    let op2 = addr_lookup(&self.program, self.pc + 2, f2);
                    if self.program[op1] != 0 {
                        self.pc = self.program[op2] as usize;
                    } else {
                        self.pc += 3
                    }
                }
                6 => {
                    let op2 = addr_lookup(&self.program, self.pc + 2, f2);
                    if self.program[op1] == 0 {
                        self.pc = self.program[op2] as usize;
                    } else {
                        self.pc += 3
                    }
                }
                // 99 => return 99,
                _ => unreachable!(
                    "Encountering an unknown opcode means something went wrong: {}",
                    opcode
                ),
            };
        }
        0
    }
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

fn parse_opcode(opcode: Int) -> (Int, bool, bool, bool) {
    (
        opcode % 100,
        (opcode / 100) % 10 == 1,
        (opcode / 1000) % 10 == 1,
        (opcode / 10000) % 10 == 1,
    )
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

    assert_eq!(
        run_with_seq(
            &vec![
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5
            ],
            &[9, 8, 7, 6, 5],
            false
        ),
        139629729
    );

    assert_eq!(
        part2(&vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5
        ])
        .unwrap(),
        139629729
    );
    assert_eq!(
        part2(&vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
        ])
        .unwrap(),
        18216
    );
}
