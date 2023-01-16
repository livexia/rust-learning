use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type OrbIndex<'a> = HashMap<&'a str, OrbId>;
type OrbId = usize;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let (edges, index) = parse_input(&input);

    part1(&edges)?;
    part2(&edges, &index)?;
    Ok(())
}

fn part1(edges: &[(OrbId, OrbId)]) -> Result<usize> {
    let start = Instant::now();

    let (orb_count, _) = bfs(edges);
    let total_count = orb_count.iter().sum();

    writeln!(io::stdout(), "Part 1: {total_count}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(total_count)
}

fn part2(edges: &[(OrbId, OrbId)], index: &OrbIndex) -> Result<usize> {
    let start = Instant::now();

    let (_, prefix) = bfs(edges);
    let &you_id = index.get("YOU").unwrap();
    let &san_id = index.get("SAN").unwrap();
    let mut you_route = find_route(&prefix, you_id);
    let mut san_route = find_route(&prefix, san_id);
    let mut count = 0;
    while !you_route.is_empty() && !san_route.is_empty() {
        let v1 = you_route.pop().unwrap();
        let v2 = san_route.pop().unwrap();
        if v1 != v2 {
            break;
        }
    }
    count += you_route.len() + san_route.len();

    writeln!(io::stdout(), "Part 2: {count}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(count)
}

fn bfs(edges: &[(OrbId, OrbId)]) -> (Vec<usize>, Vec<OrbId>) {
    let adjacent = edges_to_adjacent(edges);
    let mut orb_count = vec![0; adjacent.len()];
    let mut prefix: Vec<_> = (0..adjacent.len()).collect();
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    while let Some((cur, count)) = queue.pop_front() {
        orb_count[cur] = count;
        for &next in &adjacent[cur] {
            prefix[next] = cur;
            queue.push_back((next, count + 1))
        }
    }
    (orb_count, prefix)
}

fn find_route(prefix: &[OrbId], dest: OrbId) -> Vec<OrbId> {
    let mut route = vec![];
    let mut pre = dest;
    while pre != prefix[pre] {
        route.push(pre);
        pre = prefix[pre];
    }
    route
}

fn edges_to_adjacent(edges: &[(OrbId, OrbId)]) -> Vec<Vec<OrbId>> {
    let mut r = vec![];
    for &(start, end) in edges {
        let max = start.max(end);
        if max >= r.len() {
            r.extend((r.len()..=max).map(|_| vec![]));
        }
        r[start].push(end)
    }
    r
}

fn parse_input(input: &str) -> (Vec<(OrbId, OrbId)>, OrbIndex) {
    fn get_id<'a>(index: &mut OrbIndex<'a>, name: &'a str, last_id: &mut OrbId) -> OrbId {
        if let Some(&id) = index.get(name) {
            id
        } else {
            *last_id += 1;
            index.insert(name, *last_id);
            *last_id
        }
    }
    let mut orb_index = OrbIndex::new();
    orb_index.insert("COM", 0);
    let mut last_id = 0;
    let mut edges = vec![];
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if let Some((start, end)) = line.trim().split_once(')') {
            let start = get_id(&mut orb_index, start.trim(), &mut last_id);
            let end = get_id(&mut orb_index, end.trim(), &mut last_id);
            edges.push((start, end));
        }
    }

    (edges, orb_index)
}

#[test]
fn example_input() {
    let input = "COM)B
    B)C
    C)D
    D)E
    E)F
    B)G
    G)H
    D)I
    E)J
    J)K
    K)L";

    let (edges, _) = parse_input(input);
    assert_eq!(part1(&edges).unwrap(), 42);

    let input = "COM)B
    B)C
    C)D
    D)E
    E)F
    B)G
    G)H
    D)I
    E)J
    J)K
    K)L
    K)YOU
    I)SAN";

    let (edges, index) = parse_input(input);
    assert_eq!(part2(&edges, &index).unwrap(), 4);
}
