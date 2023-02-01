use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Coord = (usize, usize);

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let (portals, grid) = parse_grid(&input);

    part1(&portals, &grid)?;
    part2(&portals, &grid)?;
    Ok(())
}

fn part1(portals: &HashMap<String, Vec<Coord>>, grid: &[Vec<Kind>]) -> Result<usize> {
    let start = Instant::now();

    let src = portals.get("AA").unwrap()[0];
    let dest = portals.get("ZZ").unwrap()[0];
    let result = bfs(src, dest, grid, portals)?;

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(portals: &HashMap<String, Vec<Coord>>, grid: &[Vec<Kind>]) -> Result<usize> {
    let start = Instant::now();

    let src = portals.get("AA").unwrap()[0];
    let dest = portals.get("ZZ").unwrap()[0];
    let result = bfs_with_level(src, dest, grid, portals)?;

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn bfs_with_level(
    src: Coord,
    dest: Coord,
    grid: &[Vec<Kind>],
    portals: &HashMap<String, Vec<Coord>>,
) -> Result<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((src, 0, 0));
    while let Some((cur, depth, level)) = queue.pop_front() {
        if cur == dest && level == 0 {
            return Ok(depth);
        }
        if visited.insert((cur, level)) {
            let (x, y) = cur;
            for next in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                match &grid[next.0][next.1] {
                    Kind::Open => queue.push_back((next, depth + 1, level)),
                    Kind::Portal(s) => {
                        for &other in portals.get(s).unwrap() {
                            if other != next {
                                let new_level =
                                    next_level(next.0, next.1, level, grid.len(), grid[0].len());

                                queue.push_back((other, depth + 2, new_level))
                            }
                        }
                    }
                    _ => continue,
                }
            }
        }
    }
    err!("can not found path from AA to ZZ")
}

fn next_level(x: usize, y: usize, level: usize, height: usize, width: usize) -> usize {
    if x == 2 || x == height - 3 || y == 2 || y == width - 3 {
        level.saturating_sub(1)
    } else {
        level + 1
    }
}

fn bfs(
    src: Coord,
    dest: Coord,
    grid: &[Vec<Kind>],
    portals: &HashMap<String, Vec<Coord>>,
) -> Result<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((src, 0));
    while let Some((cur, depth)) = queue.pop_front() {
        if cur == dest {
            return Ok(depth);
        }
        if visited.insert(cur) {
            let (x, y) = cur;
            for next in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                match &grid[next.0][next.1] {
                    Kind::Open => queue.push_back((next, depth + 1)),
                    Kind::Portal(s) => {
                        for &other in portals.get(s).unwrap() {
                            if other != next {
                                queue.push_back((other, depth + 2))
                            } else {
                                queue.push_back((other, depth + 1))
                            }
                        }
                    }
                    _ => continue,
                }
            }
        }
    }

    err!("can not found path from AA to ZZ")
}

#[derive(Debug, Clone)]
enum Kind {
    Wall,
    Open,
    Portal(String),
    None,
}

fn parse_grid(input: &str) -> (HashMap<String, Vec<Coord>>, Vec<Vec<Kind>>) {
    let raw: Vec<Vec<_>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();
    let mut portals: HashMap<String, Vec<Coord>> = HashMap::new();
    let mut grid = vec![vec![Kind::None; raw[0].len()]; raw.len()];

    for (x, row) in raw.iter().enumerate() {
        for (y, &c) in row.iter().enumerate() {
            if c == '#' {
                grid[x][y] = Kind::Wall;
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
                    grid[x][y] = Kind::Open;
                    continue;
                };
                grid[x][y] = Kind::Portal(label.clone());
                portals.entry(label).or_default().push((x, y));
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

    let input = "
             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ";
    let (portals, grid) = parse_grid(input);
    assert_eq!(part2(&portals, &grid).unwrap(), 396);
}
