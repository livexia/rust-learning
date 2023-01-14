use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Int = usize;

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

    let mut program = program.to_owned();
    run_program_with_noun_verb(&mut program, 12, 2);
    let result = program[0];

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(program: &[Int]) -> Result<Int> {
    let start = Instant::now();

    let (noun, verb) = search_noun_verb(program, 19690720)?;
    let result = 100 * noun + verb;

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn search_noun_verb(program: &[Int], dest: Int) -> Result<(Int, Int)> {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut test = program.to_owned();
            run_program_with_noun_verb(&mut test, noun, verb);
            if test[0] == dest {
                return Ok((noun, verb));
            }
        }
    }
    err!("can not find valid noun and verb for input")
}

fn run_program_with_noun_verb(program: &mut [Int], noun: Int, verb: Int) {
    program[1] = noun;
    program[2] = verb;
    run_program(program);
}

fn run_program(program: &mut [Int]) {
    let mut pc = 0;
    while pc < program.len() {
        let opcode = program[pc];
        let op1 = program[pc + 1];
        let op2 = program[pc + 2];
        let dest = program[pc + 3];
        program[dest] = match opcode {
            1 => program[op1] + program[op2],
            2 => program[op1] * program[op2],
            99 => return,
            _ => unreachable!(
                "Encountering an unknown opcode means something went wrong: {}",
                opcode
            ),
        };
        pc += 4;
    }
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
    let input = "1,9,10,3,2,3,11,0,99,30,40,50";
    let program = parse_input(input).unwrap();

    let mut test = program.clone();
    run_program(&mut test);
    assert_eq!(test[0], 3500);
}
