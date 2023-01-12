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

fn part1(tiles: &[Image]) -> Result<usize> {
    let start = Instant::now();

    let mut result = vec![];
    let edges: Vec<_> = tiles.iter().map(|t| t.edges()).collect();

    for (i, tile) in tiles.iter().enumerate() {
        if is_corner_tile(i, &edges, 10) {
            result.push(tile.id);
        }
    }
    assert_eq!(result.len(), 4);
    let result = result.iter().product();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(tiles: &[Image]) -> Result<usize> {
    let start = Instant::now();

    let mut tiles = tiles.to_vec();
    let mapping = mapping_tiles(&mut tiles);
    let image = Image::from_images(&tiles, &mapping);

    let raw_monster = "                  # \n#    ##    ##    ###\n #  #  #  #  #  #   ";
    let mut monster = Image::from_str(55555, raw_monster);

    let total_one: usize = image.count_one();
    let monster_one: usize = monster.count_one();

    let monster_count = image.search_image_with_rotate_and_flip(&mut monster);
    let result = total_one - monster_count * monster_one;

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn mapping_tiles(tiles: &mut [Image]) -> HashMap<usize, i32> {
    let l = tiles.len();
    let tile_length = tiles[0].width;
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
                if possible_adjacent_tile(edge, &next.edges(), tile_length) {
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

#[derive(Clone)]
struct Image {
    id: usize,
    raw: Vec<u128>,
    width: usize,
    height: usize,
}

impl Image {
    fn new(id: usize, width: usize, height: usize) -> Self {
        Image {
            id,
            raw: vec![],
            width,
            height,
        }
    }

    fn from_str(id: usize, s: &str) -> Self {
        let mut raw = vec![];
        let mut width = 0;
        for line in s.lines() {
            let mut r = 0u128;
            if width == 0 {
                width = line.len();
            } else {
                assert_eq!(width, line.len())
            }
            for (i, c) in line.char_indices() {
                if c == '#' {
                    r |= 1 << (line.len() - 1 - i);
                }
            }
            raw.push(r);
        }
        let height = raw.len();
        Image {
            id,
            raw,
            width,
            height,
        }
    }

    fn search_image(&self, other: &Image) -> usize {
        let mut count = 0;
        for i in 0..=(self.height - other.height) {
            for j in 0..self.width {
                if other
                    .raw
                    .iter()
                    .enumerate()
                    .all(|(row, &other_row)| (self.raw[i + row] >> j) & other_row == other_row)
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn search_image_with_rotate_and_flip(&self, other: &mut Image) -> usize {
        for _ in 0..2 {
            for _ in 0..4 {
                let count = self.search_image(other);
                if count != 0 {
                    return count;
                }
                other.rotate_right();
            }
            // flip_monster(other, width)
            other.flip_h();
        }
        0
    }

    fn from_images(tiles: &[Image], mapping: &HashMap<usize, i32>) -> Self {
        let tile_length = tiles[0].width - 2;
        let width = (tiles.len() as f64).sqrt() as usize;

        let image_width = tile_length * width;
        let mut raw = vec![0u128; image_width];

        let mut order: Vec<_> = mapping.iter().collect();
        order.sort_by_key(|a| a.1);

        for i in 0..width {
            for j in 0..width {
                let &cur = order[i * width + j].0;
                let tile = tiles[cur].borderless();
                for (k, row) in tile.into_iter().enumerate() {
                    let x = i * tile_length + k;
                    let y = image_width - j * tile_length - tile_length;
                    raw[x] |= row << y;
                }
            }
        }

        Image::from_vec(99999, raw, image_width)
    }

    fn from_vec(id: usize, v: Vec<u128>, width: usize) -> Self {
        let height = v.len();
        Image {
            id,
            raw: v,
            width,
            height,
        }
    }

    fn update_image(&mut self, row: u128) {
        self.raw.push(row);
    }

    fn edges(&self) -> [u128; 4] {
        let mut edges = [0; 4];
        edges[0] = self.raw[0];
        edges[1] = self.column(self.raw.len() - 1);

        edges[2] = *self.raw.last().unwrap();
        edges[3] = self.column(0);

        edges
    }

    fn flip_h(&mut self) {
        for row in &mut self.raw {
            *row = reverse(*row, self.width);
        }
    }

    fn rotate_right(&mut self) {
        self.raw = (0..self.width).map(|c| self.column(c)).collect();
        (self.height, self.width) = (self.width, self.height);
    }

    fn column(&self, column: usize) -> u128 {
        let mut r = 0;
        let mask = 1 << (self.width - 1 - column);
        for (i, row) in self.raw.iter().enumerate() {
            if row & mask != 0 {
                r |= 1 << i;
            }
        }
        r
    }

    fn borderless(&self) -> Vec<u128> {
        let mut raw = vec![];
        for i in 1..self.height - 1 {
            raw.push(without_head_and_tail(self.raw[i], self.width))
        }
        raw
    }

    #[allow(dead_code)]
    fn draw(&self) -> String {
        let mut s = String::new();
        for row in &self.raw {
            for i in 0..self.width {
                if row & (1 << (self.width - 1 - i)) != 0 {
                    s.push('#')
                } else {
                    s.push('.')
                }
            }
            s.push('\n')
        }

        s
    }

    fn count_one(&self) -> usize {
        self.raw.iter().map(|&r| count_one(r)).sum()
    }
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

fn without_head_and_tail(edge: u128, length: usize) -> u128 {
    (edge & (!(1 << (length - 1)))) >> 1
}

fn reverse(edge: u128, length: usize) -> u128 {
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

fn is_corner_tile(id: usize, edges: &[[u128; 4]], width: usize) -> bool {
    let l = edges.len();
    edges[id]
        .iter()
        .enumerate()
        .filter(|(_, &cur)| {
            (0..l)
                .filter(|j| *j != id)
                .all(|j| !possible_adjacent_tile(cur, &edges[j], width))
        })
        .count()
        == 2
}

fn possible_adjacent_tile(edge: u128, edges: &[u128; 4], width: usize) -> bool {
    edges
        .iter()
        .any(|&other| edge == other || edge == reverse(other, width))
}

fn parse_input(input: &str) -> Vec<Image> {
    let mut tiles = vec![];

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        } else if let Some(id) = line.trim().strip_prefix("Tile ") {
            if let Some(id) = id.trim().strip_suffix(':') {
                tiles.push(Image::new(id.trim().parse().unwrap(), 10, 10));
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
}
