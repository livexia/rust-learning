use std::collections::HashMap;
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
    let mut jolts: Vec<usize> = input.lines().map(|l| l.trim().parse().unwrap()).collect();
    jolts.push(0);
    jolts.sort();
    jolts.dedup();

    part1(&jolts)?;
    part2(&jolts)?;
    Ok(())
}

fn part1(jolts: &[usize]) -> Result<usize> {
    let start = Instant::now();

    let mut next = 0;
    let mut one_counter = 0;
    let mut three_counter = 0;
    while next + 1 < jolts.len() {
        for i in next + 1..jolts.len() {
            if jolts[i] - jolts[next] == 1 {
                next = i;
                one_counter += 1;
                break;
            } else if jolts[i] - jolts[next] == 3 {
                next = i;
                three_counter += 1;
                break;
            } else if jolts[i] - jolts[next] > 3 {
                next = jolts.len();
                break;
            }
        }
    }
    let result = one_counter * (three_counter + 1);

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(jolts: &[usize]) -> Result<usize> {
    let start = Instant::now();

    let result = dfs(jolts, 0, &mut HashMap::new());

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn dfs(jolts: &[usize], cur: usize, cache: &mut HashMap<usize, usize>) -> usize {
    if let Some(counter) = cache.get(&cur) {
        return *counter;
    }
    let mut counter = 0;
    for i in cur + 1..jolts.len() {
        if jolts[i] - jolts[cur] < 4 && jolts[i] - jolts[cur] > 0 {
            counter += dfs(jolts, i, cache);
        } else {
            break;
        }
    }
    if cur == jolts.len() - 1 {
        counter += 1;
    }
    cache.insert(cur, counter);
    counter
}

#[test]
fn example_input() {
    let input = "16
    10
    15
    5
    1
    11
    7
    19
    6
    12
    4";
    let mut jolts: Vec<usize> = input.lines().map(|l| l.trim().parse().unwrap()).collect();
    jolts.push(0);
    jolts.sort();
    jolts.dedup();
    assert_eq!(part1(&jolts).unwrap(), 35);
    assert_eq!(part2(&jolts).unwrap(), 8);
    let input = "28
    33
    18
    42
    31
    14
    46
    20
    48
    47
    24
    23
    49
    45
    19
    38
    39
    11
    1
    32
    25
    35
    8
    17
    7
    9
    4
    2
    34
    10
    3";

    let mut jolts: Vec<usize> = input.lines().map(|l| l.trim().parse().unwrap()).collect();
    jolts.push(0);
    jolts.sort();
    jolts.dedup();
    assert_eq!(part1(&jolts).unwrap(), 220);
    assert_eq!(part2(&jolts).unwrap(), 19208);
}
