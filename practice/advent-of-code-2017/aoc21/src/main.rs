use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

// ##../#.../..../..#. => 1100100000000010
fn parse_pattern(input: &str) -> u128 {
    let mut bit_map = 0;
    for c in input.trim().chars() {
        if c == '/' {
            continue;
        }
        bit_map <<= 1;
        bit_map |= if c == '#' { 1 } else { 0 };
    }
    bit_map
}

fn parse_input(input: &str) -> HashMap<(usize, u128), u128> {
    let mut rules = HashMap::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if let Some((left, right)) = line.trim().split_once(" => ") {
            let size = if left.trim().len() == 3 * 3 + 2 { 3 } else { 4 };
            let left = parse_pattern(left);
            let right = parse_pattern(right);
            rules.insert((size, left), right);
        }
    }
    rules
}

struct Image {
    raw: Vec<u128>,
    size: usize,
}

impl Image {
    fn new() -> Self {
        Self {
            raw: vec![
                parse_pattern(".#."),
                parse_pattern("..#"),
                parse_pattern("###"),
            ],
            size: 3,
        }
    }

    fn enhance(&mut self, rules: &HashMap<(usize, u128), u128>) {
        let chunk_size = if self.size % 2 == 0 {
            2
        } else if self.size % 3 == 0 {
            3
        } else {
            unreachable!("image size {} does not divisible by 2 or 3", self.size);
        };
        // split image with chunk_size
        let chunks = self.split(chunk_size);
        // match pattern with rules
        let new_chunks = chunks
            .iter()
            .map(|&chunk| search_rule(chunk, chunk_size, rules))
            .collect();
        // merge new chunks with chunk_size + 1
        self.merge(chunk_size + 1, new_chunks);
    }

    fn split(&self, chunk_size: usize) -> Vec<u128> {
        let mut new_chunks = vec![];
        let mask = if chunk_size == 2 { 0xff } else { 0xfff };
        let chunk_count = self.size / chunk_size;
        for rows in self.raw.chunks(chunk_size) {
            let mut new_rows = vec![];
            for chunk_index in 0..chunk_count {
                let mut chunk = 0;
                for row in rows {
                    chunk <<= chunk_size;
                    chunk |= (row >> (chunk_size * chunk_index)) & mask;
                }
                new_rows.push(chunk);
            }
            new_chunks.extend(new_rows.iter().rev());
        }
        new_chunks
    }

    fn merge(&mut self, chunk_size: usize, chunks: Vec<u128>) {
        dbg!(&chunks);
        println!("{:0b}", chunks[0]);
        todo!()
    }
}

fn search_rule(pattern: u128, chunk_size: usize, rules: &HashMap<(usize, u128), u128>) -> u128 {
    for pattern in mutate_pattern(pattern, chunk_size) {
        if let Some(r) = rules.get(&(chunk_size, pattern)) {
            return *r;
        }
    }

    unreachable!("unrecognizable pattern {pattern:0b}")
}

fn mutate_pattern(pattern: u128, chunk_size: usize) -> Vec<u128> {
    todo!()
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let rules = parse_input(&input);

    part1(&rules)?;
    // part2()?;
    Ok(())
}

fn part1(rules: &HashMap<(usize, u128), u128>) -> Result<()> {
    let start = Instant::now();

    let mut image = Image::new();
    image.enhance(rules);

    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    todo!()
}
