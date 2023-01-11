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
    let tiles = parse_input(&input);

    part1(&tiles)?;
    // part2()?;
    Ok(())
}

fn part1(tiles: &[Tile]) -> Result<usize> {
    let start = Instant::now();

    let mut result = vec![];
    let edges: Vec<[u16; 4]> = tiles.iter().map(|t| t.edges()).collect();

    let l = tiles.len();

    for i in 0..l {
        if edges[i]
            .iter()
            .filter(|&&cur| {
                (0..l)
                    .filter(|&j| j != i)
                    .all(|j| !possible_adjacent(cur, &edges[j], 10))
            })
            .count()
            == 2
        {
            result.push(tiles[i].id);
        }
    }
    assert_eq!(result.len(), 4);
    let result = result.iter().product();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

struct Tile {
    id: usize,
    image: Vec<u16>,
}

impl Tile {
    fn new(id: usize) -> Self {
        Tile { id, image: vec![] }
    }

    fn update_image(&mut self, row: u16) {
        self.image.push(row);
    }

    fn edges(&self) -> [u16; 4] {
        let mut edges = [0; 4];
        edges[0] = self.image[0];
        edges[1] = *self.image.last().unwrap();
        edges[2] = {
            let mut r = 0;
            for (i, row) in self.image.iter().enumerate() {
                if row & 1 == 1 {
                    r |= 1 << i;
                }
            }
            r
        };
        edges[3] = {
            let mut r = 0;
            let mask = 1 << (self.image.len() - 1);
            for (i, row) in self.image.iter().enumerate() {
                if row & mask != 0 {
                    r |= 1 << i;
                }
            }
            r
        };

        edges
    }
}

fn reverse(edge: u16, length: usize) -> u16 {
    let mut r = 0;
    let mut edge = edge;
    let mut length = length;
    while edge != 0 {
        length -= 1;
        r |= (edge & 1) << length;
        edge >>= 1;
    }
    r
}

fn possible_adjacent(edge: u16, edges: &[u16; 4], length: usize) -> bool {
    for &other in edges {
        if edge == other || edge == reverse(other, length) {
            return true;
        }
    }
    false
}

fn parse_input(input: &str) -> Vec<Tile> {
    let mut tiles = vec![];

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        } else if let Some(id) = line.trim().strip_prefix("Tile ") {
            if let Some(id) = id.trim().strip_suffix(':') {
                tiles.push(Tile::new(id.trim().parse().unwrap()));
            } else {
                unreachable!()
            }
        } else {
            let line = line.trim();
            let mut row = 0;
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    row |= 1 << i;
                }
            }
            tiles.last_mut().unwrap().update_image(row);
        }
    }

    tiles
}

#[test]
fn example_input() {
    let input = "Tile 2311:
    ..##.#..#.
    ##..#.....
    #...##..#.
    ####.#...#
    ##.##.###.
    ##...#.###
    .#.#.#..##
    ..#....#..
    ###...#.#.
    ..###..###
    
    Tile 1951:
    #.##...##.
    #.####...#
    .....#..##
    #...######
    .##.#....#
    .###.#####
    ###.##.##.
    .###....#.
    ..#.#..#.#
    #...##.#..
    
    Tile 1171:
    ####...##.
    #..##.#..#
    ##.#..#.#.
    .###.####.
    ..###.####
    .##....##.
    .#...####.
    #.##.####.
    ####..#...
    .....##...
    
    Tile 1427:
    ###.##.#..
    .#..#.##..
    .#.##.#..#
    #.#.#.##.#
    ....#...##
    ...##..##.
    ...#.#####
    .#.####.#.
    ..#..###.#
    ..##.#..#.
    
    Tile 1489:
    ##.#.#....
    ..##...#..
    .##..##...
    ..#...#...
    #####...#.
    #..#.#.#.#
    ...#.#.#..
    ##.#...##.
    ..##.##.##
    ###.##.#..
    
    Tile 2473:
    #....####.
    #..#.##...
    #.##..#...
    ######.#.#
    .#...#.#.#
    .#########
    .###.#..#.
    ########.#
    ##...##.#.
    ..###.#.#.
    
    Tile 2971:
    ..#.#....#
    #...###...
    #.#.###...
    ##.##..#..
    .#####..##
    .#..####.#
    #..#.#..#.
    ..####.###
    ..#.#.###.
    ...#.#.#.#
    
    Tile 2729:
    ...#.#.#.#
    ####.#....
    ..#.#.....
    ....#..#.#
    .##..##.#.
    .#.####...
    ####.#.#..
    ##.####...
    ##..#.##..
    #.##...##.
    
    Tile 3079:
    #.#.#####.
    .#..######
    ..#.......
    ######....
    ####.#..#.
    .#...#.##.
    #.#####.##
    ..#.###...
    ..#.......
    ..#.###...";

    let tiles = parse_input(input);
    assert_eq!(part1(&tiles).unwrap(), 20899048083289);
}
