use std::io::{self, Read, Write};
use std::error::Error;
use std::fmt;
use std::result;
use std::str::FromStr;
use std::collections::HashSet;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn Error>>;

type Value = usize;
type Register = usize;

fn main() -> Result<()>{

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    let mut program: Program = input.parse()?;

    part1(&mut program, false)?;
    part2(&mut program, false)?;

    Ok(())
}

fn part1(program: &mut Program, debug: bool) -> Result<()> {
    program.init(vec![16457176, 0, 0, 0, 0, 0]);
    program.run(debug)?;
    writeln!(io::stdout(), "part1 answer: {}", program.registers[0])?;
    Ok(())
}

fn part2(program: &mut Program, debug: bool) -> Result<()> {
    program.init(vec![0, 0, 0, 0, 0, 0]);
    let answer = program.run(debug)?;
    writeln!(io::stdout(), "part2 answer: {}", answer)?;
    Ok(())
}

#[derive(Debug)]
struct Program {
    ip: (usize, usize),
    registers: Vec<Value>,
    instructions: Vec<Instruction>,
}

impl Program {
    fn init(&mut self, registers: Vec<Value>) {
        self.ip.1 = 0;
        self.registers = registers;
    }

    fn run(&mut self, debug: bool) -> Result<usize> {
        let n = self.instructions.len();
        let mut count: u64 = 0;
        let mut seen = HashSet::new();
        let mut cycle = vec![];

        while self.ip.1 < n {
            count += 1;
            self.registers[self.ip.0] = self.ip.1;

            if self.ip.1 == 28 {
                if seen.contains(&self.registers[5]) {
                    break;
                }
                seen.insert(self.registers[5]);
                cycle.push(self.registers[5]);
            }

            if debug {
                write!(
                    io::stdout(), 
                    "ip={:?} {:?} {} ", 
                    self.ip, self.registers, self.instructions[self.ip.1])?;
            }
            self.execute(self.ip.1)?;
            self.ip.1 = self.registers[self.ip.0] + 1;
            if debug {
                writeln!(io::stdout(), " {:?}", self.registers)?;
            }
        }
        writeln!(io::stdout(), "executing {} instructions", count)?;

        Ok(*cycle.last().unwrap())
    }

    fn execute(&mut self, ip: usize) -> Result<()> {
        let instr = &self.instructions[ip];
        let (op, a, b, c) = (instr.op.clone(), instr.op1, instr.op2, instr.output);
        self.registers[c] = match op.as_str() {
            "addr" => self.registers[a] + self.registers[b],
            "addi" => b + self.registers[a],
            "mulr" => self.registers[a] * self.registers[b],
            "muli" => b * self.registers[a],
            "banr" => self.registers[a] & self.registers[b],
            "bani" => b & self.registers[a],
            "borr" => self.registers[a] | self.registers[b],
            "bori" => self.registers[a] | b,
            "setr" => self.registers[a],
            "seti" => a,
            "gtri" => if self.registers[a] > b { 1 } else { 0 },
            "gtir" => if a > self.registers[b] { 1 } else { 0 },
            "gtrr" => if self.registers[a] > self.registers[b] { 1 } else { 0 },
            "eqri" => if self.registers[a] == b { 1 } else { 0 },
            "eqir" => if a == self.registers[b] { 1 } else { 0 },
            "eqrr" => if self.registers[a] == self.registers[b] { 1 } else { 0 },
            _ => return err!("wrong instruction: {}", instr),
        };
        Ok(())
    }
}

impl FromStr for Program {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self> {
        let lines: Vec<&str> = s.trim().lines().collect();
        let mut ip = (0, 0);
        if lines[0].starts_with("#ip ") {
            ip.0 = lines[0].strip_prefix("#ip ").unwrap().parse()?;
        }
        let mut instructions = vec![];
        for &line in &lines[1..] {
            instructions.push(line.parse()?);
        }
        Ok(Self {ip, instructions, registers: vec![0, 0, 0, 0, 0, 0]})
    }
}

#[derive(Debug)]
struct Instruction {
    op: String,
    op1: usize,
    op2: usize,
    output: Register
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let instr: Vec<&str> = s.trim().split(" ").collect();
        if instr.len() != 4 {
            return err!("wrong instruction: {}", s);
        }
        Ok(Self{
            op: instr[0].parse()?,
            op1: instr[1].parse()?,
            op2: instr[2].parse()?,
            output: instr[3].parse()?,
        })
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {}", self.op, self.op1, self.op2, self.output)
    }
}