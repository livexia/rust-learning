#[macro_use]
extern crate lazy_static;

use std::hash::Hash;
use std::io::{self, Read, Write};
use std::error::Error;
use std::result;
use std::str::FromStr;
use std::cmp::Eq;
use std::collections::HashMap;
use std::collections::HashSet;

use regex::Regex;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn Error>>;

type Step = char;

type Required = HashMap<Step, HashSet<Step>>;

fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let instructions = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Instruction>>>()?;
    if instructions.is_empty() {
        return err!("no requirements given");
    }
    
    let mut requirements: Required = HashMap::new();

    for deps in &instructions {
        requirements.entry(deps.step).or_default().insert(deps.required);
        requirements.entry(deps.required).or_default();
    }

    part1(&requirements)?;
    part2(&requirements)?;
    
    Ok(())
}

fn part1(required: &Required) -> Result<()> {
    let mut taken: HashSet<Step> = HashSet::new();
    let mut order: Vec<Step> = vec![];
    let mut next: Vec<Step> = vec![];
    loop {
        find_next_steps(&required, &taken, &taken, &mut next);
        let next_step = match next.pop() {
            None => break,
            Some(next_step) => next_step,
        };
        taken.insert(next_step);
        order.push(next_step);
    }

    let answer: String = order.iter().clone().collect();
    writeln!(io::stdout(), "step order: {}", answer)?;
    Ok(())
}

fn part2(required: &Required) -> Result<()> {
    let mut workers = Workers::new(5);
    let mut assigned: HashSet<Step> = HashSet::new();
    let mut done: HashSet<Step> = HashSet::new();
    let mut order: Vec<Step> = vec![];
    let mut next: Vec<Step> = vec![];

    let mut seconds = 0;
    loop {
        workers.run_one_step(&mut order, &mut done);

        find_next_steps(&required, &assigned, &done, &mut next);
        if next.is_empty() && workers.all_idle() {
            break;
        }
        for worker in workers.available() {
            let next_step = match next.pop() {
                None => break,
                Some(next_step) => next_step,
            };
            assigned.insert(next_step);
            workers.work_on(worker, next_step);
        }
        seconds += 1;
    }

    let answer: String = order.iter().clone().collect();
    writeln!(io::stdout(), "step order part2: {}", answer)?;
    writeln!(io::stdout(), "total seconds: {}", seconds)?;
    Ok(())
}

fn find_next_steps(
    required: &Required,
    taken: &HashSet<Step>,
    done: &HashSet<Step>,
    next_stack: &mut Vec<Step>
) {
    for (&step, deps) in required {
        if taken.contains(&step) {
            continue;
        }
        if deps.iter().all(|s| done.contains(s)) {
            next_stack.push(step);
        }
    }
    next_stack.sort();
    next_stack.dedup();
    next_stack.reverse();
}

type WorkerID = usize;

struct Workers {
    status: Vec<Status>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    Idle,
    Working { step: Step, remaining: u32 }
}

impl Workers {
    fn new(count: usize) -> Self {
        Workers {
            status: vec![Status::Idle; count]
        }
    }

    fn available(&self) -> Vec<WorkerID> {
        let mut available = vec![];
        for (worker, &status) in self.status.iter().enumerate() {
            if status == Status::Idle {
                available.push(worker);
            }
        }
        available
    }

    fn all_idle(&self) -> bool {
        self.status.iter().all(|s| *s == Status::Idle)
    }

    fn work_on(&mut self, worker: WorkerID, step: Step) {
        let status = &mut self.status[worker];
        assert!(*status == Status::Idle, "worker {} is not avaliable", worker);

        let remaining = step as u32 - b'A' as u32 + 1 + 60;
        *status = Status::Working { step, remaining }
    }

    fn run_one_step(&mut self, order: &mut Vec<Step>, done: &mut HashSet<Step>) {
        for worker in 0..self.status.len() {
            let mut is_done = false;
            match self.status[worker] {
                Status::Idle => {},
                Status::Working { step, ref mut remaining} => {
                    *remaining -= 1;
                    if *remaining == 0 {
                        is_done = true;
                        order.push(step);
                        done.insert(step);
                    }
                }
            }
            if is_done {
                self.status[worker] = Status::Idle;
            }
        }
    }
}

#[derive(Debug)]
struct Instruction {
    step: Step,
    required: Step,
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x)
                Step.*(?P<requirement>[A-Z]).*(?P<step>[A-Z]).*
            ").unwrap();
        }
        let caps = match RE.captures(s) {
            None => return err!("unrecongnized instruction"),
            Some(caps) => caps,
        };
        Ok(Instruction {
            step: caps["step"].parse()?,
            required: caps["requirement"].parse()?
        })
    }
}