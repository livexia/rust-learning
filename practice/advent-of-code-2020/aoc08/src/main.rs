use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Integer = i32;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let program: Program = input.parse()?;

    part1(program.clone())?;
    part2(program)?;
    Ok(())
}

fn part1(mut program: Program) -> Result<Integer> {
    let start = Instant::now();
    program.run_loop();
    let result = program.accumulator;

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(program: Program) -> Result<Integer> {
    let start = Instant::now();

    let mut result = 0;
    for mut p in program.iter() {
        if !p.run_loop() {
            result = p.accumulator;
            break;
        }
    }

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

#[derive(Clone)]
enum Instruction {
    Acc(Integer),
    Jmp(Integer),
    Nop(Integer),
}

impl Instruction {
    fn fix(&mut self) {
        match self {
            Instruction::Acc(_) => (),
            Instruction::Jmp(n) => *self = Instruction::Nop(*n),
            Instruction::Nop(n) => *self = Instruction::Jmp(*n),
        }
    }
}

#[derive(Clone)]
struct Program {
    instructions: Vec<Instruction>,
    pc: Integer,
    accumulator: Integer,
}

impl Program {
    fn run(&mut self) -> bool {
        let instr = &self.instructions[self.pc as usize];
        match instr {
            Instruction::Acc(n) => self.accumulator += n,
            Instruction::Jmp(offset) => self.pc += offset - 1,
            Instruction::Nop(_) => (),
        }
        self.pc += 1;
        (self.pc as usize) < self.instructions.len()
    }

    fn run_loop(&mut self) -> bool {
        let mut visited = HashSet::new();
        while visited.insert(self.pc) {
            if !self.run() {
                return false;
            }
        }
        true
    }

    fn iter(&self) -> ProgramIter {
        ProgramIter {
            program: self.clone(),
            index: 0,
        }
    }
}

impl FromStr for Program {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut instructions: Vec<Instruction> = vec![];
        for line in s.lines() {
            instructions.push(line.parse()?);
        }
        Ok(Self {
            instructions,
            pc: 0,
            accumulator: 0,
        })
    }
}

struct ProgramIter {
    program: Program,
    index: usize,
}

impl Iterator for ProgramIter {
    type Item = Program;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.program.instructions.len() {
            return None;
        }
        match self.program.instructions[self.index] {
            Instruction::Acc(_) => {
                self.index += 1;
                self.next()
            }
            Instruction::Jmp(_) | Instruction::Nop(_) => {
                let mut instructions = self.program.instructions.clone();
                instructions[self.index].fix();
                self.index += 1;
                Some(Program {
                    instructions,
                    pc: 0,
                    accumulator: 0,
                })
            }
        }
    }
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        match s.trim().split_once(' ') {
            Some(("nop", num)) => Ok(Self::Nop(num.trim().parse()?)),
            Some(("acc", num)) => Ok(Self::Acc(num.trim().parse()?)),
            Some(("jmp", num)) => Ok(Self::Jmp(num.trim().parse()?)),
            _ => err!("not a valid instruction: {}", s),
        }
    }
}

#[test]
fn example_input() {
    let input = "nop +0
    acc +1
    jmp +4
    acc +3
    jmp -3
    acc -99
    acc +1
    jmp -4
    acc +6";
    let program: Program = input.parse().unwrap();
    assert_eq!(part1(program.clone()).unwrap(), 5);
    assert_eq!(part2(program).unwrap(), 8);
}
