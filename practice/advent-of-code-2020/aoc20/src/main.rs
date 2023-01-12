use std::collections::{HashMap, HashSet};
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
    part2(&tiles)?;
    Ok(())
}

fn part1(tiles: &[Tile]) -> Result<usize> {
    let start = Instant::now();

    let mut result = vec![];
    let edges: Vec<[u16; 4]> = tiles.iter().map(|t| t.edges()).collect();

    for (i, tile) in tiles.iter().enumerate() {
        if outermost_edges(i, &edges, 10).len() == 2 {
            result.push(tile.id);
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

    let mut tiles = tiles.to_vec();
    let mapping = mapping_tiles(&mut tiles);
    let (image, size) = merge_tiles(&tiles, &mapping);

    let raw_monster = "                  # \n#    ##    ##    ###\n #  #  #  #  #  #   ";
    let mut monster = vec![];
    for line in raw_monster.lines() {
        let mut r = 0u128;
        for (i, c) in line.char_indices() {
            if c == '#' {
                r |= 1 << (line.len() - 1 - i);
            }
        }
        monster.push(r);
    }

    let monster_count = search_monster_with_rotate_and_flip(&image, size, &mut monster, 20, 3);

    let total_one: usize = image.iter().map(|&r| count_one(r)).sum();
    let monster_one: usize = monster.iter().map(|&r| count_one(r)).sum();
    let result = total_one - monster_count * monster_one;

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn rotate_monster(monster: &mut Vec<u128>, width: usize) {
    let mut temp = vec![0; width];
    for (i, new_row) in temp.iter_mut().enumerate() {
        let mut r = 0;

        let mask = 1 << (width - 1 - i);
        for (j, row) in monster.iter().enumerate() {
            if row & mask != 0 {
                r |= 1 << j
            }
        }
        *new_row = r;
    }
    *monster = temp;
}

fn flip_monster(monster: &mut [u128], width: usize) {
    for row in monster {
        *row = reverse_u128(*row, width);
    }
}

#[allow(dead_code)]
fn draw_monster(monster: &[u128], width: usize) -> String {
    let mut s = String::new();
    for row in monster {
        for i in 0..width {
            if row & (1 << (width - 1 - i)) != 0 {
                s.push('#')
            } else {
                s.push(' ')
            }
        }
        s.push('\n')
    }

    s
}

fn search_monster(image: &[u128], size: usize, monster: &[u128]) -> usize {
    let monster_height = monster.len();
    let mut count = 0;
    for i in 0..=(size - monster_height) {
        for j in 0..size {
            if monster
                .iter()
                .enumerate()
                .all(|(row, &monster_row)| (image[i + row] >> j) & monster_row == monster_row)
            {
                count += 1;
            }
        }
    }
    count
}

fn search_monster_with_rotate_and_flip(
    image: &[u128],
    size: usize,
    monster: &mut Vec<u128>,
    width: usize,
    height: usize,
) -> usize {
    let (mut width, mut height) = (width, height);
    for _ in 0..2 {
        for _ in 0..4 {
            let count = search_monster(image, size, monster);
            if count != 0 {
                return count;
            }
            rotate_monster(monster, width);
            (width, height) = (height, width);
        }
        flip_monster(monster, width)
    }
    0
}

fn count_one(num: u128) -> usize {
    let mut num = num;
    let mut count = 0;
    while num != 0 {
        if num & 1 == 1 {
            count += 1;
        }
        num >>= 1;
    }
    count
}

fn mapping_tiles(tiles: &mut [Tile]) -> HashMap<usize, i32> {
    let l = tiles.len();
    let tile_length = tiles[0].length;
    let width = (l as f64).sqrt() as i32;

    let mut mapping = HashMap::new();

    let mut stack = Vec::new();
    stack.push((0, 0));

    while let Some((cur, id)) = stack.pop() {
        let edges = tiles[cur].edges();
        'outer: for (e, &edge) in edges.iter().enumerate() {
            let other_edge = (2 + e) % 4;
            'inner: for (j, next) in tiles.iter_mut().enumerate() {
                if j == cur {
                    continue;
                }
                if mapping.contains_key(&j) {
                    continue;
                }
                if possible_adjacent(edge, &next.edges(), tile_length).0 != 0 {
                    for _ in 0..2 {
                        for _ in 0..4 {
                            if edge == next.edges()[other_edge] {
                                let next_id = if e == 0 {
                                    // up edge
                                    id - width
                                } else if e == 1 {
                                    // left edge
                                    id + 1
                                } else if e == 2 {
                                    // bottom edge
                                    id + width
                                } else if e == 3 {
                                    // right edge
                                    id - 1
                                } else {
                                    unreachable!()
                                };
                                stack.push((j, next_id));
                                continue 'outer;
                            }
                            if mapping.contains_key(&j) {
                                continue 'inner;
                            }
                            next.rotate_right();
                        }
                        next.flip_h();
                    }
                }
            }
        }

        mapping.insert(cur, id);
    }

    let &min = mapping.values().min().unwrap();
    for (_, v) in mapping.iter_mut() {
        *v -= min;
    }

    let all_id: HashSet<_> = mapping.values().collect();
    assert_eq!(all_id.len(), mapping.len());

    assert_eq!(l, mapping.len());
    mapping
}

fn merge_tiles(tiles: &[Tile], mapping: &HashMap<usize, i32>) -> (Vec<u128>, usize) {
    let tile_length = tiles[0].length - 2;
    let width = (tiles.len() as f64).sqrt() as usize;

    let image_width = tile_length * width;
    let mut image = vec![0u128; image_width];

    let mut order: Vec<_> = mapping.iter().collect();
    order.sort_by_key(|a| a.1);

    for i in 0..width {
        for j in 0..width {
            let &cur = order[i * width + j].0;
            let tile = tiles[cur].borderless();
            for (k, row) in tile.into_iter().enumerate() {
                let x = i * tile_length + k;
                let y = image_width - j * tile_length - tile_length;
                image[x] |= (row as u128) << y;
            }
        }
    }

    (image, image_width)
}

#[derive(Clone)]
struct Tile {
    id: usize,
    image: Vec<u16>,
    length: usize,
}

impl Tile {
    fn new(id: usize, length: usize) -> Self {
        Tile {
            id,
            image: vec![],
            length,
        }
    }

    fn update_image(&mut self, row: u16) {
        self.image.push(row);
    }

    fn edges(&self) -> [u16; 4] {
        let mut edges = [0; 4];
        edges[0] = self.image[0];
        edges[1] = self.column(self.image.len() - 1);

        edges[2] = *self.image.last().unwrap();
        edges[3] = self.column(0);

        edges
    }

    fn flip_h(&mut self) {
        for row in &mut self.image {
            *row = reverse_u16(*row, self.length);
        }
    }

    fn rotate_right(&mut self) {
        self.image = (0..self.image.len())
            .map(|c| reverse_u16(self.column(c), self.length))
            .collect();
    }

    fn column(&self, column: usize) -> u16 {
        let mut r = 0;
        let mask = 1 << (self.image.len() - 1 - column);
        for (i, row) in self.image.iter().rev().enumerate() {
            if row & mask != 0 {
                r |= 1 << i;
            }
        }
        r
    }

    fn borderless(&self) -> Vec<u16> {
        let mut image = vec![];
        for i in 1..self.length - 1 {
            image.push(without_head_and_tail(self.image[i], self.length))
        }
        image
    }
}

trait Draw {
    fn draw(&self, length: usize) -> String;
}

impl Draw for [u16] {
    fn draw(&self, length: usize) -> String {
        let mut s = String::new();
        for row in self {
            for i in 0..length {
                if row & (1 << (length - 1 - i)) != 0 {
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

impl Draw for [u128] {
    fn draw(&self, length: usize) -> String {
        let mut s = String::new();
        for row in self {
            for i in 0..length {
                if row & (1 << (length - 1 - i)) != 0 {
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

fn without_head_and_tail(edge: u16, length: usize) -> u16 {
    (edge & (!(1 << (length - 1)))) >> 1
}

fn outermost_edges(id: usize, edges: &[[u16; 4]], length: usize) -> Vec<usize> {
    let l = edges.len();
    edges[id]
        .iter()
        .enumerate()
        .filter(|(_, &cur)| {
            (0..l)
                .filter(|j| *j != id)
                .all(|j| possible_adjacent(cur, &edges[j], length).0 == 0)
        })
        .map(|(i, _)| i)
        .collect()
}

fn reverse_u16(edge: u16, length: usize) -> u16 {
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

fn reverse_u128(edge: u128, length: usize) -> u128 {
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
        } else if edge == reverse_u16(other, length) {
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
                tiles.push(Tile::new(id.trim().parse().unwrap(), 10));
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

    let t = tiles[tiles.len() - 1].clone();
    println!("{}", &t.borderless().draw(t.length - 2));
}
