use std::io::{self, Read, Write};
use std::error::Error;
use std::result;
use std::str::FromStr;
use std::time::Instant;

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
    for _ in 1..=generations {
        tunel.next();
    }
    let mut sum = 0;
    let n = tunel.pots.len();
    for i in 0..n {
        sum += tunel.pots[i] as i32 * (i as i32 - tunel.zero as i32);
    }
    writeln!(io::stdout(), "{} generations answer: {}", generations, sum)?;
    Ok(())
}

#[derive(Debug, Clone)]
struct Tunel {
    pots: Vec<u8>,
    zero: usize,
    notes: Vec<Note>,
}

impl Tunel {
    fn new(init: Vec<u8>, notes: Vec<Note>) -> Self {
        let mut pots = [0].repeat(3).iter().chain(init.iter()).cloned().collect::<Vec<u8>>();
        pots.extend([0].repeat(3));
        Tunel {
            pots,
            zero: 3,
            notes,
        }
    }

    fn next(&mut self) {
        let mut pots = vec![];
        
        let n = self.pots.len();
        let mut slice = vec![0, 0, self.pots[0], self.pots[1]];
        let mut l = 4;
        for i in 0..n {
            if i + 2 < n {
                slice.push(self.pots[i+2]);
            } else {
                slice.push(0);
            }
            l += 1;
            let s = &slice[l-5..l];
            if let Some(index) = self.notes.iter().position(|n| n.current_state == s) {
                pots.push(self.notes[index].next_state);
            } else {
                pots.push(0);
            }
        }
        pots.push(0);
        pots.push(0);
        pots.push(0);
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