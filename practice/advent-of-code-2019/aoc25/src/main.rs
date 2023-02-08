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
    Ok(())
}

fn part1(program: &[Int]) -> Result<Int> {
    let start = Instant::now();

    let mut computer = Computer::new(program);
    computer.run();
    println!("{}", run_command(&mut computer, "north")); // hull breach go north
    println!("{}", run_command(&mut computer, "take mug"));

    println!("{}", run_command(&mut computer, "north"));
    println!("{}", run_command(&mut computer, "take food ration"));

    println!("{}", run_command(&mut computer, "south"));
    println!("{}", run_command(&mut computer, "east"));
    println!("{}", run_command(&mut computer, "north"));
    // println!("{}", run_command(&mut computer, "take photons")); // eaten by a Grue
    println!("{}", run_command(&mut computer, "east"));
    println!("{}", run_command(&mut computer, "take semiconductor"));

    // println!("{}", run_command(&mut computer, "east"));
    // println!("{}", run_command(&mut computer, "take escape pod")); // You're launched into space! Bye!

    println!("{}", run_command(&mut computer, "west"));
    println!("{}", run_command(&mut computer, "south"));
    println!("{}", run_command(&mut computer, "west"));

    println!("{}", run_command(&mut computer, "south")); // get hull breach
    println!("{}", run_command(&mut computer, "south")); // hull breach go south

    // println!("{}", run_command(&mut computer, "take giant electromagnet")); // You can't move
    println!("{}", run_command(&mut computer, "east"));
    println!("{}", run_command(&mut computer, "take mouse"));

    println!("{}", run_command(&mut computer, "south")); // at Security Checkpoint

    println!("{}", run_command(&mut computer, "north"));
    println!("{}", run_command(&mut computer, "west"));
    println!("{}", run_command(&mut computer, "north")); // back hull breach

    println!("{}", run_command(&mut computer, "east")); // hull breach go east get Engineering
    println!("{}", run_command(&mut computer, "take ornament"));

    println!("{}", run_command(&mut computer, "north")); // Engineering -> Observatory
    println!("{}", run_command(&mut computer, "take coin"));

    println!("{}", run_command(&mut computer, "east")); // Observatory -> Stables
    println!("{}", run_command(&mut computer, "take mutex"));

    println!("{}", run_command(&mut computer, "west")); // Stables -> Observatory
    println!("{}", run_command(&mut computer, "south")); // Observatory -> Engineering

    println!("{}", run_command(&mut computer, "west")); // Engineering -> hull Breach

    println!("{}", run_command(&mut computer, "east")); // hull Breach -> Engineering
    println!("{}", run_command(&mut computer, "east")); // Engineering -> Warp Drive Maintenance
    println!("{}", run_command(&mut computer, "take candy cane"));
    println!("{}", run_command(&mut computer, "west"));
    println!("{}", run_command(&mut computer, "west")); // back to hull breach

    // hull breach to checkpoint
    println!("{}", run_command(&mut computer, "south"));
    println!("{}", run_command(&mut computer, "east"));
    println!("{}", run_command(&mut computer, "south"));
    // test weight
    // println!("{}", run_command(&mut computer, "drop food ration"));
    // println!("{}", run_command(&mut computer, "drop candy cane"));
    // println!("{}", run_command(&mut computer, "drop mouse"));
    // println!("{}", run_command(&mut computer, "drop mug"));
    // println!("{}", run_command(&mut computer, "drop coin"));
    // println!("{}", run_command(&mut computer, "drop ornament"));
    // println!("{}", run_command(&mut computer, "drop semiconductor"));
    // println!("{}", run_command(&mut computer, "drop mutex"));

    println!("{}", run_command(&mut computer, "west")); // test weight

    // println!("{}", run_command(&mut computer, "south")); // hull Breach -> Hot Chocolate Fountain

    // println!("{}", run_command(&mut computer, "west")); // Hot Chocolate Fountain -> Hallway
    // println!("{}", run_command(&mut computer, "north")); // Hallway -> Storage

    // println!("{}", run_command(&mut computer, "take molten lava")); // melt

    // println!("{}", run_command(&mut computer, "east")); // Hot Chocolate Fountain -> Corridor

    // println!("{}", run_command(&mut computer, "north")); // hull Breach -> Crew Quarters

    // println!("{}", run_command(&mut computer, "east")); // Crew Quarters -> Holodeck
    // println!("{}", run_command(&mut computer, "north")); // Holodeck -> Kitchen
    // println!("{}", run_command(&mut computer, "north")); // Kitchen -> Kitchen

    // println!("{}", run_command(&mut computer, "north")); // Crew Quarters -> Sick Bay

    // println!("{}", run_command(&mut computer, "inv"));
    let output = 0;

    writeln!(io::stdout(), "Part 1: {output}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(output)
}

fn run_command(computer: &mut Computer, command: &str) -> String {
    computer.input = command_to_input(command);
    computer.run();
    let mut output = vec![];
    std::mem::swap(&mut output, &mut computer.output);
    show_output(&output)
}

fn command_to_input(command: &str) -> Vec<Int> {
    command
        .bytes()
        .map(|b| b as Int)
        .chain([b'\n' as Int])
        .rev()
        .collect()
}

fn show_output(output: &[Int]) -> String {
    let mut s = String::new();
    for &i in output {
        s.push(i as u8 as char);
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
