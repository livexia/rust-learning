use std::collections::{HashMap, HashSet, VecDeque};
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
    let result = bfs(src, dest, grid, portals).unwrap();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn bfs(
    src: Coord,
    dest: Coord,
    grid: &HashMap<Coord, Kind>,
    portals: &HashMap<String, Vec<Coord>>,
) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((src, 0));
    while let Some((cur, depth)) = queue.pop_front() {
        if visited.insert(cur) {
            if cur == dest {
                return Some(depth);
            }
            let (x, y) = cur;
            for next in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                if let Some(k) = grid.get(&next) {
                    match k {
                        Kind::Wall => continue,
                        Kind::Open => queue.push_back((next, depth + 1)),
                        Kind::Portal(s) => {
                            for &other in portals.get(s).unwrap() {
                                if other != next {
                                    queue.push_back((other, depth + 2))
                                }
                            }
                            queue.push_back((next, depth + 1))
                        }
                    }
                }
            }
        }
    }
    None
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
    assert_eq!(part1(&portals, &grid).unwrap(), 58);
}
