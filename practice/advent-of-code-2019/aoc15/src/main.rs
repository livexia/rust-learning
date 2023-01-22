use std::collections::{HashSet, VecDeque};
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
type Coord = (i32, i32);

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let program = parse_input(&input)?;

    part1(&program)?;
    part2(&program)?;
    Ok(())
}

fn part1(program: &[Int]) -> Result<usize> {
    let start = Instant::now();

    let mut computer = Computer::new(program);
    let (result, _) = droid_bfs(&mut computer, &mut HashSet::new(), 1);

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(program: &[Int]) -> Result<usize> {
    let start = Instant::now();

    let mut computer = Computer::new(program);
    let mut grid = HashSet::new();
    let (_, oxygen) = droid_bfs(&mut computer, &mut grid, 2);

    let result = oxygen_bfs(&grid, oxygen);
    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn droid_bfs(computer: &mut Computer, grid: &mut HashSet<Coord>, part: u8) -> (usize, Coord) {
    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), computer.clone()));
    let mut visited = HashSet::new();
    let mut depth = 1;
    let mut oxygen = (0, 0);
    while !queue.is_empty() {
        let length = queue.len();
        for _ in 0..length {
            let (cur, computer) = queue.pop_front().unwrap();
            if visited.insert(cur) {
                grid.insert(cur);
                for input in 1..5 {
                    let mut c = computer.clone();
                    c.add_input(input);
                    if c.run() == 4 {
                        if let Some(&output) = c.get_output().last() {
                            let next = move_droid(cur, input);
                            if output == 2 {
                                oxygen = next;
                                if part == 1 {
                                    return (depth, oxygen);
                                } else {
                                    queue.push_back((next, c.clone()))
                                }
                            } else if output == 0 {
                                continue;
                            } else {
                                queue.push_back((next, c.clone()))
                            }
                        }
                    }
                }
            }
        }
        depth += 1;
    }

    (depth, oxygen)
}

fn oxygen_bfs(grid: &HashSet<Coord>, coord: Coord) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back(coord);
    let mut visited = HashSet::new();
    let mut depth = 0;
    while !queue.is_empty() {
        let length = queue.len();
        for _ in 0..length {
            let cur = queue.pop_front().unwrap();
            if visited.insert(cur) {
                for input in 1..5 {
                    let next = move_droid(cur, input);
                    if grid.contains(&next) {
                        queue.push_back(next);
                    }
                }
            }
        }
        depth += 1;
    }
    depth
}

fn move_droid(coord: Coord, input: Int) -> Coord {
    let (x, y) = coord;
    match input {
        1 => (x - 1, y),
        2 => (x + 1, y),
        3 => (x, y - 1),
        4 => (x, y + 1),
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone)]
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

    fn get_output(&mut self) -> Vec<Int> {
        self.output.drain(0..).collect()
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
                        return 3;
                    }
                }
                4 => {
                    self.output.push(self.get(op1));
                    self.pc += 2;
                    return 4;
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
    // let mut computer = Computer::new(&[
    //     109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    // ]);
    // computer.run();
    // assert_eq!(
    //     computer.output,
    //     vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,]
    // );

    let mut computer = Computer::new(&[1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
    computer.run();
    assert_eq!(format!("{}", computer.output.last().unwrap()).len(), 16);

    let mut computer = Computer::new(&[104, 1125899906842624, 99]);
    computer.run();
    assert_eq!(*computer.output.last().unwrap(), 1125899906842624);
}
