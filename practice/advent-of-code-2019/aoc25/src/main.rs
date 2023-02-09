use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};
use std::iter::repeat;
use std::mem::swap;
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

fn part1(program: &[Int]) -> Result<i128> {
    let start = Instant::now();

    let mut computer = Computer::new(program);
    computer.run();
    let mut output = vec![];
    swap(&mut output, &mut computer.output);
    let output = output_to_string(&output);
    let mut visited = HashSet::new();
    let mut found_items = HashSet::new();
    let result = dfs(&mut computer, output, &mut visited, &mut found_items).unwrap();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn dfs(
    computer: &mut Computer,
    output: String,
    visited: &mut HashSet<(String, String)>,
    found_items: &mut HashSet<String>,
) -> Option<i128> {
    let cur = cur_location(&output);
    let leads = leads(&output);

    if cur == "Pressure-Sensitive Floor" {
        let entry_command = rev_dir(&leads[0]);
        let r = run_command(computer, "inv").1;
        let items = parse_inv(&r);
        for i in 0..256 {
            let mut c = computer.clone();
            for (j, item) in items.iter().enumerate() {
                if i & (1 << j) != 0 {
                    run_command(&mut c, &format!("drop {item}"));
                }
            }
            let (_, r) = run_command(&mut c, entry_command);
            if !r.contains("you are ejected back to the checkpoint") {
                return r.split(' ').find_map(|w| w.parse::<i128>().ok());
            }
        }
    }
    let items = items(&output);
    for item in items {
        if found_items.insert(item.clone()) {
            if item == "infinite loop" {
                continue;
            }
            let mut c = computer.clone();
            let command = format!("take {item}");
            let (status, _r) = run_command(&mut c, &command);
            if !leads.is_empty() {
                let (_, r) = run_command(&mut c, &leads[0]);
                if r.contains("You can't move") {
                    continue;
                }
            }
            if status != 99 {
                run_command(computer, &command);
            }
        }
    }
    for next in leads {
        if visited.insert((cur.clone(), next.clone())) {
            let (status, r) = run_command(computer, &next);
            assert_ne!(status, 99);
            if let Some(s) = dfs(computer, r, visited, found_items) {
                return Some(s);
            }
            run_command(computer, rev_dir(&next));
        }
    }
    None
}

fn rev_dir(dir: &str) -> &str {
    match dir {
        "north" => "south",
        "south" => "north",
        "east" => "west",
        "west" => "east",
        _ => unreachable!("dir: {dir}"),
    }
}

fn parse_inv(output: &str) -> Vec<&str> {
    let mut r = vec![];
    for line in output.lines() {
        if let Some(item) = line.trim().strip_prefix('-') {
            r.push(item.trim());
        }
    }
    r
}

fn cur_location(output: &str) -> String {
    for line in output.lines() {
        if let Some(line) = line.trim().strip_prefix("==") {
            if let Some(cur) = line.trim().strip_suffix("==") {
                return cur.trim().to_string();
            }
        }
    }
    unreachable!("{output}")
}

fn leads(output: &str) -> Vec<String> {
    let mut r = vec![];
    let mut flag = false;
    for line in output.lines() {
        if line.trim() == "Doors here lead:" {
            flag = true;
        } else if flag {
            if let Some(m) = line.trim().strip_prefix('-') {
                r.push(m.trim().to_string())
            } else {
                return r;
            }
        }
    }

    r
}

fn items(output: &str) -> Vec<String> {
    let mut r = vec![];
    let mut flag = false;
    for line in output.lines() {
        if line.trim() == "Items here:" {
            flag = true;
        } else if flag {
            if let Some(m) = line.trim().strip_prefix('-') {
                r.push(m.trim().to_string())
            } else {
                return r;
            }
        }
    }

    r
}

fn run_command(computer: &mut Computer, command: &str) -> (Int, String) {
    computer.input = command_to_input(command);
    let status = computer.run();
    let mut output = vec![];
    swap(&mut output, &mut computer.output);
    (status, output_to_string(&output))
}

fn command_to_input(command: &str) -> Vec<Int> {
    command
        .bytes()
        .map(|b| b as Int)
        .chain([b'\n' as Int])
        .rev()
        .collect()
}

fn output_to_string(output: &[Int]) -> String {
    let mut s = String::new();
    for &i in output {
        s.push(i as u8 as char);
    }
    s
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
