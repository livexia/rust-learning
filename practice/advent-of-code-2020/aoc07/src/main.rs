use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Index<'a> = HashMap<&'a str, usize>;
type Rule = Vec<(usize, usize)>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let (index, rules) = parse_input(&input);

    part1(&index, &rules)?;
    part1_reverse(&index, &rules)?;
    part2(&index, &rules)?;
    Ok(())
}

fn part1(index: &Index, rules: &[Rule]) -> Result<usize> {
    let start = Instant::now();

    let &shiny_gold = index.get("shiny gold").unwrap();
    let mut cache = HashMap::new();
    let result = (0..rules.len())
        .filter(|&bag| part1_dfs(rules, bag, shiny_gold, &mut cache))
        .count();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part1_dfs(rules: &[Rule], src: usize, dest: usize, cache: &mut HashMap<usize, bool>) -> bool {
    if let Some(&b) = cache.get(&src) {
        return b;
    }
    for &(bag, _) in &rules[src] {
        if bag == dest || part1_dfs(rules, bag, dest, cache) {
            cache.insert(src, true);
            return true;
        }
    }
    cache.insert(src, false);
    false
}

fn part1_reverse(index: &Index, rules: &[Rule]) -> Result<usize> {
    let start = Instant::now();

    let &shiny_gold = index.get("shiny gold").unwrap();
    let mut reverse_rules = vec![vec![]; rules.len()];
    for (i, bags) in rules.iter().enumerate() {
        for &(j, _) in bags {
            reverse_rules[j].push(i);
        }
    }
    let mut results: HashSet<usize> = HashSet::new();
    part1_reverse_dfs(&reverse_rules, shiny_gold, &mut results);
    let result = results.len();

    writeln!(io::stdout(), "Part 1 with reverse: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part1_reverse_dfs(rules: &[Vec<usize>], src: usize, results: &mut HashSet<usize>) {
    for &next in &rules[src] {
        if results.insert(next) {
            part1_reverse_dfs(rules, next, results);
        }
    }
}

fn part2(index: &Index, rules: &[Rule]) -> Result<usize> {
    let start = Instant::now();

    let &shiny_gold = index.get("shiny gold").unwrap();
    let result = part2_dfs(rules, shiny_gold, &mut HashMap::new());

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2_dfs(rules: &[Rule], src: usize, cache: &mut HashMap<usize, usize>) -> usize {
    if let Some(&c) = cache.get(&src) {
        return c;
    }
    let mut counter = 0;
    for &(bag, c) in &rules[src] {
        counter += c;
        counter += c * part2_dfs(rules, bag, cache);
    }
    cache.insert(src, counter);
    counter
}

fn parse_input(input: &str) -> (Index, Vec<Rule>) {
    fn get_index<'a>(index: &mut Index<'a>, rules: &mut Vec<Rule>, bag: &'a str) -> usize {
        match index.get(&bag) {
            Some(&i) => i,
            None => {
                index.insert(bag, rules.len());
                rules.push(vec![]);
                rules.len() - 1
            }
        }
    }
    let mut index = HashMap::new();
    let mut rules = vec![];
    for line in input.lines() {
        if let Some((bag, bags)) = line.trim().split_once("bags contain") {
            let bag = bag.trim();
            let i = get_index(&mut index, &mut rules, bag);
            for b in bags.split(',') {
                let b = if let Some(b) = b.trim().strip_suffix(" bag") {
                    b
                } else if let Some(b) = b.trim().strip_suffix(" bag.") {
                    b
                } else if let Some(b) = b.trim().strip_suffix(" bags") {
                    b
                } else if let Some(b) = b.trim().strip_suffix(" bags.") {
                    b
                } else {
                    unreachable!("{:?}", b)
                };
                if b == "no other" {
                    continue;
                } else if let Some((counter, bag)) = b.split_once(' ') {
                    let counter: usize = counter.parse().unwrap();
                    let j = get_index(&mut index, &mut rules, bag);
                    rules[i].push((j, counter));
                }
            }
        }
    }
    (index, rules)
}

#[test]
fn example_input() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
    dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    bright white bags contain 1 shiny gold bag.
    muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    dark olive bags contain 3 faded blue bags, 4 dotted black bags.
    vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
    faded blue bags contain no other bags.
    dotted black bags contain no other bags.";
    let (index, rules) = parse_input(&input);
    assert_eq!(part1(&index, &rules).unwrap(), 4);
    assert_eq!(part1_reverse(&index, &rules).unwrap(), 4);
    assert_eq!(part2(&index, &rules).unwrap(), 32);

    let input = "shiny gold bags contain 2 dark red bags.
    dark red bags contain 2 dark orange bags.
    dark orange bags contain 2 dark yellow bags.
    dark yellow bags contain 2 dark green bags.
    dark green bags contain 2 dark blue bags.
    dark blue bags contain 2 dark violet bags.
    dark violet bags contain no other bags.";
    let (index, rules) = parse_input(&input);
    assert_eq!(part2(&index, &rules).unwrap(), 126);
}
