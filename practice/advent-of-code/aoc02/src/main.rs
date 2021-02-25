use std::collections::HashMap;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
} 

fn part1(input: &str) -> Result<()> {
    let mut two_times= 0;
    let mut three_times = 0;

    for line in input.lines() {
        let mut count: HashMap<char, u8> = HashMap::new();
        for c in line.chars() {
            let id =  count.entry(c).or_insert(0);
            *id += 1;
        }
        if count.iter().any(|(_, f)| *f == 2) {
            two_times += 1;
        }
        if count.iter().any(|(_, f)| *f == 3) {
            three_times += 1;
        }
    }

    writeln!(io::stdout(), "{}", two_times * three_times)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut id = String::new();

    let lines: Vec<_> = input.lines().collect();
    let n = lines.len();
    for i in 0..n {
        for j in i..n {
            let mut flag = 0;
            for (a, b) in lines[i].chars().zip(lines[j].chars()) {
                if a != b {
                    flag += 1;
                }
                if flag > 1 {
                    break;
                }
            }
            if flag == 1 {
                for (a, b) in lines[i].chars().zip(lines[j].chars()) {
                    if a == b {
                        id.push(a);
                    }
                }
                break;
            }
        }
    }

    writeln!(io::stdout(), "{}", id)?;
    Ok(())
}
