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
    part2(&stream)?;
    Ok(())
}

fn part1(stream: &Stream) -> Result<usize> {
    let start = Instant::now();

    let result = stream.get_score(1);

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(stream: &Stream) -> Result<usize> {
    let start = Instant::now();

    let result = stream.get_char_count();

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

#[derive(Debug)]
enum Stream {
    Group(Vec<Stream>),
    Garbage(usize),
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

    fn get_score(&self, level: usize) -> usize {
        match self {
            Stream::Group(v) => level + v.iter().map(|k| k.get_score(level + 1)).sum::<usize>(),
            Stream::Garbage(_) => 0,
        }
    }

    fn get_char_count(&self) -> usize {
        match self {
            Stream::Group(v) => v.iter().map(|k| k.get_char_count()).sum(),
            Stream::Garbage(c) => *c,
        }
    }

    fn from_vec(input: &mut Vec<char>) -> Option<Stream> {
        let mut stack = vec![];
        let mut count = 0;
        while let Some(c) = input.pop() {
            match c {
                '{' => {
                    count += 1;
                    stack.push(Stream::new_group());
                }
                '}' => {
                    count -= 1;
                    if count != 0 {
                        if let Some(inner) = stack.pop() {
                            if let Some(outer) = stack.last_mut() {
                                outer.add_stream(inner);
                            }
                        }
                    }
                }
                '<' => {
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
        let mut count = 0;
        let mut flag = false;
        while let Some(c) = input.pop() {
            if flag {
                flag = false;
                continue;
            }
            match c {
                '!' => flag = true,
                '>' => return Stream::Garbage(count),
                _ => count += 1,
            }
        }
        Stream::Garbage(count)
    }
}

fn parse_input(input: &str) -> Option<Stream> {
    let mut input = input.chars().rev().collect();
    Stream::from_vec(&mut input)
}

#[test]
fn example_input() {
    let input = "{{<a!>},{<a!>},{<a!>},{<ab>}}";
    assert_eq!(part1(&parse_input(input).unwrap()).unwrap(), 3);
    assert_eq!(
        part1(&parse_input("{{<!!>},{<!!>},{<!!>},{<!!>}}").unwrap()).unwrap(),
        9
    );
    assert_eq!(
        part1(&parse_input("{{<ab>},{<ab>},{<ab>},{<ab>}}").unwrap()).unwrap(),
        9
    );
    assert_eq!(
        part1(&parse_input("{<a>,<a>,<a>,<a>}").unwrap()).unwrap(),
        1
    );
    assert_eq!(part1(&parse_input("{{{},{},{{}}}}").unwrap()).unwrap(), 16);
    assert_eq!(part1(&parse_input("{{},{}}").unwrap()).unwrap(), 5);
    assert_eq!(part1(&parse_input("{{{}}}").unwrap()).unwrap(), 6);
    assert_eq!(part1(&parse_input("{}").unwrap()).unwrap(), 1);
    assert_eq!(
        part2(&parse_input("{<{o\"i!a,<{i<a>}").unwrap()).unwrap(),
        10
    );
}
