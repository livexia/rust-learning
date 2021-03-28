use std::io::{self, Read, Write};
use std::error::Error;
use std::fmt;
use std::result;
use std::str::FromStr;

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

    part1(&mut program, true)?;
    // part2(&mut program, false)?;

    Ok(())
}

fn part1(program: &mut Program, debug: bool) -> Result<()> {
    program.init(vec![200000, 0, 0, 0, 0, 0]);
    program.run(debug)?;
    writeln!(io::stdout(), "part1 answer: {}", program.registers[0])?;
    Ok(())
}

fn part2(program: &mut Program, debug: bool) -> Result<()> {
    program.init(vec![1, 0, 0, 0, 0, 0]);
    program.run(debug)?;
    writeln!(io::stdout(), "part2 answer: {}", program.registers[0])?;
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

    fn run(&mut self, debug: bool) -> Result<()> {
        let n = self.instructions.len();
        let mut count = 0;

        while self.ip.1 < n {
            self.registers[self.ip.0] = self.ip.1;
            if debug {
                print!("ip={:?} {:?} {} ", self.ip, self.registers, self.instructions[self.ip.1]);
            }
            self.execute(self.ip.1)?;
            self.ip.1 = self.registers[self.ip.0] + 1;
            if debug {
                println!(" {:?}", self.registers);
            }
            count += 1;
            if count > 40 {
                break;
            }
        }

        Ok(())
    }

    fn execute(&mut self, ip: usize) -> Result<()> {
        let instr = &self.instructions[ip];
        let (op, a, b, c) = (instr.op.clone(), instr.op1, instr.op2, instr.output);
        match op.as_str() {
            "addr" => self.addr(a as Register, b as Register, c),
            "addi" => self.addi(a as Register, b, c),
            "mulr" => self.mulr(a as Register, b as Register, c),
            "muli" => self.muli(a as Register, b, c),
            "banr" => self.banr(a as Register, b as Register, c),
            "bani" => self.bani(a as Register, b, c),
            "borr" => self.borr(a as Register, b as Register, c),
            "bori" => self.bori(a as Register, b, c),
            "setr" => self.setr(a as Register, c),
            "seti" => self.seti(a, c),
            "gtri" => self.gtri(a as Register, b, c),
            "gtir" => self.gtir(a, b as Register, c),
            "gtrr" => self.gtrr(a as Register, b as Register, c),
            "eqri" => self.eqri(a as Register, b, c),
            "eqir" => self.eqir(a, b as Register, c),
            "eqrr" => self.eqrr(a as Register, b as Register, c),
            _ => return err!("wrong instruction: {}", instr),
        }
        Ok(())
    }

    fn addr(&mut self, a: Register, b: Register, c: Register) {
        self.registers[c] = self.registers[a] + self.registers[b]
    }
    
    fn addi(&mut self, a: Register, b: Value, c: Register) {
        self.registers[c] = b + self.registers[a]
    }
    
    fn mulr(&mut self, a: Register, b: Register, c: Register) {
        self.registers[c] = self.registers[a] * self.registers[b]
    }
    
    fn muli(&mut self, a: Register, b: Value, c: Register) {
        self.registers[c] = b * self.registers[a]
    }
    
    fn banr(&mut self, a: Register, b: Register, c: Register) {
        self.registers[c] = self.registers[a] & self.registers[b]
    }
    
    fn bani(&mut self, a: Register, b: Value, c: Register) {
        self.registers[c] = b & self.registers[a]
    }
    
    fn borr(&mut self, a: Register, b: Register, c: Register) {
        self.registers[c] = self.registers[a] | self.registers[b]
    }
    
    fn bori(&mut self, a: Register, b: Value, c: Register) {
        self.registers[c] = b | self.registers[a]
    }
    
    fn setr(&mut self, a: Register, c: Register) {
        self.registers[c] = self.registers[a]
    }
    
    fn seti(&mut self, a: Value, c: Register) {
        self.registers[c] = a
    }
    
    fn gtir(&mut self, a: Value, b: Register, c: Register) {
        self.registers[c] = if a > self.registers[b] { 1 } else { 0 }
    }
    
    fn gtri(&mut self, a: Register, b: Value, c: Register) {
        self.registers[c] = if self.registers[a] > b { 1 } else { 0 }
    }
    
    fn gtrr(&mut self, a: Register, b: Register, c: Register) {
        self.registers[c] = if self.registers[a] > self.registers[b] { 1 } else { 0 }
    }
    
    fn eqir(&mut self, a: Value, b: Register, c: Register) {
        self.registers[c] = if a == self.registers[b] { 1 } else { 0 }
    }
    
    fn eqri(&mut self, a: Register, b: Value, c: Register) {
        self.registers[c] = if self.registers[a] == b { 1 } else { 0 }
    }
    
    fn eqrr(&mut self, a: Register, b: Register, c: Register) {
        self.registers[c] = if self.registers[a] == self.registers[b] { 1 } else { 0 }
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