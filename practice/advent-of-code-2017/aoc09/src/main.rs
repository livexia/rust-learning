use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let stream = parse_input(&input).unwrap();

    part1(&stream)?;
    // part2()?;
    Ok(())
}

fn part1(stream: &Stream) -> Result<usize> {
    let start = Instant::now();

    println!("{:?}", stream);
    let result = 1 + stream.get_score(1);

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

#[derive(Debug)]
enum Stream {
    Group(Vec<Stream>),
    Garbage(Vec<char>),
}

impl Stream {
    fn new_group() -> Self {
        Stream::Group(vec![])
    }

    fn add_stream(&mut self, inner: Stream) {
        if let Stream::Group(s) = self {
            s.push(inner);
        }
    }

    fn get_score(&self, outer_score: usize) -> usize {
        match self {
            Stream::Group(v) => v
                .iter()
                .map(|k| k.get_score(outer_score + 1) + outer_score)
                .sum::<usize>(),
            Stream::Garbage(_) => 0,
        }
    }

    fn from_vec(input: &mut Vec<char>) -> Option<Stream> {
        let mut stack = vec![];
        let mut count = 0;
        while let Some(c) = input.pop() {
            match c {
                '}' => {
                    count += 1;
                    stack.push(Stream::new_group());
                }
                '{' => {
                    count -= 1;
                    if count != 0 {
                        if let Some(inner) = stack.pop() {
                            if let Some(outer) = stack.last_mut() {
                                outer.add_stream(inner);
                            }
                        }
                    }
                }
                '>' => {
                    if let Some(r) = stack.last_mut() {
                        r.add_stream(Stream::from_vec_to_grabage(input))
                    }
                }
                _ => (),
            };
        }
        assert_eq!(count, 0);
        assert_eq!(stack.len(), 1);
        stack.pop()
    }

    fn from_vec_to_grabage(input: &mut Vec<char>) -> Stream {
        let mut index = 0;
        for (i, &c) in input.iter().enumerate() {
            if c == '<' {
                index = i;
                break;
            }
        }
        Stream::Garbage(input.drain(index..).collect())
    }
}

fn parse_input(input: &str) -> Option<Stream> {
    let mut input = input.chars().collect();
    Stream::from_vec(&mut input)
}

#[test]
fn example_input() {
    let input = "{{<a!>},{<a!>},{<a!>},{<ab>}}";
    assert_eq!(part1(&parse_input(input).unwrap()).unwrap(), 3);
}
