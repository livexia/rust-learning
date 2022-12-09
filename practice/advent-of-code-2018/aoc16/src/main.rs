use std::io::{self, Read, Write};
use std::error::Error;
use std::fmt;
use std::result;
use std::str::FromStr;
use std::collections::HashSet;
use std::collections::HashMap;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn Error>>;

type Value = i32;
type Register = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Op {
    ADDr, ADDi,
    MULr, MULi,
    BANr, BANi,
    BORr, BORi,
    SETr, SETi,
    GTri, GTir, GTrr,
    EQri, EQir, EQrr,
}

fn main() -> Result<()>{

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    let mut samples = vec![];
    let mut test = vec![];
    let mut is_smaple = true;
    let mut sample = vec![];
    for line in input.lines() {
        if let Some(l) = line.strip_prefix("Before: [") {
            let l: Vec<i32> = l.strip_suffix("]")
                .unwrap().split(", ")
                .map(|f| f.parse().unwrap()).collect();
            is_smaple = true;
            sample.push(l)
        } else if let Some(l) = line.strip_prefix("After:  [") {
            let l: Vec<i32> = l.strip_suffix("]")
                .unwrap().split(", ")
                .map(|f| f.parse().unwrap()).collect();
            is_smaple = false;
            sample.push(l);
            samples.push(sample.clone());
            sample.clear();
        } else if is_smaple {
            let l: Vec<i32> = line.trim().split(" ")
                .map(|f| f.parse().unwrap()).collect();
            sample.push(l)
        } else if line.trim().len() == 0 {
            continue;
        } else if !is_smaple {
            let instr: Instruction = line.parse()?;
            test.push(instr);
        }
    }

    let mut machine = Machine::new();
    part1(&samples,&mut machine)?;
    part2(&test,&mut machine)?;

    Ok(())
}

fn part1(samples: &Vec<Vec<Vec<i32>>>, machine: &mut Machine) -> Result<()> {
    use crate::Op::*;
    
    let ops = vec![
        ADDr, ADDi,
        MULr, MULi,
        BANr, BANi,
        BORr, BORi,
        SETr, SETi,
        GTri, GTir, GTrr,
        EQri, EQir, EQrr,
    ];
    let mut answer = 0;
    let mut opcodes: Vec<HashSet<Op>> = vec![HashSet::new(); 16];
    for sample in samples {
        let before = sample[0].clone();
        let after = sample[2].clone();

        let instr = sample[1].clone();
        let (opcode, op1, op2, output) = (instr[0] as usize, instr[1], instr[2], instr[3] as usize);
        let mut count = 0;

        for &op in &ops {
            machine.registers = before.clone();
            machine.execute(&Instruction{ opcode, op1, op2, output }, op)?;
            if machine.registers == after {
                count += 1;
                opcodes[opcode].insert(op);
            }
        }
        if count > 2 {
            answer += 1;
        }
    }

    writeln!(io::stdout(), "part1 ansewr: {}", answer)?;

    while machine.opcodes.len() < 16 {
        for i in 0..16 {
            if opcodes[i].len() == 1 {
                machine.opcodes.insert(i, opcodes[i].iter().next().unwrap().clone());
            }
            for (_, op) in &machine.opcodes {
                opcodes[i].remove(op);
            }
        }
    }
    Ok(())
}

fn part2(test: &Vec<Instruction>, machine: &mut Machine) -> Result<()> {
    for instr in test {
        let op = machine.opcodes.get(&instr.opcode).unwrap().clone();
        machine.execute(&instr, op)?;
    }

    writeln!(io::stdout(), "part2 ansewr: {:?}", machine.registers[0])?;

    Ok(())
}

#[derive(Debug)]
struct Machine {
    registers: Vec<Value>,
    opcodes: HashMap<usize, Op>,
}

impl Machine {
    fn new() -> Self {
        Self {
            registers: vec![0; 4],
            opcodes: HashMap::new(),
        }
    }

    fn execute(&mut self, instr: &Instruction, op: Op) -> Result<()>{
        use crate::Op::*;
        let (a, b, c) = (instr.op1, instr.op2, instr.output);
        match op {
            ADDr => self.addr(a as Register, b as Register, c),
            ADDi => self.addi(a as Register, b, c),
            MULr => self.mulr(a as Register, b as Register, c),
            MULi => self.muli(a as Register, b, c),
            BANr => self.banr(a as Register, b as Register, c),
            BANi => self.bani(a as Register, b, c),
            BORr => self.borr(a as Register, b as Register, c),
            BORi => self.bori(a as Register, b, c),
            SETr => self.setr(a as Register, c),
            SETi => self.seti(a, c),
            GTri => self.gtri(a as Register, b, c),
            GTir => self.gtir(a, b as Register, c),
            GTrr => self.gtrr(a as Register, b as Register, c),
            EQri => self.eqri(a as Register, b, c),
            EQir => self.eqir(a, b as Register, c),
            EQrr => self.eqrr(a as Register, b as Register, c),
        }
        Ok(())
    }

    fn addr(&mut self, a: Register, b: Register, c: Register) {
        self.registers[c] = self.registers[a] + self.registers[b]
    }
    
    fn addi(&mut self, a: Register, b: Value, c: Register) {
        self.registers[c] = b + self.registers[a] as i32
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

#[derive(Debug)]
struct Instruction {
    opcode: usize,
    op1: i32,
    op2: i32,
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
            opcode: instr[0].parse()?,
            op1: instr[1].parse()?,
            op2: instr[2].parse()?,
            output: instr[3].parse()?,
        })
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {}", self.opcode, self.op1, self.op2, self.output)
    }
}
