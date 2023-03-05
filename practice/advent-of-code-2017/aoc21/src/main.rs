use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
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

fn mutate_size_two_pattern(input: &str) -> Vec<String> {
    let origin: Vec<_> = input
        .trim()
        .chars()
        .filter(|&l| l == '.' || l == '#')
        .collect();
    assert_eq!(origin.len(), 4);
    let r = [
        [origin[0], origin[1], origin[2], origin[3]],
        [origin[2], origin[0], origin[3], origin[1]],
        [origin[3], origin[2], origin[1], origin[0]],
        [origin[1], origin[3], origin[0], origin[2]],
        [origin[1], origin[0], origin[3], origin[2]],
        [origin[3], origin[1], origin[2], origin[0]],
        [origin[2], origin[3], origin[0], origin[1]],
        [origin[0], origin[2], origin[1], origin[3]],
    ];
    r.into_iter().map(|v| v.into_iter().collect()).collect()
}

fn mutate_size_three_pattern(input: &str) -> Vec<String> {
    let o: Vec<_> = input
        .trim()
        .chars()
        .filter(|&l| l == '.' || l == '#')
        .collect();
    assert_eq!(o.len(), 9);
    let r = [
        [o[0], o[1], o[2], o[3], o[4], o[5], o[6], o[7], o[8]],
        [o[6], o[3], o[0], o[7], o[4], o[1], o[8], o[5], o[2]],
        [o[8], o[7], o[6], o[5], o[4], o[3], o[2], o[1], o[1]],
        [o[2], o[5], o[8], o[1], o[4], o[7], o[0], o[3], o[6]],
        [o[2], o[1], o[0], o[5], o[4], o[4], o[8], o[7], o[6]],
        [o[8], o[5], o[2], o[7], o[4], o[1], o[6], o[3], o[0]],
        [o[6], o[7], o[8], o[3], o[4], o[5], o[0], o[1], o[2]],
        [o[0], o[3], o[6], o[1], o[4], o[7], o[2], o[5], o[8]],
    ];
    r.into_iter().map(|v| v.into_iter().collect()).collect()
}

fn parse_input(input: &str) -> HashMap<(usize, u128), u128> {
    let mut rules = HashMap::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if let Some((left, right)) = line.trim().split_once(" => ") {
            let (size, patterns) = if left.trim().len() == 3 * 3 + 2 {
                (3, mutate_size_three_pattern(left))
            } else {
                (2, mutate_size_two_pattern(left))
            };
            let right = parse_pattern(right);
            for left in patterns {
                rules.insert((size, parse_pattern(&left)), right);
            }
        }
    }
    rules
}

#[derive(Clone, Hash, PartialEq, Eq)]
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
        let mask = if chunk_size == 2 { 0b11 } else { 0b111 };
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
        let mask = if chunk_size == 3 { 0b111 } else { 0b1111 };
        let chunk_count = self.size / (chunk_size - 1);
        self.raw = vec![0; self.raw.len() + chunk_count];
        for (x, rows) in chunks.chunks(chunk_count).enumerate() {
            for chunk in rows.iter() {
                for i in 0..chunk_size {
                    let bits = (chunk >> (chunk_size * (chunk_size - 1 - i))) & mask;
                    self.raw[x * chunk_size + i] <<= chunk_size;
                    self.raw[x * chunk_size + i] |= bits;
                }
            }
        }

        self.size += chunk_count;
    }

    fn from_raw(raw: Vec<u128>, size: usize) -> Self {
        Self { raw, size }
    }

    fn split_to_images(&self, chunk_size: usize) -> Vec<Image> {
        if chunk_size != 3 {
            unimplemented!("unable to split image to other size than 3x3");
        }
        let mut images = vec![];
        let mask = if chunk_size == 2 { 0b11 } else { 0b111 };
        let chunk_count = self.size / chunk_size;
        for rows in self.raw.chunks(chunk_size) {
            for chunk_index in 0..chunk_count {
                let mut raw = vec![];
                for row in rows {
                    raw.push((row >> (chunk_size * chunk_index)) & mask);
                }
                images.push(Image::from_raw(raw, chunk_size));
            }
        }
        images
    }

    fn pixel_count(&self) -> u32 {
        self.raw.iter().map(|r| r.count_ones()).sum()
    }
}

fn search_rule(pattern: u128, chunk_size: usize, rules: &HashMap<(usize, u128), u128>) -> u128 {
    if let Some(r) = rules.get(&(chunk_size, pattern)) {
        return *r;
    }

    unreachable!("unrecognizable pattern {pattern:0b}")
}

fn enhance_result(rules: &HashMap<(usize, u128), u128>, count: usize) -> u32 {
    let mut image = Image::new();
    for _ in 0..count {
        image.enhance(rules);
    }

    image.pixel_count()
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let rules = parse_input(&input);

    part1(&rules)?;
    part2(&rules)?;
    Ok(())
}

fn part1(rules: &HashMap<(usize, u128), u128>) -> Result<u32> {
    let start = Instant::now();

    let result = enhance_result(rules, 5);

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(rules: &HashMap<(usize, u128), u128>) -> Result<u32> {
    let start = Instant::now();

    let mut images = vec![Image::new()];
    let mut cache: HashMap<Image, Image> = HashMap::new();
    for _ in 0..3 {
        let mut temp = vec![];
        for image in images.iter_mut() {
            if let Some(image) = cache.get(image) {
                temp.extend(image.split_to_images(3));
            } else {
                let origin = image.clone();
                for _ in 0..6 {
                    image.enhance(rules)
                }
                temp.extend(image.split_to_images(3));
                cache.insert(origin, image.clone());
            }
        }
        images = temp;
    }

    let result = images.iter().map(|i| i.pixel_count()).sum();

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

#[test]
fn example_input() {
    let input = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";
    assert_eq!(enhance_result(&parse_input(input), 2), 12);
}
