use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Int = i64;

#[derive(Debug, Clone)]
enum Operand {
    Register(char),
    Immediate(Int),
}

impl Operand {
    fn value(&self, registers: &HashMap<char, Int>) -> Int {
        match self {
            Operand::Register(c) => *registers.get(c).unwrap_or(&0),
            Operand::Immediate(i) => *i,
        }
    }

    fn set(&self, registers: &mut HashMap<char, Int>, v: Int) -> Option<i64> {
        match self {
            Operand::Register(c) => registers.insert(*c, v),
            Operand::Immediate(_) => None,
        }
    }
}

impl FromStr for Operand {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s.parse() {
            Ok(i) => Operand::Immediate(i),
            Err(_) => Operand::Register(s.chars().next().unwrap()),
        })
    }
}

#[derive(Clone, Debug)]
enum Instr {
    Snd(Operand),
    Set((Operand, Operand)),
    Add((Operand, Operand)),
    Mul((Operand, Operand)),
    Mod((Operand, Operand)),
    Rcv(Operand),
    Jgz((Operand, Operand)),
}

impl FromStr for Instr {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        use Instr::*;

        let ops: Vec<_> = s.split_whitespace().collect();
        Ok(match ops[0] {
            "snd" => Snd(ops[1].parse()?),
            "set" => Set((ops[1].parse()?, ops[2].parse()?)),
            "add" => Add((ops[1].parse()?, ops[2].parse()?)),
            "mul" => Mul((ops[1].parse()?, ops[2].parse()?)),
            "mod" => Mod((ops[1].parse()?, ops[2].parse()?)),
            "rcv" => Rcv(ops[1].parse()?),
            "jgz" => Jgz((ops[1].parse()?, ops[2].parse()?)),
            _ => return err!("opcode: {}", ops[1]),
        })
    }
}

struct Cpu {
    registers: HashMap<char, Int>,
    pc: Int,
    buffer: Int,
    program: Vec<Instr>,
}

impl Cpu {
    fn new(program: Vec<Instr>) -> Self {
        Self {
            registers: HashMap::new(),
            pc: 0,
            buffer: 0,
            program,
        }
    }

    fn reset(&mut self) {
        self.registers.clear();
        self.pc = 0;
        self.buffer = 0;
    }

    fn execute(&mut self) -> Option<Int> {
        use Instr::*;

        let temp = self.pc;
        self.pc += 1;
        match &self.program[temp as usize] {
            Snd(op1) => {
                self.buffer = op1.value(&self.registers);
            }
            Set((op1, op2)) => {
                let v = op2.value(&self.registers);
                op1.set(&mut self.registers, v);
            }
            Add((op1, op2)) => {
                let v = op1.value(&self.registers) + op2.value(&self.registers);
                op1.set(&mut self.registers, v);
            }
            Mul((op1, op2)) => {
                let v = op1.value(&self.registers) * op2.value(&self.registers);
                op1.set(&mut self.registers, v);
            }
            Mod((op1, op2)) => {
                let v = op1.value(&self.registers) % op2.value(&self.registers);
                op1.set(&mut self.registers, v);
            }
            Rcv(op1) => {
                if op1.value(&self.registers) != 0 {
                    op1.set(&mut self.registers, self.buffer);
                    return Some(self.buffer);
                }
            }
            Jgz((op1, op2)) => {
                if op1.value(&self.registers) > 0 {
                    self.pc = temp + op2.value(&self.registers);
                }
            }
        }
        None
    }
}

fn parse_input(input: &str) -> Result<Cpu> {
    Ok(Cpu::new(
        input
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.parse())
            .collect::<Result<Vec<_>>>()?,
    ))
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut cpu = parse_input(&input)?;

    part1(&mut cpu)?;
    // part2()?;
    Ok(())
}

fn part1(cpu: &mut Cpu) -> Result<Int> {
    let start = Instant::now();

    let result;
    loop {
        if let Some(r) = cpu.execute() {
            result = r;
            break;
        }
    }

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

#[test]
fn example_input() {
    let input = "
        set a 1
        add a 2
        mul a a
        mod a 5
        snd a
        set a 0
        rcv a
        jgz a -1
        set a 1
        jgz a -2
        ";
    let mut cpu = parse_input(input).unwrap();
    assert_eq!(part1(&mut cpu).unwrap(), 4);
}
