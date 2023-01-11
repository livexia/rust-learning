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

    let expres = input
        .lines()
        .map(|l| parse_input(&l.trim().chars().filter(|&c| c != ' ').collect::<Vec<_>>()))
        .collect::<Result<Vec<_>>>()?;

    part1(&expres)?;
    part2(&expres)?;
    Ok(())
}

fn part1(expres: &[Experssion]) -> Result<usize> {
    let start = Instant::now();

    let result: usize = expres.iter().map(|e| e.eval().unwrap()).sum();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(expres: &[Experssion]) -> Result<usize> {
    let start = Instant::now();

    let result: usize = expres.iter().map(|e| e.advance_eval().unwrap()).sum();

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

#[derive(Debug, Clone)]
enum Experssion {
    Int(usize),
    Op(char),
    Formula(Vec<Experssion>),
}

fn parse_input(s: &[char]) -> Result<Experssion> {
    let mut i = 0;
    let mut f = vec![];
    while i < s.len() {
        if s[i].is_numeric() {
            f.push(Experssion::Int((s[i] as u8 - b'0') as usize));
            i += 1;
        } else if s[i] == '*' || s[i] == '+' {
            f.push(Experssion::Op(s[i]));
            i += 1;
        } else if s[i] == '(' {
            let mut right = i + 1;
            let mut left_count = 1;
            let mut right_count = 0;
            while left_count != right_count {
                if s[right] == '(' {
                    left_count += 1
                } else if s[right] == ')' {
                    right_count += 1
                }
                right += 1;
            }
            f.push(parse_input(&s[i + 1..right - 1])?);
            i = right;
        } else {
            unreachable!()
        }
    }
    f.reverse();
    if f.len() == 1 {
        Ok(f.pop().unwrap())
    } else {
        Ok(Experssion::Formula(f))
    }
}

impl Experssion {
    fn pretty(&self) -> String {
        let mut s = String::new();
        match self {
            Experssion::Int(n) => s.push_str(&format!("{}", n)),
            Experssion::Op(c) => s.push(*c),
            Experssion::Formula(c) => {
                for e in c.iter().rev() {
                    if e.is_formula() {
                        s.push('(');
                        s.push_str(&e.pretty());
                        s.push(')');
                    } else {
                        s.push_str(&e.pretty());
                    }
                }
            }
        }
        s
    }

    fn is_formula(&self) -> bool {
        if let Experssion::Formula(_) = self {
            return true;
        }
        false
    }

    fn eval(&self) -> Result<usize> {
        match self {
            Experssion::Int(n) => Ok(*n),
            Experssion::Op(_) => unreachable!(),
            Experssion::Formula(v) => {
                let mut v = v.clone();
                if v.len() < 3 {
                    unreachable!();
                }
                for e in &mut v {
                    if e.is_formula() {
                        *e = Experssion::Int(e.eval()?)
                    }
                }
                while v.len() > 1 {
                    let r = calc_three(&mut v)?;
                    v.push(r);
                }
                if let Some(Experssion::Int(n)) = v.pop() {
                    return Ok(n);
                }
                err!("can not eval expression: {}", self.pretty())
            }
        }
    }

    fn advance_eval(&self) -> Result<usize> {
        match self {
            Experssion::Int(n) => Ok(*n),
            Experssion::Op(_) => unreachable!(),
            Experssion::Formula(v) => {
                let mut v = v.clone();
                if v.len() < 3 {
                    unreachable!();
                }
                if v.len() == 3 {}
                for e in &mut v {
                    if e.is_formula() {
                        *e = Experssion::Int(e.advance_eval()?)
                    }
                }
                let mut i = 0;
                while i < v.len() {
                    if let Experssion::Op(op) = v[i] {
                        if op == '+' {
                            if let Experssion::Int(op1) = v[i - 1] {
                                if let Experssion::Int(op2) = v[i + 1] {
                                    let r = op1 + op2;
                                    for _ in 0..3 {
                                        v.remove(i - 1);
                                    }
                                    v.insert(i - 1, Experssion::Int(r));
                                } else {
                                    unreachable!()
                                }
                            } else {
                                unreachable!()
                            }
                        } else {
                            i += 1;
                        }
                    } else {
                        i += 1;
                    }
                }
                while v.len() > 1 {
                    let r = calc_three(&mut v)?;
                    v.push(r);
                }
                if let Some(Experssion::Int(n)) = v.pop() {
                    return Ok(n);
                }
                err!("can not eval expression: {}", self.pretty())
            }
        }
    }
}

fn calc_three(v: &mut Vec<Experssion>) -> Result<Experssion> {
    if let Some(Experssion::Int(op1)) = v.pop() {
        if let Some(Experssion::Op(op)) = v.pop() {
            if let Some(Experssion::Int(op2)) = v.pop() {
                return Ok(Experssion::Int(calc(op, op1, op2)));
            }
        }
    }
    err!("can not eval expression")
}

fn calc(op: char, op1: usize, op2: usize) -> usize {
    if op == '+' {
        op1 + op2
    } else if op == '*' {
        op1 * op2
    } else {
        unreachable!()
    }
}

#[test]
fn example_input() {
    let e = parse_input(&"4 * 5".chars().filter(|&c| c != ' ').collect::<Vec<_>>()).unwrap();
    assert_eq!(e.eval().unwrap(), 20);

    let e = parse_input(
        &"2 * 3 + (4 * 5)"
            .chars()
            .filter(|&c| c != ' ')
            .collect::<Vec<_>>(),
    )
    .unwrap();
    assert_eq!(e.eval().unwrap(), 26);
    assert_eq!(e.advance_eval().unwrap(), 46);

    let e = parse_input(
        &"5 + (8 * 3 + 9 + 3 * 4 * 3)"
            .chars()
            .filter(|&c| c != ' ')
            .collect::<Vec<_>>(),
    )
    .unwrap();
    assert_eq!(e.eval().unwrap(), 437);
    assert_eq!(e.advance_eval().unwrap(), 1445);

    let e = parse_input(
        &"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
            .chars()
            .filter(|&c| c != ' ')
            .collect::<Vec<_>>(),
    )
    .unwrap();
    assert_eq!(e.eval().unwrap(), 13632);
    assert_eq!(e.advance_eval().unwrap(), 23340);
}
