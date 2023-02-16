use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Instr<'a> = (&'a str, &'a str, Box<dyn Fn(i32, i32) -> i32>);

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let instrs = parse_input(&input)?;

    part1(&instrs)?;
    part2(&instrs)?;
    Ok(())
}

fn part1(instrs: &[Instr]) -> Result<i32> {
    let start = Instant::now();

    let mut registers: HashMap<&str, i32> = HashMap::new();

    for (v1, v2, f) in instrs {
        let &v2 = registers.get(v2).unwrap_or(&0);
        let v1 = registers.entry(v1).or_insert(0);
        *v1 = f(*v1, v2);
    }

    let result = *registers.values().max().unwrap();
    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(instrs: &[Instr]) -> Result<i32> {
    let start = Instant::now();

    let mut registers: HashMap<&str, i32> = HashMap::new();

    let mut result = i32::MIN;
    for (v1, v2, f) in instrs {
        let &v2 = registers.get(v2).unwrap_or(&0);
        let v1 = registers.entry(v1).or_insert(0);
        *v1 = f(*v1, v2);
        result = result.max(*v1);
    }

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn parse_instr(row: &str) -> Result<Instr> {
    if let Some((left, right)) = row.trim().split_once(" if ") {
        let left: Vec<_> = left.split_whitespace().collect();
        let modify_reg = left[0].trim();
        let modify_op = if left[1].trim() == "inc" {
            i32::saturating_add
        } else {
            i32::saturating_sub
        };
        let modify_value: i32 = left[2].trim().parse().unwrap();

        let right: Vec<_> = right.split_whitespace().collect();
        let cond_reg = right[0].trim();
        let cond = match right[1].trim() {
            "==" => i32::eq,
            "!=" => i32::ne,
            "<=" => i32::le,
            ">=" => i32::ge,
            "<" => i32::lt,
            ">" => i32::gt,
            _ => return err!("Not a valid condition: {row}"),
        };
        let cond_value: i32 = right[2].trim().parse().unwrap();
        Ok((
            modify_reg,
            cond_reg,
            Box::new(move |a, b| {
                if cond(&b, &cond_value) {
                    modify_op(a, modify_value)
                } else {
                    a
                }
            }),
        ))
    } else {
        err!("Not a valid instruction: {row}")
    }
}

fn parse_input(input: &str) -> Result<Vec<Instr>> {
    input.lines().map(parse_instr).collect()
}

#[test]
fn example_input() {
    let input = "b inc 5 if a > 1
        a inc 1 if b < 5
        c dec -10 if a >= 1
        c inc -20 if c == 10";
    let instrs = parse_input(input).unwrap();
    assert_eq!(part1(&instrs).unwrap(), 1);
    assert_eq!(part2(&instrs).unwrap(), 10);
}
