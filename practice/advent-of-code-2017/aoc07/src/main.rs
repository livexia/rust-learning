use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Index<'a> = HashMap<&'a str, usize>;
type Weight = Vec<i32>;
type Connect = Vec<Vec<usize>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (index, weights, connect) = parse_input(&input);

    part1(&index, &weights, &connect)?;
    part2(&index, &weights, &connect)?;
    Ok(())
}

fn part1<'a>(index: &Index<'a>, weights: &Weight, connect: &Connect) -> Result<&'a str> {
    let start = Instant::now();

    let id = get_root(weights, connect);
    let result = index.iter().find(|(_, &i)| i == id).unwrap().0;

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(_index: &Index<'_>, weights: &Weight, connect: &Connect) -> Result<i32> {
    let start = Instant::now();

    let id = get_root(weights, connect);
    let result = dfs(id, weights, connect).unwrap_err();

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn dfs(id: usize, weights: &Weight, connect: &Connect) -> std::result::Result<i32, i32> {
    let mut r = vec![];
    for &n in &connect[id] {
        match dfs(n, weights, connect) {
            Ok(w) => r.push(w),
            Err(w) => return Err(w),
        }
    }
    if let Some((i, d)) = find_difference(&r) {
        Err(weights[connect[id][i]] + d)
    } else {
        Ok(r.iter().sum::<i32>() + weights[id])
    }
}

fn find_difference(r: &[i32]) -> Option<(usize, i32)> {
    if r.len() < 3 {
        None
    } else {
        if r.iter().sum::<i32>() == r[0] * r.len() as i32 {
            return None;
        }
        let mut w_count = HashMap::new();
        for w in r.iter() {
            *w_count.entry(w).or_insert(0) += 1;
        }
        let &&w = w_count.iter().find(|(_, &c)| c == 1).unwrap().0;
        let i = r.iter().position(|&x| x == w).unwrap();
        let o_w = r.iter().find(|&&x| x != w).unwrap();
        Some((i, o_w - w))
    }
}

fn get_root(weights: &Weight, connect: &Connect) -> usize {
    let mut adjacent: Vec<_> = (0..weights.len()).collect();
    for (i, next) in connect.iter().enumerate() {
        for &j in next {
            adjacent[j] = i;
        }
    }

    while adjacent[0] != adjacent[adjacent[0]] {
        adjacent[0] = adjacent[adjacent[0]];
    }

    adjacent[0]
}

fn parse_input(input: &str) -> (Index, Weight, Connect) {
    fn get_id<'a>(
        name: &'a str,
        index: &mut Index<'a>,
        last_id: &mut usize,
        connect: &mut Connect,
        weights: &mut Weight,
    ) -> usize {
        if let Some(id) = index.get(&name) {
            *id
        } else {
            for _ in weights.len()..=*last_id {
                weights.push(0);
                connect.push(vec![]);
            }
            index.insert(name, *last_id);
            *last_id += 1;
            *last_id - 1
        }
    }

    let mut index = HashMap::new();
    let mut last_id = 0;

    let mut weights = Vec::new();
    let mut connect = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let (left, right) = if let Some((left, right)) = line.trim().split_once(" -> ") {
            (left, right)
        } else {
            (line.trim(), "")
        };
        let (name, weight) = if let Some((name, weight)) = left.trim().split_once(' ') {
            (
                name.trim(),
                weight
                    .trim_matches('(')
                    .trim_matches(')')
                    .trim()
                    .parse()
                    .unwrap(),
            )
        } else {
            unreachable!()
        };

        let id = get_id(name, &mut index, &mut last_id, &mut connect, &mut weights);
        weights[id] = weight;
        connect[id] = right
            .split(',')
            .filter(|w| !w.trim().is_empty())
            .map(|w| {
                get_id(
                    w.trim(),
                    &mut index,
                    &mut last_id,
                    &mut connect,
                    &mut weights,
                )
            })
            .collect();
    }
    (index, weights, connect)
}

#[test]
fn exmaple_input() {
    let input = "pbga (66)
        xhth (57)
        ebii (61)
        havc (66)
        ktlj (57)
        fwft (72) -> ktlj, cntj, xhth
        qoyq (66)
        padx (45) -> pbga, havc, qoyq
        tknk (41) -> ugml, padx, fwft
        jptl (61)
        ugml (68) -> gyxo, ebii, jptl
        gyxo (61)
        cntj (57)";
    let (index, weights, connect) = parse_input(input);
    assert_eq!("tknk", part1(&index, &weights, &connect).unwrap());
    assert_eq!(60, part2(&index, &weights, &connect).unwrap());
}
