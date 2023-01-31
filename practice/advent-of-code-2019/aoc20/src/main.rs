use std::collections::{HashMap, HashSet};
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
    let (portals, grid) = parse_grid(&input);

    part1(&portals, &grid)?;
    // part2()?;
    Ok(())
}

fn part1(portals: &HashMap<String, Vec<Coord>>, grid: &HashMap<Coord, Kind>) -> Result<usize> {
    let start = Instant::now();

    let src = portals.get("AA").unwrap()[0];
    let dest = portals.get("ZZ").unwrap()[0];
    let result = dfs(
        src,
        dest,
        grid,
        portals,
        &mut HashMap::new(),
        &mut HashSet::new(),
    )
    .unwrap();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn dfs(
    src: Coord,
    dest: Coord,
    grid: &HashMap<Coord, Kind>,
    portals: &HashMap<String, Vec<Coord>>,
    cache: &mut HashMap<(Coord, Coord), Option<usize>>,
    visited: &mut HashSet<Coord>,
) -> Option<usize> {
    if let Some(&r) = cache.get(&(src, dest)) {
        return r;
    }
    visited.insert(src);
    if src == dest {
        return Some(0);
    }
    let mut result = usize::MAX;
    let (x, y) = src;
    for next in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
        if visited.insert(next) {
            if let Some(k) = grid.get(&next) {
                match k {
                    Kind::Wall => continue,
                    Kind::Open => {
                        if let Some(d) = dfs(next, dest, grid, portals, cache, visited) {
                            result = result.min(1 + d);
                        }
                    }
                    Kind::Portal(s) => {
                        for &other in portals.get(s).unwrap() {
                            if visited.insert(other) && other != next {
                                if let Some(d) = dfs(other, dest, grid, portals, cache, visited) {
                                    result = result.min(2 + d);
                                }
                            }
                        }

                        if let Some(d) = dfs(next, dest, grid, portals, cache, visited) {
                            result = result.min(1 + d);
                        }
                    }
                }
            }
        }
    }
    let r = if result == usize::MAX {
        None
    } else {
        Some(result)
    };
    cache.insert((src, dest), r);
    r
}

#[derive(Debug, Clone)]
enum Kind {
    Wall,
    Open,
    Portal(String),
}

fn parse_grid(input: &str) -> (HashMap<String, Vec<Coord>>, HashMap<Coord, Kind>) {
    let raw: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut portals: HashMap<String, Vec<(i32, i32)>> = HashMap::new();
    let mut grid = HashMap::new();

    for (x, row) in raw.iter().enumerate() {
        for (y, &c) in row.iter().enumerate() {
            if c == '#' {
                grid.insert((x as i32, y as i32), Kind::Wall);
            } else if c == '.' {
                // check 4 direction
                let label = if is_label(raw[x - 1][y]) {
                    label_to_hash(raw[x - 2][y], raw[x - 1][y])
                } else if is_label(raw[x + 1][y]) {
                    label_to_hash(raw[x + 1][y], raw[x + 2][y])
                } else if is_label(raw[x][y - 1]) {
                    label_to_hash(raw[x][y - 2], raw[x][y - 1])
                } else if is_label(raw[x][y + 1]) {
                    label_to_hash(raw[x][y + 1], raw[x][y + 2])
                } else {
                    grid.insert((x as i32, y as i32), Kind::Open);
                    continue;
                };
                grid.insert((x as i32, y as i32), Kind::Portal(label.clone()));
                portals.entry(label).or_default().push((x as i32, y as i32));
            }
        }
    }

    (portals, grid)
}

fn is_label(c: char) -> bool {
    c.is_ascii_uppercase()
}

fn label_to_hash(c1: char, c2: char) -> String {
    let mut s = c1.to_string();
    s.push(c2);
    s
}

#[test]
fn example_input() {
    let input = "
         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       ";
    let (portals, grid) = parse_grid(input);
    assert_eq!(part1(&portals, &grid).unwrap(), 23);

    let input = "
                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               ";
    let (portals, grid) = parse_grid(input);
    println!("{:?}", portals);
    assert_eq!(part1(&portals, &grid).unwrap(), 58);
}
