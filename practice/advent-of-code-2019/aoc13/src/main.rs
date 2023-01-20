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
type Coord = (Int, Int);

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let program = parse_input(&input)?;

    part1(&program)?;
    part2(&program)?;
    part2_with_proper_solution(&program)?;
    Ok(())
}

fn part1(program: &[Int]) -> Result<usize> {
    let start = Instant::now();

    let mut arcade = Computer::new(program);
    let mut grid = HashMap::new();
    let mut block_count = 0;
    arcade.run();
    update_grid(arcade.take_output(), &mut grid, &mut block_count);
    let result = block_count;

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(program: &[Int]) -> Result<Int> {
    let start = Instant::now();

    let mut arcade = Computer::new(program);
    let mut grid = HashMap::new();
    arcade.program[0] = 2;
    let mut status = arcade.run();
    let mut block_count = 0;

    let (mut score, _, _) = update_grid(arcade.take_output(), &mut grid, &mut block_count);
    while block_count != 0 {
        if status == 3 {
            arcade.add_input(1);
            status = arcade.run();
            score = score.max(update_grid(arcade.take_output(), &mut grid, &mut block_count).0);
        } else {
            writeln!(io::stdout(), "arcade exit with code {status}")?;
            break;
        }
    }

    writeln!(io::stdout(), "Part 2: {score}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(score)
}

// play the game via https://www.reddit.com/r/adventofcode/comments/e9zgse/comment/fan8x27
fn part2_with_proper_solution(program: &[Int]) -> Result<Int> {
    let start = Instant::now();

    let mut arcade = Computer::new(program);
    let mut grid = HashMap::new();
    arcade.program[0] = 2;
    let mut status = arcade.run();
    let mut block_count = 0;

    let (mut score, mut ball_pos, mut paddle_pos) =
        update_grid(arcade.take_output(), &mut grid, &mut block_count);
    while block_count != 0 {
        if status == 3 {
            match ball_pos.0.cmp(&paddle_pos.0) {
                std::cmp::Ordering::Less => arcade.add_input(-1),
                std::cmp::Ordering::Equal => arcade.add_input(0),
                std::cmp::Ordering::Greater => arcade.add_input(1),
            }
            status = arcade.run();
            (score, ball_pos, paddle_pos) =
                update_grid(arcade.take_output(), &mut grid, &mut block_count);
        } else {
            writeln!(io::stdout(), "arcade exit with code {status}")?;
            break;
        }
    }

    writeln!(io::stdout(), "Part 2: {score}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(score)
}

fn update_grid(
    output: Vec<Int>,
    grid: &mut HashMap<Coord, Int>,
    block_count: &mut usize,
) -> (Int, Coord, Coord) {
    let mut score = 0;
    let mut ball_pos = (0, 0);
    let mut paddle_pos = (0, 0);
    for instr in output.chunks(3) {
        let (x, y, id) = (instr[0], instr[1], instr[2]);
        if x == -1 && y == 0 {
            score = id;
            continue;
        }
        match id {
            2 => *block_count += 1,
            3 => paddle_pos = (x, y),
            4 => ball_pos = (x, y),
            _ => (),
        }
        if let Some(pre_id) = grid.insert((x, y), id) {
            if id != 2 && pre_id == 2 {
                *block_count -= 1;
            }
        }
    }
    (score, ball_pos, paddle_pos)
}

#[allow(dead_code)]
fn draw(grid: &HashMap<Coord, Int>) -> String {
    let mut s = String::new();
    let (mut min_x, mut min_y) = (Int::MAX, Int::MAX);
    let (mut max_x, mut max_y) = (Int::MIN, Int::MIN);
    for &(x, y) in grid.keys() {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some(&id) = grid.get(&(x, y)) {
                match id {
                    1 => s.push('W'),
                    2 => s.push('B'),
                    3 => s.push('p'),
                    4 => s.push('o'),
                    _ => s.push(' '),
                }
            } else {
                s.push(' ')
            };
        }
        s.push('\n')
    }
    s
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

    fn take_output(&mut self) -> Vec<Int> {
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
