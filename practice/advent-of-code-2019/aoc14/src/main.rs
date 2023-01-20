use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Index<'a> = HashMap<&'a str, usize>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let (index, reactions) = parse_input(&input)?;

    part1(&index, &reactions)?;
    part2(&index, &reactions)?;
    Ok(())
}

fn part1(index: &Index, reactions: &[Option<Reaction>]) -> Result<usize> {
    let start = Instant::now();

    let &ore_id = index.get("ORE").unwrap();
    let &fuel_id = index.get("FUEL").unwrap();
    let count = reverse_dfs(reactions, fuel_id, ore_id, 1);

    writeln!(io::stdout(), "Part 1: {count}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(count)
}

fn part2(index: &Index, reactions: &[Option<Reaction>]) -> Result<usize> {
    let start = Instant::now();

    let &ore_id = index.get("ORE").unwrap();
    let &fuel_id = index.get("FUEL").unwrap();

    let count = reverse_dfs(reactions, fuel_id, ore_id, 1);
    let mut left = 1000000000000 / count;
    let mut right = left * 2;

    // let right = match (left..right)
    //     .collect::<Vec<_>>()
    //     .binary_search_by(|&c| reverse_dfs(reactions, fuel_id, ore_id, c).cmp(&1000000000000))
    // {
    //     Ok(n) => left + n - 1,
    //     Err(n) => left + n - 1,
    // };

    // binary search https://doc.rust-lang.org/src/core/slice/mod.rs.html#2452
    while left < right {
        let mid = (left + right) / 2;
        let count = reverse_dfs(reactions, fuel_id, ore_id, mid);
        match count.cmp(&1000000000000) {
            Ordering::Less => left = mid + 1,
            Ordering::Equal => break,
            Ordering::Greater => right = mid,
        }
    }

    left -= 1;

    writeln!(io::stdout(), "Part 2: {left}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(left)
}

fn reverse_dfs(
    reactions: &[Option<Reaction>],
    src: usize,
    dest: usize,
    fuel_count: usize,
) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((fuel_count, src));

    let mut remain_chemical = vec![0; reactions.len()];
    let mut result = 0;
    while let Some((count, cur)) = queue.pop_front() {
        if count <= remain_chemical[cur] {
            remain_chemical[cur] -= count;
            continue;
        }
        let count = count - remain_chemical[cur];
        remain_chemical[cur] = 0;
        if cur == dest {
            result += count;
            continue;
        }
        if let Some(reaction) = &reactions[cur] {
            let times = (count + reaction.right.0 - 1) / reaction.right.0;
            remain_chemical[cur] += times * reaction.right.0 - count;
            for &(c, part) in &reaction.left {
                queue.push_back((c * times, part));
            }
        } else {
            unreachable!()
        }
    }
    result
}

#[derive(Debug)]
struct Reaction {
    left: Vec<(usize, usize)>,
    right: (usize, usize),
}

fn parse_input(input: &str) -> Result<(Index, Vec<Option<Reaction>>)> {
    let mut index = HashMap::new();
    let mut last_id = 0;

    let mut reactions = vec![];

    for line in input.lines() {
        if let Some((left, right)) = line.trim().split_once(" => ") {
            let left = left
                .split(',')
                .map(|p| parse_part(p.trim(), &mut index, &mut last_id))
                .collect::<Result<Vec<_>>>()?;
            let right = parse_part(right.trim(), &mut index, &mut last_id)?;
            for _ in reactions.len()..=right.1 {
                reactions.push(None);
            }
            if reactions[right.1].is_some() {
                return err!("two reaction produce one same chemical");
            }
            reactions[right.1] = Some(Reaction { left, right });
        } else {
            return err!("not a valid reaction: {line}");
        }
    }

    Ok((index, reactions))
}

fn parse_part<'a>(
    part: &'a str,
    index: &mut Index<'a>,
    last_id: &mut usize,
) -> Result<(usize, usize)> {
    if let Some((count, chemical)) = part.split_once(' ') {
        let count: usize = count.parse()?;
        let chemical = get_id(index, chemical.trim(), last_id);
        return Ok((count, chemical));
    }
    err!("not a valid part for chemical: {part}")
}

fn get_id<'a>(index: &mut Index<'a>, name: &'a str, last_id: &mut usize) -> usize {
    if let Some(id) = index.get(&name) {
        *id
    } else {
        index.insert(name, *last_id);
        *last_id += 1;
        *last_id - 1
    }
}

#[test]
fn example_input() {
    let input = "9 ORE => 2 A
    8 ORE => 3 B
    7 ORE => 5 C
    3 A, 4 B => 1 AB
    5 B, 7 C => 1 BC
    4 C, 1 A => 1 CA
    2 AB, 3 BC, 4 CA => 1 FUEL";

    let (index, reactions) = parse_input(input).unwrap();
    assert_eq!(part1(&index, &reactions).unwrap(), 165);

    let input = "157 ORE => 5 NZVS
    165 ORE => 6 DCFZ
    44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
    12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
    179 ORE => 7 PSHF
    177 ORE => 5 HKGWZ
    7 DCFZ, 7 PSHF => 2 XJWVT
    165 ORE => 2 GPVTF
    3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

    let (index, reactions) = parse_input(input).unwrap();
    assert_eq!(part1(&index, &reactions).unwrap(), 13312);
    assert_eq!(part2(&index, &reactions).unwrap(), 82892753);

    let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
    17 NVRVD, 3 JNWZP => 8 VPVL
    53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
    22 VJHF, 37 MNCFX => 5 FWMGM
    139 ORE => 4 NVRVD
    144 ORE => 7 JNWZP
    5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
    5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
    145 ORE => 6 MNCFX
    1 NVRVD => 8 CXFTF
    1 VJHF, 6 MNCFX => 4 RFSQX
    176 ORE => 6 VJHF";

    let (index, reactions) = parse_input(input).unwrap();
    assert_eq!(part1(&index, &reactions).unwrap(), 180697);
    assert_eq!(part2(&index, &reactions).unwrap(), 5586022);

    let input = "171 ORE => 8 CNZTR
    7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
    114 ORE => 4 BHXH
    14 VRPVC => 6 BMBT
    6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
    6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
    15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
    13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
    5 BMBT => 4 WPTQ
    189 ORE => 9 KTJDG
    1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
    12 VRPVC, 27 CNZTR => 2 XDBXC
    15 KTJDG, 12 BHXH => 5 XCVML
    3 BHXH, 2 VRPVC => 7 MZWV
    121 ORE => 7 VRPVC
    7 XCVML => 6 RJRHP
    5 BHXH, 4 VRPVC => 5 LTCX";

    let (index, reactions) = parse_input(input).unwrap();
    assert_eq!(part1(&index, &reactions).unwrap(), 2210736);
    assert_eq!(part2(&index, &reactions).unwrap(), 460664);
}
