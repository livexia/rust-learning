use std::{env::set_current_dir, io::{self, Read, Write}};
use std::error::Error;
use std::result;
use std::str::FromStr;
use std::time::Instant;
use std::collections::HashMap;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    let input: Vec<&str> = input.split("\n").filter(|&x| x.len() != 0).collect();
    let init: Vec<u8> = input[0].split(":").collect::<Vec<&str>>()[1]
        .trim().chars()
        .map(|s| if s == '.' { 0 } else { 1 }).collect();
    
    let notes = input[1..]
        .iter()
        .map(|&s| s.parse())
        .collect::<Result<Vec<Note>>>()?;

    let tunel = Tunel::new(init, notes);

    let start = Instant::now();
    run(tunel.clone(), 20)?;
    println!("Time elapsed in run(20) is: {:?}", start.elapsed());

    let start = Instant::now();
    run(tunel.clone(), 50)?;
    println!("Time elapsed in run(50) is: {:?}", start.elapsed());

    let start = Instant::now();
    run(tunel.clone(), 500)?;
    println!("Time elapsed in run(500) is: {:?}", start.elapsed());

    let start = Instant::now();
    run(tunel.clone(), 5000)?;
    println!("Time elapsed in run(5000) is: {:?}", start.elapsed());

    let start = Instant::now();
    run(tunel.clone(), 50000)?;
    println!("Time elapsed in run(50000) is: {:?}", start.elapsed());


    Ok(())
}

fn run(mut tunel: Tunel, generations: u32) -> Result<()>{
    for i in 1..=generations {
        tunel.next();
        if i % 10000 == 0 {
            println!("generation: {}, planted pots: {}", i, tunel.pots.len());
        }
    }
    let sum: i32 = tunel.pots
        .iter()
        .map(|(i, _)| i)
        .sum();

    writeln!(io::stdout(), "{} generations answer: {}", generations, sum)?;
    Ok(())
}

#[derive(Debug, Clone)]
struct Tunel {
    pots: HashMap<i32, u8>,
    zero: usize,
    notes: HashMap<Vec<u8>, u8>,
}

impl Tunel {
    fn new(init: Vec<u8>, notes: Vec<Note>) -> Self {
        let pots: HashMap<i32, u8> = init
            .iter()
            .enumerate()
            .filter(|(_, &x)| x != 0)
            .map(|(i, &x)| (i as i32, x))
            .collect();
        Tunel {
            pots,
            zero: 3,
            notes: notes.iter()
                .filter(|n| n.next_state != 0)
                .map(|n| (n.current_state.clone(), n.next_state)).collect(),
        }
    }

    fn next(&mut self) {
        let mut pots: HashMap<i32, u8> = HashMap::new();
        for i in self.pots.keys() {
            for j in i-2..=i+2 {
                let mut slice = vec![];
                for k in j-2..=j+2 {
                    if let Some(&x) = self.pots.get(&k) {
                        slice.push(x);
                    } else {
                        slice.push(0);
                    }
                }
                if let Some(&next_state) = self.notes.get(&slice) {
                    pots.insert(j, next_state);
                }
            }
        }
        self.pots = pots;
    }
}

#[derive(Debug, Clone)]
struct Note {
    current_state: Vec<u8>,
    next_state: u8,
}

impl FromStr for Note {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let s: Vec<&str> = s.split("=>").collect();
        let current: Vec<u8> = s[0].trim().chars().map(|s| if s == '.' { 0 } else { 1 }).collect();
        if current.len() != 5 {
            return err!("unrecongnized note");
        }
        let next: u8 = match s[1].trim().chars().next().unwrap() {
            '.' => 0,
            '#' => 1,
            _ => return err!("unrecongnized note"),
        };
        Ok(Note {
            current_state: current,
            next_state: next,
        })
    }
}