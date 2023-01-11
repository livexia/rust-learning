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
    let (rules, messages) = parse_input(&input)?;

    part1(&rules, &messages)?;
    part2(&rules, &messages)?;
    Ok(())
}

fn part1(rules: &[Option<Rule>], messages: &[&str]) -> Result<usize> {
    let start = Instant::now();

    let r = rules[0].as_ref().unwrap();
    let result = messages
        .iter()
        .filter(|m| {
            let (matched, length) = r.match_msg(m, rules);
            matched && length == m.len()
        })
        .count();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(rules: &[Option<Rule>], messages: &[&str]) -> Result<usize> {
    let start = Instant::now();

    let rules = replace_rules(rules);
    let r = rules[0].as_ref().unwrap();
    let result = messages
        .iter()
        .filter(|m| {
            let (matched, length) = r.match_msg(m, &rules);
            if matched && length == m.len() {
                println!("{}", m)
            }
            matched && length == m.len()
        })
        .count();

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn replace_rules(rules: &[Option<Rule>]) -> Vec<Option<Rule>> {
    let mut rules = rules.to_vec();
    rules[8] = Some(Rule::Sub(vec![vec![42], vec![42, 8]]));
    rules[11] = Some(Rule::Sub(vec![vec![42, 31], vec![42, 11, 31]]));
    rules
}

#[derive(Debug, Clone)]
enum Rule {
    Sub(Vec<Vec<usize>>),
    Single(char),
}

impl Rule {
    fn match_msg(&self, msg: &str, rules: &[Option<Rule>]) -> (bool, usize) {
        match self {
            Rule::Sub(subs) => {
                'subs: for sub in subs {
                    let mut start = 0;
                    for (i, &r) in sub.iter().enumerate() {
                        if let Some(r) = &rules[r] {
                            if start >= msg.len() {
                                println!("2333");
                                return (true, start);
                            }
                            let (matched, checked_length) = r.match_msg(&msg[start..], rules);

                            if matched {
                                start += checked_length;
                            } else {
                                continue 'subs;
                            }
                        } else {
                            unreachable!()
                        }
                    }
                    return (true, start);
                }
                (false, 0)
            }
            Rule::Single(c) => (msg.starts_with(*c), 1),
        }
    }
}

fn parse_input(input: &str) -> Result<(Vec<Option<Rule>>, Vec<&str>)> {
    let mut rules = vec![];
    let mut messages = vec![];

    let mut part = 1;

    for line in input.lines() {
        if line.trim().is_empty() {
            part += 1;
            continue;
        }
        if part == 1 {
            if let Some((id, content)) = line.trim().split_once(": ") {
                let id: usize = id.trim().parse()?;
                if id >= rules.len() {
                    for _ in rules.len()..id + 1 {
                        rules.push(None);
                    }
                }
                let content: Vec<&str> = content.split(' ').collect();
                if content.len() == 1 && content[0].starts_with('"') {
                    rules[id] = Some(Rule::Single(content[0].chars().nth(1).unwrap()))
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
                    rules[id] = Some(Rule::Sub(subs));
                }
            }
        } else {
            messages.push(line.trim())
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

    let input = "42: 9 14 | 10 1
    9: 14 27 | 1 26
    10: 23 14 | 28 1
    1: \"a\"
    11: 42 31
    5: 1 14 | 15 1
    19: 14 1 | 14 14
    12: 24 14 | 19 1
    16: 15 1 | 14 14
    31: 14 17 | 1 13
    6: 14 14 | 1 14
    2: 1 24 | 14 4
    0: 8 11
    13: 14 3 | 1 12
    15: 1 | 14
    17: 14 2 | 1 7
    23: 25 1 | 22 14
    28: 16 1
    4: 1 1
    20: 14 14 | 1 15
    3: 5 14 | 16 1
    27: 1 6 | 14 18
    14: \"b\"
    21: 14 1 | 1 14
    25: 1 1 | 1 14
    22: 14 14
    8: 42
    26: 14 22 | 1 20
    18: 15 15
    7: 14 5 | 1 21
    24: 14 1
    
    abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
    bbabbbbaabaabba
    babbbbaabbbbbabbbbbbaabaaabaaa
    aaabbbbbbaaaabaababaabababbabaaabbababababaaa
    bbbbbbbaaaabbbbaaabbabaaa
    bbbababbbbaaaaaaaabbababaaababaabab
    ababaaaaaabaaab
    ababaaaaabbbaba
    baabbaaaabbaaaababbaababb
    abbbbabbbbaaaababbbbbbaaaababb
    aaaaabbaabaaaaababaa
    aaaabbaaaabbaaa
    aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
    babaaabbbaaabaababbaabababaaab
    aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
    let (rules, messages) = parse_input(&input).unwrap();
    assert_eq!(part1(&rules, &messages).unwrap(), 3);

    let rules = replace_rules(&rules);
    let r = rules[0].as_ref().unwrap();
    let result = r.match_msg("aaaabbaaaabbaaa", &rules);
    println!("{:?}", result);

    // assert_eq!(part2(&rules, &messages).unwrap(), 12);
}
