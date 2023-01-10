use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::ops::Range;
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Fields<'a> = HashMap<&'a str, Vec<Range<usize>>>;
type Ticket = Vec<usize>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let (fields, my_ticket, nearby_tickets) = parse_input(&input)?;

    part1(&fields, &nearby_tickets)?;
    part2(&fields, &my_ticket, &nearby_tickets)?;
    Ok(())
}

fn part1(fields: &Fields, nearby_tickets: &[Ticket]) -> Result<usize> {
    let start = Instant::now();

    let mut error_rate = 0;
    for v in nearby_tickets {
        for n in v {
            if !fields.values().flatten().any(|r| r.contains(n)) {
                error_rate += n;
            }
        }
    }

    writeln!(io::stdout(), "Part 1: {error_rate}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(error_rate)
}

fn part2(fields: &Fields, my_ticket: &Ticket, nearby_tickets: &[Ticket]) -> Result<usize> {
    let start = Instant::now();

    let valid_tickets: Vec<_> = nearby_tickets
        .iter()
        .filter(|v| {
            v.iter()
                .all(|n| fields.values().flatten().any(|r| r.contains(n)))
        })
        .collect();

    let mut index = HashMap::new();
    let mut found = vec![];

    let fields_count = my_ticket.len();
    while index.len() < fields_count {
        'search: for i in (0..fields_count).filter(|i| !found.contains(i)) {
            let mut name = None;
            for (k, rs) in fields.iter().filter(|(name, _)| !index.contains_key(name)) {
                if valid_tickets
                    .iter()
                    .map(|t| &t[i])
                    .all(|n| rs.iter().any(|r| r.contains(n)))
                {
                    if name.is_none() {
                        name = Some(k);
                    } else {
                        continue 'search;
                    }
                }
            }
            if let Some(name) = name {
                // println!("Found fields: {}", name);
                index.insert(name, i);
                found.push(i);
                break;
            }
        }
    }

    let result = index
        .iter()
        .filter(|(name, _)| name.starts_with("departure"))
        .map(|(_, i)| my_ticket[*i])
        .product();

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn parse_input(input: &str) -> Result<(Fields, Ticket, Vec<Ticket>)> {
    let mut fields: Fields = HashMap::new();
    let mut my_ticket = vec![];
    let mut nearby_tickets = vec![];
    let mut part = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            part += 1;
            continue;
        }
        if part == 0 {
            if let Some((name, ranges)) = line.trim().split_once(": ") {
                for r in ranges.split(" or ") {
                    if let Some((start, end)) = r.split_once('-') {
                        fields
                            .entry(name)
                            .or_default()
                            .push(start.parse::<usize>()?..end.parse::<usize>()? + 1);
                    }
                }
            }
        } else if part == 1 {
            if line.trim().starts_with("your ticket:") {
                continue;
            }
            my_ticket = line
                .trim()
                .split(',')
                .map(|s| s.trim().parse::<usize>().unwrap())
                .collect();
        } else {
            if line.trim().starts_with("nearby tickets:") {
                continue;
            }
            nearby_tickets.push(
                line.trim()
                    .split(',')
                    .map(|s| s.trim().parse::<usize>().unwrap())
                    .collect(),
            )
        }
    }
    Ok((fields, my_ticket, nearby_tickets))
}

#[test]
fn example_input() {
    let input = "class: 1-3 or 5-7
    row: 6-11 or 33-44
    seat: 13-40 or 45-50
    
    your ticket:
    7,1,14
    
    nearby tickets:
    7,3,47
    40,4,50
    55,2,20
    38,6,12";
    let (fields, _, nearby_tickets) = parse_input(input).unwrap();
    assert_eq!(part1(&fields, &nearby_tickets).unwrap(), 71);

    let input = "class: 0-1 or 4-19
    row: 0-5 or 8-19
    seat: 0-13 or 16-19

    your ticket:
    11,12,13

    nearby tickets:
    3,9,18
    15,1,5
    5,14,9";
    let (fields, my_ticket, nearby_tickets) = parse_input(input).unwrap();
    part2(&fields, &my_ticket, &nearby_tickets).unwrap();
}
