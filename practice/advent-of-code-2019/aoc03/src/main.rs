use std::collections::HashSet;
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
    let (path1, path2) = parse_input(&input)?;

    part1(&path1, &path2)?;
    // part2()?;
    Ok(())
}

fn part1(path1: &[Path], path2: &[Path]) -> Result<i32> {
    let start = Instant::now();

    let mut intersection = HashSet::new();
    for p1 in path1 {
        for p2 in path2 {
            if !p1.is_parallel(p2) {
                intersection.extend(p1.intersection(p2));
            }
        }
    }
    println!("{:?}", intersection.len());
    intersection.remove(&(0, 0));
    let result = intersection
        .iter()
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

struct Path {
    dir: (i32, i32),
    start: (i32, i32),
}

impl Path {
    fn new(start: (i32, i32), d: char, step: i32) -> Self {
        let dir = match d {
            'U' => (-step, 0),
            'D' => (step, 0),
            'L' => (0, -step),
            'R' => (0, step),
            _ => unreachable!(),
        };
        Path { dir, start }
    }

    fn end(&self) -> (i32, i32) {
        (self.start.0 + self.dir.0, self.start.1 + self.dir.1)
    }

    fn locations(&self) -> HashSet<(i32, i32)> {
        let mut r = HashSet::new();
        let (x, y) = self.start;
        let dir = self.dir;
        let (start_x, end_x) = if dir.0 < 0 { (dir.0, 0) } else { (0, dir.0) };
        let (start_y, end_y) = if dir.1 < 0 { (dir.1, 0) } else { (0, dir.1) };
        for dx in start_x..=end_x {
            for dy in start_y..=end_y {
                r.insert((x + dx, y + dy));
            }
        }
        r
    }

    fn is_parallel(&self, other: &Path) -> bool {
        (self.dir.0 == 0 && other.dir.0 == 0) || (self.dir.1 == 0 && other.dir.1 == 0)
    }

    fn intersection(&self, other: &Path) -> HashSet<(i32, i32)> {
        let p1 = self.locations();
        let p2 = other.locations();
        p1.intersection(&p2).cloned().collect()
    }
}

fn parse_input(input: &str) -> Result<(Vec<Path>, Vec<Path>)> {
    let lines: Vec<_> = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    Ok((parse_input_line(lines[0])?, parse_input_line(lines[1])?))
}

fn parse_input_line(line: &str) -> Result<Vec<Path>> {
    let mut r = vec![];
    let mut start = (0, 0);
    for word in line.trim().split(',') {
        let (d, step) = word.split_at(1);
        let d = d.chars().next().unwrap();
        let step = step.parse()?;
        let p = Path::new(start, d, step);
        start = p.end();
        r.push(p);
    }

    Ok(r)
}

#[test]
fn example_input() {
    let input = "R8,U5,L5,D3\nU7,R6,D4,L4";
    helper(input, 6, 0);

    let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
    U62,R66,U55,R34,D71,R55,D58,R83";
    helper(input, 159, 0);

    let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
    U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
    helper(input, 135, 0);

    fn helper(input: &str, r1: i32, r2: i32) {
        let (path1, path2) = parse_input(&input).unwrap();
        assert_eq!(part1(&path1, &path2).unwrap(), r1);
        // assert_eq!(part2(&path1, &path2), r2);
    }
}
