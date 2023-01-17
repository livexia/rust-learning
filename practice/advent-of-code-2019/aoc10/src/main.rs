use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Coord = (i32, i32);

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let map = parse_input(&input);

    part1(&map)?;
    part2(&map)?;
    Ok(())
}

fn part1(map: &[Coord]) -> Result<usize> {
    let start = Instant::now();

    let visible = build_visible(map);

    let max_visible = visible.values().map(|h| h.len()).max().unwrap();

    writeln!(io::stdout(), "Part 1: {max_visible}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(max_visible)
}

fn part2(map: &[Coord]) -> Result<i32> {
    let start = Instant::now();

    let visible = build_visible(map);

    let (center, mut station_visible) = visible.into_iter().max_by_key(|(_, h)| h.len()).unwrap();

    // https://doc.rust-lang.org/std/primitive.f64.html#method.atan2
    // use atan2 to sort the visible location
    station_visible.sort_by(|&c1, &c2| atan2(c1, center).partial_cmp(&atan2(c2, center)).unwrap());
    assert!(station_visible.len() > 200);
    // find the first visible location
    let index = (station_visible
        .iter()
        .position(|&c| c.0 == center.0 && c.1 < center.1)
        .unwrap()
        + 199)
        % station_visible.len();

    let result = station_visible[index].0 * 100 + station_visible[index].1;

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn atan2(c1: Coord, c2: Coord) -> f64 {
    ((c1.1 - c2.1) as f64).atan2((c1.0 - c2.0) as f64)
}

fn build_visible(map: &[Coord]) -> HashMap<Coord, Vec<Coord>> {
    let mut r: HashMap<Coord, Vec<Coord>> = HashMap::new();
    for (i, &c1) in map.iter().enumerate() {
        for &c2 in map.iter().skip(i + 1) {
            if visible(c1, c2, &map[i + 1..]) {
                r.entry(c1).or_default().push(c2);
                r.entry(c2).or_default().push(c1);
            }
        }
    }

    r
}

fn visible(c1: Coord, c2: Coord, map: &[Coord]) -> bool {
    for &c3 in map {
        if c3 == c1 || c3 == c2 {
            continue;
        }
        if in_between(c1, c2, c3) {
            return false;
        }
    }
    true
}

fn in_between(c1: Coord, c2: Coord, c3: Coord) -> bool {
    // https://stackoverflow.com/a/328122
    let croos = cross_product(c1, c2, c3);
    if croos != 0 {
        return false;
    }
    let dot = dot_product(c1, c2, c3);
    if dot < 0 {
        return false;
    }
    let length = squared_lenth(c1, c2);
    if dot > length {
        return false;
    }
    true
}

fn cross_product(c1: Coord, c2: Coord, c3: Coord) -> i32 {
    (c3.1 - c1.1) * (c2.0 - c1.0) - (c3.0 - c1.0) * (c2.1 - c1.1)
}

fn dot_product(c1: Coord, c2: Coord, c3: Coord) -> i32 {
    (c3.0 - c1.0) * (c2.0 - c1.0) + (c3.1 - c1.1) * (c2.1 - c1.1)
}

fn squared_lenth(c1: Coord, c2: Coord) -> i32 {
    (c1.0 - c2.0).pow(2) + (c1.1 - c2.1).pow(2)
}

fn parse_input(input: &str) -> Vec<Coord> {
    let mut map = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.trim().char_indices() {
            if char == '#' {
                map.push((x as i32, y as i32));
            }
        }
    }
    map
}

#[test]
fn example_input() {
    let input = ".#..#
    .....
    #####
    ....#
    ...##";
    let map = parse_input(input);
    assert_eq!(part1(&map).unwrap(), 8);

    let input = ".#..##.###...#######
    ##.############..##.
    .#.######.########.#
    .###.#######.####.#.
    #####.##.#.##.###.##
    ..#####..#.#########
    ####################
    #.####....###.#.#.##
    ##.#################
    #####.##.###..####..
    ..######..##.#######
    ####.##.####...##..#
    .#####..#.######.###
    ##...#.##########...
    #.##########.#######
    .####.#.###.###.#.##
    ....##.##.###..#####
    .#.#.###########.###
    #.#.#.#####.####.###
    ###.##.####.##.#..##";
    let map = parse_input(input);
    assert_eq!(part1(&map).unwrap(), 210);
    assert_eq!(part2(&map).unwrap(), 802);
}
