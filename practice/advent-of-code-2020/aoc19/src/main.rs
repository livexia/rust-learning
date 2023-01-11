use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let (rules, messages) = parse_input(&input)?;

    part1(&rules, &messages)?;
    // part2()?;
    Ok(())
}
fn part1(rules: &[Rule], messages: &[Vec<char>]) -> Result<usize> {
    let start = Instant::now();

    let r = &rules[0];
    let result = messages.iter().filter(|m| r.match_msg(m, rules).0).count();

    writeln!(io::stdout(), "Part 1 {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

#[derive(Debug)]
enum Rule {
    Sub(Vec<Vec<usize>>),
    Single(char),
}

impl Rule {
    fn match_msg(&self, msg: &[char], rules: &[Rule]) -> (bool, usize) {
        match self {
            Rule::Sub(subs) => {
                for sub in subs {
                    let mut start = 0;
                    for &r in sub {
                        let (matched, checked_length) = rules[r].match_msg(&msg[start..], rules);
                        if matched {
                            start += checked_length;
                        } else {
                            break;
                        }
                    }
                    if start == msg.len() {
                        return (start == msg.len(), start);
                    }
                }
                (false, 0)
            }
            Rule::Single(c) => (&msg[0] == c, 1),
        }
    }
}

fn parse_input(input: &str) -> Result<(Vec<Rule>, Vec<Vec<char>>)> {
    let mut rules = vec![];
    let mut messages = vec![];

    let mut part = 1;

    for line in input.lines() {
        if line.trim().is_empty() {
            part += 1;
            continue;
        }
        if part == 1 {
            if let Some((_, content)) = line.trim().split_once(": ") {
                let content: Vec<&str> = content.split(' ').collect();
                if content.len() == 1 && content[0].starts_with('"') {
                    rules.push(Rule::Single(content[0].chars().skip(1).next().unwrap()))
                } else {
                    let mut sub = vec![];
                    let mut subs = vec![];
                    for p in content {
                        if p.trim() == "|" {
                            subs.push(sub);
                            sub = vec![];
                        } else {
                            sub.push(p.trim().parse()?)
                        }
                    }
                    if !sub.is_empty() {
                        subs.push(sub)
                    }
                    rules.push(Rule::Sub(subs));
                }
            }
        } else {
            messages.push(line.trim().chars().collect())
        }
    }
    Ok((rules, messages))
}

#[test]
fn example_input() {
    let input = "0: 1 2
    1: \"a\"
    2: 1 3 | 3 1
    3: \"b\"
    
    aab
    aba
    abb
    ";
    let (rules, messages) = parse_input(&input).unwrap();
    assert_eq!(part1(&rules, &messages).unwrap(), 2);

    let input = "0: 4 1 5
    1: 2 3 | 3 2
    2: 4 4 | 5 5
    3: 4 5 | 5 4
    4: \"a\"
    5: \"b\"
    
    ababbb
    bababa
    abbbab
    aaabbb
    aaaabbb";
    let (rules, messages) = parse_input(&input).unwrap();
    assert_eq!(part1(&rules, &messages).unwrap(), 2);
}
