use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

const TILE_LENGTH: usize = 10;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let tiles = parse_input(&input);

    part1(&tiles)?;
    part2(&tiles)?;
    Ok(())
}

fn part1(tiles: &[Tile]) -> Result<usize> {
    let start = Instant::now();

    let mut result = vec![];
    let edges: Vec<[u16; 4]> = tiles.iter().map(|t| t.edges()).collect();

    let l = tiles.len();

    for i in 0..l {
        if outermost_edges(i, &edges).len() == 2 {
            result.push(tiles[i].id);
        }
    }
    assert_eq!(result.len(), 4);
    let result = result.iter().product();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(tiles: &[Tile]) -> Result<usize> {
    let start = Instant::now();

    let edges: Vec<[u16; 4]> = tiles.iter().map(|t| t.edges()).collect();

    let l = tiles.len();
    let width = (l as f64).sqrt() as usize;

    let mut connected = vec![[(0, 0, 0); 4]; l]; // every tile edge connected
    for i in 0..l {
        for e in 0..4 {
            let cur = edges[i][e];
            for j in 0..l {
                if j == i {
                    continue;
                }
                let p = possible_adjacent(cur, &edges[j], TILE_LENGTH);
                if p.0 != 0 {
                    // println!(
                    //     "id: {} <-> {} : {} <-> {} {}",
                    //     tiles[i].id, tiles[j].id, e, p.1, p.0
                    // );
                    assert_eq!(connected[i][e], (0, 0, 0));
                    connected[i][e] = (j, p.1, p.0);
                }
            }
        }
    }

    let result = 0;

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

#[derive(Clone)]
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
        edges[1] = self.cloumn(self.image.len() - 1);

        edges[2] = *self.image.last().unwrap();
        edges[3] = self.cloumn(0);

        edges
    }

    fn flip_h(&mut self) {
        for row in &mut self.image {
            *row = reverse(*row, TILE_LENGTH);
        }
    }

    fn flip_v(&mut self) {
        let image = self.image.clone();
        let l = image.len() - 1;
        for i in 0..image.len() {
            self.image[l - i] = image[i];
        }
    }

    fn rotate_right(&mut self, times: usize) {
        let times = times % 4;
        for _ in 0..times {
            self.image = (0..self.image.len())
                .map(|c| reverse(self.cloumn(c), TILE_LENGTH))
                .collect();
        }
    }

    fn cloumn(&self, cloumn: usize) -> u16 {
        let mut r = 0;
        let mask = 1 << (self.image.len() - 1 - cloumn);
        for (i, row) in self.image.iter().rev().enumerate() {
            if row & mask != 0 {
                r |= 1 << i;
            }
        }
        r
    }

    fn draw(&self) -> String {
        let mut s = String::new();
        for row in &self.image {
            for i in 0..TILE_LENGTH {
                if row & (1 << (TILE_LENGTH - 1 - i)) != 0 {
                    s.push('#')
                } else {
                    s.push('.')
                }
            }
            s.push('\n')
        }

        s
    }
}

fn outermost_edges(id: usize, edges: &[[u16; 4]]) -> Vec<usize> {
    let l = edges.len();
    edges[id]
        .iter()
        .enumerate()
        .filter(|(_, &cur)| {
            (0..l)
                .filter(|j| *j != id)
                .all(|j| possible_adjacent(cur, &edges[j], TILE_LENGTH).0 == 0)
        })
        .map(|(i, _)| i)
        .collect()
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

fn possible_adjacent(edge: u16, edges: &[u16; 4], length: usize) -> (i32, usize) {
    for (i, &other) in edges.iter().enumerate() {
        if edge == other {
            return (1, i);
        } else if edge == reverse(other, length) {
            return (-1, i);
        }
    }
    (0, 0)
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
            let length = line.len() - 1;
            let mut row = 0;
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    row |= 1 << (length - i);
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
    assert_eq!(part2(&tiles).unwrap(), 273);

    // let mut t = tiles[tiles.len() - 1].clone();
}
