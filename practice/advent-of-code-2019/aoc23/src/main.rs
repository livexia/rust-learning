use std::collections::VecDeque;
use std::error::Error;
use std::io::{self, Read, Write};
use std::iter::repeat;
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

    let result;

    let mut computers = init_computers(50, program);
    let mut queues: Vec<VecDeque<(Int, Int)>> = vec![VecDeque::new(); 50];
    'outer: loop {
        for (i, c) in computers.iter_mut().enumerate() {
            c.run();

            if let Some((_, y)) = send(c, &mut queues) {
                result = y;
                break 'outer;
            };
            recv(c, &mut queues[i]);
        }
    }

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(program: &[Int]) -> Result<Int> {
    let start = Instant::now();

    let result;

    let mut computers = init_computers(50, program);
    let mut queues: Vec<VecDeque<(Int, Int)>> = vec![VecDeque::new(); 50];
    let mut nat = None;
    let mut last_y = None;
    'outer: loop {
        let mut idle = 0u64;
        for (i, c) in computers.iter_mut().enumerate() {
            c.run();

            if let Some((x, y)) = send(c, &mut queues) {
                nat = Some((x, y));
            }
            if !recv(c, &mut queues[i]) {
                idle |= 1 << i;
            } else {
                idle &= !(1 << i);
            };
        }
        if idle.count_ones() == 50 && queues.iter().all(|q| q.is_empty()) {
            if let Some((x, y)) = nat {
                queues[0].push_back((x, y));
                if last_y == Some(y) {
                    result = y;
                    break 'outer;
                }
                last_y = Some(y);
            }
        }
    }

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn send(computer: &mut Computer, queues: &mut [VecDeque<(Int, Int)>]) -> Option<(Int, Int)> {
    let output = &mut computer.output;
    while !output.is_empty() && output.len() % 3 == 0 {
        let (y, x, dest) = (
            output.pop().unwrap(),
            output.pop().unwrap(),
            output.pop().unwrap(),
        );
        if dest < 50 {
            queues[dest as usize].push_back((x, y));
        } else if dest == 255 {
            return Some((x, y));
        }
    }
    None
}

fn recv(computer: &mut Computer, queue: &mut VecDeque<(Int, Int)>) -> bool {
    while let Some((x, y)) = queue.pop_front() {
        computer.add_input(x);
        computer.run();
        computer.add_input(y);
    }
    if computer.input.is_empty() {
        computer.add_input(-1);
        false
    } else {
        true
    }
}

fn init_computers(n: usize, program: &[Int]) -> Vec<Computer> {
    let mut r = vec![];
    for i in 0..n {
        let mut c = Computer::new(program);
        c.add_input(i as Int);
        r.push(c);
    }
    r
}

struct Computer {
    program: Vec<Int>,
    pc: Addr,
    base: Int,
    input: Vec<Int>,
    output: Vec<Int>,
}

impl Computer {
    fn new(program: &[Int]) -> Self {
        Self {
            program: program.to_owned(),
            pc: 0,
            base: 0,
            input: vec![],
            output: vec![],
        }
    }

    fn add_input(&mut self, i: Int) {
        self.input.push(i);
        self.input.reverse();
    }

    fn run(&mut self) -> Int {
        while self.pc < self.program.len() {
            let (opcode, f1, f2, f3) = parse_opcode(self.get(self.pc));
            if opcode == 99 {
                return 99;
            }
            let op1 = self.addr_lookup(self.pc + 1, f1);
            match opcode {
                1 | 2 | 7 | 8 => {
                    self.instr_with_four(self.pc, opcode, f1, f2, f3);
                    self.pc += 4;
                }
                3 => {
                    if let Some(i) = self.input.pop() {
                        self.set(op1, i);
                        self.pc += 2
                    } else {
                        // self.set(op1, -1);
                        // self.pc += 2
                        return 3;
                    }
                }
                4 => {
                    self.output.push(self.get(op1));
                    self.pc += 2;
                    // return 4;
                }
                5 => {
                    let op2 = self.addr_lookup(self.pc + 2, f2);
                    if self.get(op1) != 0 {
                        self.pc = self.get(op2) as usize;
                    } else {
                        self.pc += 3
                    }
                }
                6 => {
                    let op2 = self.addr_lookup(self.pc + 2, f2);
                    if self.get(op1) == 0 {
                        self.pc = self.get(op2) as usize;
                    } else {
                        self.pc += 3
                    }
                }
                9 => {
                    self.base += self.get(op1);
                    self.pc += 2;
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

    fn addr_lookup(&self, pc: Addr, flag: Int) -> Addr {
        if flag == 0 {
            assert!(self.get(pc) >= 0);
            self.get(pc) as usize
        } else if flag == 1 {
            pc
        } else if flag == 2 {
            (self.get(pc) + self.base) as usize
        } else {
            unimplemented!(
                "Only support Parameters in mode 0, 1, 2, mode {} not supported",
                flag
            )
        }
    }

    fn instr_with_four(&mut self, pc: Addr, opcode: Int, f1: Int, f2: Int, f3: Int) {
        let op1 = self.get(self.addr_lookup(pc + 1, f1));
        let op2 = self.get(self.addr_lookup(pc + 2, f2));
        let dest = self.addr_lookup(pc + 3, f3);
        self.set(
            dest,
            match opcode {
                1 => op1 + op2,
                2 => op1 * op2,
                7 => (op1 < op2) as Int,
                8 => (op1 == op2) as Int,
                _ => unreachable!(),
            },
        )
    }

    fn get(&self, addr: Addr) -> Int {
        *self.program.get(addr).unwrap_or(&0)
    }

    fn set(&mut self, addr: Addr, value: Int) {
        if addr > self.program.len() - 1 {
            let mut l = addr + 1 - self.program.len();
            if l < 50 {
                l = 50;
            }
            self.program.extend(repeat(0).take(l));
        }
        self.program[addr] = value;
    }
}

fn parse_opcode(opcode: Int) -> (Int, Int, Int, Int) {
    (
        opcode % 100,
        (opcode / 100) % 10,
        (opcode / 1000) % 10,
        (opcode / 10000) % 10,
    )
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
    let mut computer = Computer::new(&[
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ]);
    computer.run();
    assert_eq!(
        computer.output,
        vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,]
    );

    let mut computer = Computer::new(&[1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
    computer.run();
    assert_eq!(format!("{}", computer.output.last().unwrap()).len(), 16);

    let mut computer = Computer::new(&[104, 1125899906842624, 99]);
    computer.run();
    assert_eq!(*computer.output.last().unwrap(), 1125899906842624);
}
