use hashbrown::{HashMap, HashSet};
use std::collections::VecDeque;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Coord = (usize, usize);
type ShortestPathMatrix = HashMap<Coord, HashSet<(Coord, (usize, u32, u32))>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut grid = parse_input(&input);

    part1(&grid)?;

    let entrance = find_entrances(&grid)[0];
    update_map(&mut grid, entrance);
    part2(&grid)?;

    Ok(())
}

fn part1(grid: &[Vec<char>]) -> Result<usize> {
    let start = Instant::now();

    let entrances = find_entrances(grid);
    let keys = find_keys(grid);
    let shortest_paths = build_shortest_path_matrix(grid, &entrances, &keys);

    let complete_keys = get_complete_keys(&entrances, &shortest_paths, grid);

    let prepare = start.elapsed();
    let start = Instant::now();

    let result = dfs(
        grid,
        &shortest_paths,
        coords_to_hash(&entrances),
        1,
        0,
        complete_keys,
        &mut HashMap::new(),
    )
    .unwrap();

    let dfs_time = start.elapsed();
    let start = Instant::now();

    assert_eq!(
        result,
        bfs(grid, &shortest_paths, &entrances, complete_keys).unwrap()
    );

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(
        io::stdout(),
        "> Time elapsed is: prepare: {:?}, dfs: {:?}, bfs: {:?}",
        prepare,
        dfs_time,
        start.elapsed()
    )?;
    Ok(result)
}

fn part2(grid: &[Vec<char>]) -> Result<usize> {
    let start = Instant::now();

    let entrances = find_entrances(grid);
    let keys = find_keys(grid);
    let shortest_paths = build_shortest_path_matrix(grid, &entrances, &keys);
    let complete_keys = get_complete_keys(&entrances, &shortest_paths, grid);

    let prepare = start.elapsed();
    let start = Instant::now();

    let result = dfs(
        grid,
        &shortest_paths,
        coords_to_hash(&entrances),
        entrances.len(),
        0,
        complete_keys,
        &mut HashMap::new(),
    )
    .unwrap();

    let dfs_time = start.elapsed();
    let start = Instant::now();

    assert_eq!(
        bfs(grid, &shortest_paths, &entrances, complete_keys,).unwrap(),
        result
    );

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(
        io::stdout(),
        "> Time elapsed is: prepare: {:?}, dfs: {:?}, bfs: {:?}",
        prepare,
        dfs_time,
        start.elapsed()
    )?;
    Ok(result)
}

fn bfs(
    grid: &[Vec<char>],
    shortest_paths: &ShortestPathMatrix,
    src: &[Coord],
    complete_keys: u32,
) -> Option<usize> {
    let mut result = usize::MAX;
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    queue.push_back((coords_to_hash(src), 0, 0));
    while let Some((coords, owned_keys, dis)) = queue.pop_front() {
        if dis >= result {
            continue;
        }
        if let Some(&prev) = visited.get(&(coords, owned_keys)) {
            if prev <= dis {
                continue;
            }
        }
        visited.insert((coords, owned_keys), dis);

        if owned_keys == complete_keys {
            result = result.min(dis);
            continue;
        }
        for i in 0..src.len() {
            let cur = get_coord_from_hash(coords, i);
            for &(next, (distance, found_keys, required_keys)) in shortest_paths.get(&cur).unwrap()
            {
                let kind = grid[next.0][next.1];
                if kind == '@' || key_hash(kind) & owned_keys != 0 {
                    continue;
                }
                if owned_keys | required_keys > owned_keys {
                    continue;
                }
                queue.push_back((
                    update_hash_with_coord(coords, i, next),
                    owned_keys | key_hash(kind) | found_keys,
                    dis + distance,
                ));
            }
        }
    }

    if result == usize::MAX {
        None
    } else {
        Some(result)
    }
}

fn coords_to_hash(coords: &[Coord]) -> u64 {
    assert!(coords.len() <= 4);
    let mut h = 0;
    for (i, &(x, y)) in coords.iter().enumerate() {
        h |= ((x as u64) << 8 | y as u64) << (i * 16)
    }
    h
}

fn get_coord_from_hash(h: u64, i: usize) -> Coord {
    let h = (h >> (i * 16)) & 0xffff;
    ((h >> 8) as usize, (h & 0xff) as usize)
}

fn update_hash_with_coord(h: u64, i: usize, coord: Coord) -> u64 {
    let (x, y) = coord;
    (h & !(0xffff << (i * 16))) | ((x as u64) << 8 | y as u64) << (i * 16)
}

fn dfs(
    grid: &[Vec<char>],
    shortest_paths: &ShortestPathMatrix,
    coords: u64,
    length: usize,
    owned_keys: u32,
    complete_keys: u32,
    cache: &mut HashMap<(u64, u32), Option<usize>>,
) -> Option<usize> {
    if let Some(&r) = cache.get(&(coords, owned_keys)) {
        return r;
    }
    if owned_keys == complete_keys {
        return Some(0);
    }
    let mut result = usize::MAX;
    for i in 0..length {
        let cur = get_coord_from_hash(coords, i);
        for &(next, (distance, found_keys, required_keys)) in shortest_paths.get(&cur).unwrap() {
            let kind = grid[next.0][next.1];
            if kind == '@' || key_hash(kind) & owned_keys != 0 {
                continue;
            }
            if owned_keys | required_keys > owned_keys {
                continue;
            }
            if let Some(d) = dfs(
                grid,
                shortest_paths,
                update_hash_with_coord(coords, i, next),
                length,
                owned_keys | key_hash(kind) | found_keys,
                complete_keys,
                cache,
            ) {
                result = result.min(distance + d);
            } else {
                unreachable!();
            }
        }
    }
    let result = if result == usize::MAX {
        None
    } else {
        Some(result)
    };
    cache.insert((coords, owned_keys), result);
    result
}

fn get_complete_keys(
    entrances: &[Coord],
    shortest_paths: &ShortestPathMatrix,
    grid: &[Vec<char>],
) -> u32 {
    let mut r = 0;
    for e in entrances {
        if let Some(v) = shortest_paths.get(e) {
            r |= v
                .iter()
                .fold(0, |s, ((x, y), _)| s | key_hash(grid[*x][*y]))
        } else {
            unreachable!()
        }
    }

    r
}

fn build_shortest_path_matrix(
    grid: &[Vec<char>],
    entrances: &[Coord],
    keys: &[Coord],
) -> ShortestPathMatrix {
    let mut shortest_paths = ShortestPathMatrix::new();
    let mut found = HashSet::new();
    for (i, &a) in keys.iter().chain(entrances.iter()).enumerate() {
        for &b in keys.iter().chain(entrances.iter()).skip(i + 1) {
            if found.insert((a, b)) || found.insert((b, a)) {
                if let Some(distance) =
                    shortest_path_bfs(grid, a, b, &mut shortest_paths, &mut found)
                {
                    shortest_paths.entry(a).or_default().insert((b, distance));
                    shortest_paths.entry(b).or_default().insert((a, distance));
                }
            }
        }
    }
    shortest_paths
}

// https://www.reddit.com/r/adventofcode/comments/ec8090/comment/fba6uh7
fn shortest_path_bfs(
    grid: &[Vec<char>],
    src: Coord,
    dest: Coord,
    shortest_paths: &mut ShortestPathMatrix,
    found: &mut HashSet<(Coord, Coord)>,
) -> Option<(usize, u32, u32)> {
    let height = grid.len();
    let width = grid[0].len();

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((src, 0, 0, 0));
    while let Some(((x, y), depth, mut keys, mut doors)) = queue.pop_front() {
        if visited.insert((x, y)) {
            let kind = grid[x][y];
            if is_door(kind) {
                doors |= 1 << (kind as u8 - b'A')
            }
            if is_key(kind) {
                if found.insert((src, (x, y))) || found.insert(((x, y), src)) {
                    shortest_paths
                        .entry(src)
                        .or_default()
                        .insert(((x, y), (depth, keys, doors)));
                    shortest_paths
                        .entry((x, y))
                        .or_default()
                        .insert((src, (depth, keys, doors)));
                }
                keys |= key_hash(kind)
            }
            if (x, y) == dest {
                return Some((depth, keys, doors));
            }
            for next in [
                (x.saturating_sub(1), y),
                (x + 1, y),
                (x, y.saturating_sub(1)),
                (x, y + 1),
            ] {
                if valid_coord(next.0, next.1, height, width) && grid[next.0][next.1] != '#' {
                    queue.push_back((next, depth + 1, keys, doors));
                }
            }
        }
    }
    None
}

fn update_map(grid: &mut [Vec<char>], entrance: Coord) {
    let (x, y) = entrance;
    grid[x][y] = '#';
    grid[x - 1][y] = '#';
    grid[x][y - 1] = '#';
    grid[x + 1][y] = '#';
    grid[x][y + 1] = '#';
    grid[x - 1][y - 1] = '@';
    grid[x - 1][y + 1] = '@';
    grid[x + 1][y - 1] = '@';
    grid[x + 1][y + 1] = '@';
}

fn key_hash(key: char) -> u32 {
    1 << (key as u8 - b'a')
}

fn find_keys(grid: &[Vec<char>]) -> Vec<Coord> {
    let mut keys = Vec::new();

    for x in 1..grid.len() - 1 {
        for y in 1..grid[0].len() - 1 {
            if is_key(grid[x][y]) {
                keys.push((x, y));
            }
        }
    }

    keys
}

fn find_entrances(grid: &[Vec<char>]) -> Vec<Coord> {
    let mut r = vec![];
    for x in 1..grid.len() - 1 {
        for y in 1..grid[0].len() - 1 {
            if grid[x][y] == '@' {
                r.push((x, y));
            }
        }
    }
    r
}

fn valid_coord(x: usize, y: usize, height: usize, width: usize) -> bool {
    x > 0 && y > 0 && x < height - 1 && y < width - 1
}

fn is_door(c: char) -> bool {
    ('A'..='Z').contains(&c)
}

fn is_key(c: char) -> bool {
    ('a'..='z').contains(&c)
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.trim().chars().collect()).collect()
}

#[test]
fn example_input() {
    let input = "#########
    #b.A.@.a#
    #########";
    let grid = parse_input(input);
    assert_eq!(part1(&grid).unwrap(), 8);

    let input = "########################
    #f.D.E.e.C.b.A.@.a.B.c.#
    ######################.#
    #d.....................#
    ########################";
    let grid = parse_input(input);
    assert_eq!(part1(&grid).unwrap(), 86);

    let input = "########################
    #...............b.C.D.f#
    #.######################
    #.....@.a.B.c.d.A.e.F.g#
    ########################";
    let grid = parse_input(input);
    assert_eq!(part1(&grid).unwrap(), 132);

    let input = "#################
    #i.G..c...e..H.p#
    ########.########
    #j.A..b...f..D.o#
    ########@########
    #k.E..a...g..B.n#
    ########.########
    #l.F..d...h..C.m#
    #################";
    let grid = parse_input(input);
    assert_eq!(part1(&grid).unwrap(), 136);

    let input = "########################
    #@..............ac.GI.b#
    ###d#e#f################
    ###A#B#C################
    ###g#h#i################
    ########################";
    let grid = parse_input(input);
    assert_eq!(part1(&grid).unwrap(), 81);

    // part2
    let input = "#######
    #a.#Cd#
    ##@#@##
    #######
    ##@#@##
    #cB#Ab#
    #######";
    let grid = parse_input(input);
    assert_eq!(part2(&grid).unwrap(), 8);

    let input = "###############
    #d.ABC.#.....a#
    ######@#@######
    ###############
    ######@#@######
    #b.....#.....c#
    ###############";
    let grid = parse_input(input);
    assert_eq!(part2(&grid).unwrap(), 24);

    let input = "#############
    #DcBa.#.GhKl#
    #.###@#@#I###
    #e#d#####j#k#
    ###C#@#@###J#
    #fEbA.#.FgHi#
    #############";
    let grid = parse_input(input);
    assert_eq!(part2(&grid).unwrap(), 32);

    let input = "#############
    #g#f.D#..h#l#
    #F###e#E###.#
    #dCba@#@BcIJ#
    #############
    #nK.L@#@G...#
    #M###N#H###.#
    #o#m..#i#jk.#
    #############";
    let grid = parse_input(input);
    assert_eq!(part2(&grid).unwrap(), 72);
    // failed because of https://www.reddit.com/r/adventofcode/comments/ec8090/comment/fbbuh42
    // so this solution won't work on all inputs
}
