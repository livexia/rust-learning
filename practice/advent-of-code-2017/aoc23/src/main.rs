use std::collections::{HashMap, VecDeque};
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

    fn set(&self, registers: &mut HashMap<char, Int>, v: Int) -> Option<Int> {
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
    Sub((Operand, Operand)),
    Mul((Operand, Operand)),
    Mod((Operand, Operand)),
    Rcv(Operand),
    Jgz((Operand, Operand)),
    Jnz((Operand, Operand)),
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
            "sub" => Sub((ops[1].parse()?, ops[2].parse()?)),
            "mul" => Mul((ops[1].parse()?, ops[2].parse()?)),
            "mod" => Mod((ops[1].parse()?, ops[2].parse()?)),
            "rcv" => Rcv(ops[1].parse()?),
            "jgz" => Jgz((ops[1].parse()?, ops[2].parse()?)),
            "jnz" => Jnz((ops[1].parse()?, ops[2].parse()?)),
            _ => return err!("opcode: {}", ops[1]),
        })
    }
}

#[derive(Debug, Clone)]
struct Cpu {
    registers: HashMap<char, Int>,
    pc: Int,
    queue: VecDeque<Int>,
    program: Vec<Instr>,
}

impl Cpu {
    fn new(program: Vec<Instr>) -> Self {
        Self {
            registers: HashMap::new(),
            pc: 0,
            queue: VecDeque::new(),
            program,
        }
    }

    fn execute(&mut self, queue: Option<&mut VecDeque<Int>>) -> Result<Option<Int>> {
        use Instr::*;

        if self.pc as usize >= self.program.len() {
            return err!("finished");
        }
        let temp = self.pc;
        self.pc += 1;
        match &self.program[temp as usize] {
            Set((op1, op2)) => {
                let v = op2.value(&self.registers);
                op1.set(&mut self.registers, v);
            }
            Add((op1, op2)) => {
                let v = op1.value(&self.registers) + op2.value(&self.registers);
                op1.set(&mut self.registers, v);
            }
            Sub((op1, op2)) => {
                let v = op1.value(&self.registers) - op2.value(&self.registers);
                op1.set(&mut self.registers, v);
            }
            Mul((op1, op2)) => {
                let v = op1.value(&self.registers) * op2.value(&self.registers);
                op1.set(&mut self.registers, v);
                return Ok(Some(0));
            }
            Mod((op1, op2)) => {
                let v = op1.value(&self.registers) % op2.value(&self.registers);
                op1.set(&mut self.registers, v);
            }
            Snd(op1) => {
                self.queue.push_back(op1.value(&self.registers));
            }
            Rcv(op1) => {
                if let Some(queue) = queue {
                    if let Some(v) = queue.pop_front() {
                        op1.set(&mut self.registers, v);
                        return Ok(Some(v));
                    } else {
                        self.pc = temp;
                        return err!("waiting data");
                    }
                } else if op1.value(&self.registers) != 0 {
                    let &v = self.queue.back().unwrap();
                    op1.set(&mut self.registers, v);
                    return Ok(Some(v));
                }
            }
            Jgz((op1, op2)) => {
                if op1.value(&self.registers) > 0 {
                    self.pc = temp + op2.value(&self.registers);
                }
            }
            Jnz((op1, op2)) => {
                if op1.value(&self.registers) != 0 {
                    self.pc = temp + op2.value(&self.registers);
                }
            }
        }
        Ok(None)
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

fn is_composite(n: Int) -> bool {
    for f in 2..n {
        if n % f == 0 {
            return true;
        }
    }
    false
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let cpu = parse_input(&input)?;

    part1(&cpu)?;
    part2(&cpu)?;
    Ok(())
}

fn part1(cpu: &Cpu) -> Result<Int> {
    let start = Instant::now();

    let mut cpu = cpu.to_owned();
    let mut result = 0;
    while let Ok(r) = cpu.execute(None) {
        if r.is_some() {
            result += 1;
        }
    }

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(cpu: &Cpu) -> Result<Int> {
    let start = Instant::now();

    let mut cpu = cpu.to_owned();
    cpu.registers.insert('a', 1);

    cpu.pc = 19;
    let b = 109300 + 17 * 1000;
    cpu.registers.insert('d', b - 2);
    cpu.registers.insert('f', 1);
    cpu.registers.insert('b', b);
    cpu.registers.insert('c', 126300);
    cpu.registers.insert('e', b);
    cpu.registers.insert('g', 0);
    // while cpu.execute(None).is_ok() {
    // if cpu.pc == 19 {
    // println!("{}", cpu.pc);
    // println!("{:?}", cpu.registers);
    // }
    // }
    let result = (109300..=126300)
        .step_by(17)
        .filter(|&n| is_composite(n))
        .count() as Int;

    writeln!(io::stdout(), "Part 2: {result}")?;
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
    let cpu = parse_input(input).unwrap();
    assert_eq!(part1(&cpu).unwrap(), 4);

    let input = "snd 1
        snd 2
        snd p
        rcv a
        rcv b
        rcv c
        rcv d";
    let cpu = parse_input(input).unwrap();
    assert_eq!(part2(&cpu).unwrap(), 3);
}

fn input() -> () {
    let mut pc = 0;
    let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h) = (1, 0, 0, 0, 0, 0, 0, 0);

    b = 93; // 0
    c = b; // 1
    if a != 0 {
        // pc == 2 -> 4
        pc += 2;
        b *= 100;
        c += 100_000;
        c = b;
    } else {
        // pc == 3 -> 8
        pc += 5;
    }
    b *= 100; // 4
    b += 100_000; // 5
    c = b; // 6
    c += 17_000; // 7
    f = 1; // 8
    d = 2; // 9
    e = 2; // 10
    g = d; // 11
    g *= e; // 12
    g = g - b; // 13
    if g != 0 {
        // pc == 14 -> 16
        pc += 2;
    }
    f = 0; // 15
    e += 1; // 16
    g = e; // 17
    g -= b; // 18
    if g != 0 {
        // pc == 19 -> 11
        pc -= 8;
    }
    d += 1; // 20
    g = d; // 21
    g -= b; // 22
    if g != 0 {
        // pc == 23
        pc -= 13;
    } else if f != 0 {
        // pc == 24
        pc += 2;
    }
    h += 1; // 25
    g = b; // 26
    g -= c; // 27
    if g != 0 {
        // pc == 28
        pc += 2;
    } else {
        // pc == 29
        // return h;
    }
    b += 17; // 30
    pc -= 23; // pc == 31
}
