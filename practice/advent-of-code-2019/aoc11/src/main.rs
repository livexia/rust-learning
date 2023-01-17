use std::collections::HashMap;
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
    let mut robot = Robot::new();
    let mut grid = HashMap::new();
    while robot.paint(&mut grid, &mut computer, 0) != 99 {}
    let output = grid.len();

    writeln!(io::stdout(), "Part 1: {output}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(output)
}

fn part2(program: &[Int]) -> Result<String> {
    let start = Instant::now();

    let mut computer = Computer::new(program);
    let mut robot = Robot::new();
    let mut grid = HashMap::new();
    while robot.paint(&mut grid, &mut computer, 1) != 99 {}
    let output = draw(&grid, 1);

    writeln!(io::stdout(), "Part 2: \n{output}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(output)
}

fn draw(grid: &HashMap<Coord, u8>, default_color: u8) -> String {
    let mut s = String::new();
    let (mut min_x, mut min_y) = (i32::MAX, i32::MAX);
    let (mut max_x, mut max_y) = (i32::MIN, i32::MIN);
    for &(x, y) in grid.keys() {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if let Some(&color) = grid.get(&(x, y)) {
                if color == default_color {
                    s.push('#')
                } else {
                    s.push(' ')
                }
            } else {
                s.push(' ')
            };
        }
        s.push('\n')
    }
    s
}

struct Robot {
    facing: i8, // up:0 , right: 1, down: 2, left: 3
    x: i32,
    y: i32,
}

impl Robot {
    fn new() -> Self {
        Robot {
            facing: 0,
            x: 0,
            y: 0,
        }
    }

    fn turn(&mut self, dir: u8) {
        if dir == 0 {
            self.facing = (self.facing - 1).rem_euclid(4);
        } else if dir == 1 {
            self.facing = (self.facing + 1).rem_euclid(4);
        } else {
            unreachable!("turn dir: {dir}")
        }
        self.move_forward()
    }

    fn move_forward(&mut self) {
        match self.facing {
            0 => self.x -= 1,
            1 => self.y += 1,
            2 => self.x += 1,
            3 => self.y -= 1,
            _ => unreachable!(),
        }
    }

    fn coord(&self) -> Coord {
        (self.x, self.y)
    }

    fn paint(
        &mut self,
        grid: &mut HashMap<Coord, u8>,
        computer: &mut Computer,
        default_color: u8,
    ) -> Int {
        let &input = grid.get(&self.coord()).unwrap_or(&default_color);
        computer.add_input(input as Int);
        let (status, output) = Robot::run_program(computer);
        if status == 4 {
            grid.insert(self.coord(), output as u8);
        } else if status == 99 {
            return 99;
        };

        let (status, output) = Robot::run_program(computer);
        if status == 4 {
            self.turn(output as u8)
        } else if status == 99 {
            return 99;
        }
        0
    }

    fn run_program(computer: &mut Computer) -> (Int, Int) {
        let status = computer.run();
        if status != 4 && status != 99 {
            unreachable!("{status}")
        }
        (status, computer.get_output().unwrap_or(0))
    }
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

    fn get_output(&self) -> Option<Int> {
        self.output.last().copied()
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
