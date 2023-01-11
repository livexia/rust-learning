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

    part1(&input)?;
    // part2()?;
    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    let start = Instant::now();

    let result: usize = input
        .lines()
        .map(|l| eval(&parse_expersion(l.trim())))
        .collect::<Result<Vec<_>>>()?
        .iter()
        .sum();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

#[derive(Debug, Clone, Copy)]
enum Experssion {
    Int(usize),
    Op(char),
    Parentheses(char),
}

fn pretty(expr: &[Experssion]) -> String {
    let mut s = String::new();
    for e in expr {
        match e {
            Experssion::Int(n) => s.push_str(&format!("{}", n)),
            Experssion::Op(c) => s.push(*c),
            Experssion::Parentheses(c) => s.push(*c),
        }
        s.push(' ');
    }
    s
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

fn eval(expr: &[Experssion]) -> Result<usize> {
    let mut stack = expr.to_vec();
    stack.reverse();
    while stack.len() > 1 {
        let mut prev_num = None;
        let mut prev_op = None;

        while let Some(cur) = stack.pop() {
            match cur {
                Experssion::Int(n) => {
                    if let Some(p_n) = prev_num {
                        if let Some(op) = prev_op {
                            let r = calc(op, p_n, n);
                            stack.push(Experssion::Int(r));
                            break;
                        }
                    } else {
                        prev_num = Some(n);
                    }
                }
                Experssion::Op(c) => {
                    prev_op = Some(c);
                }
                Experssion::Parentheses(c) => {
                    let mut left_count = 1;
                    let mut right_count = 0;
                    let mut new_stack = vec![];
                    if c == '(' {
                        while left_count != right_count {
                            let next = stack.pop().unwrap();
                            new_stack.push(next);
                            match next {
                                Experssion::Int(_) | Experssion::Op(_) => (),
                                Experssion::Parentheses(n_c) => {
                                    if n_c == '(' {
                                        left_count += 1
                                    } else {
                                        right_count += 1
                                    };
                                }
                            }
                        }
                        new_stack.pop();
                        let r = eval(&new_stack)?;
                        stack.push(Experssion::Int(r));
                    } else {
                        unreachable!("{:?} {}", cur, pretty(expr));
                    }
                }
            }
        }
    }

    if let Some(Experssion::Int(n)) = stack.pop() {
        return Ok(n);
    }
    err!("can not eval the experssion: {}", pretty(expr))
}

fn parse_expersion(input: &str) -> Vec<Experssion> {
    let mut expr = vec![];

    for c in input.chars() {
        if c == ' ' {
            continue;
        } else if c == '(' || c == ')' {
            expr.push(Experssion::Parentheses(c))
        } else if c.is_numeric() {
            expr.push(Experssion::Int((c as u8 - b'0') as usize))
        } else {
            expr.push(Experssion::Op(c))
        }
    }

    expr
}

#[test]
fn example_input() {
    assert_eq!(eval(&parse_expersion("4 * 5")).unwrap(), 20);
    assert_eq!(eval(&parse_expersion("2 * 3 + (4 * 5)")).unwrap(), 26);
    assert_eq!(
        eval(&parse_expersion("5 + (8 * 3 + 9 + 3 * 4 * 3)")).unwrap(),
        437
    );
    assert_eq!(
        eval(&parse_expersion(
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
        ))
        .unwrap(),
        13632
    );
}
