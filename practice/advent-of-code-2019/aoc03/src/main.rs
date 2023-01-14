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
    let (path1, path2) = parse_input(&input)?;

    part1(&path1, &path2)?;
    part2(&path1, &path2)?;
    Ok(())
}

fn part1(path1: &[Path], path2: &[Path]) -> Result<i32> {
    let start = Instant::now();

    let mut result = i32::MAX;
    let mut path1 = path1.to_owned();
    let mut path2 = path2.to_owned();
    path1.sort_by_key(|p| p.min_dis);
    path2.sort_by_key(|p| p.min_dis);
    for p1 in &path1 {
        if p1.min_dis >= result {
            break;
        }
        for p2 in &path2 {
            if p2.min_dis >= result {
                break;
            }
            if let Some((x, y)) = p1.min_intersection(p2) {
                let r = x.abs() + y.abs();
                result = result.min(r);
            }
        }
    }

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(path1: &[Path], path2: &[Path]) -> Result<i32> {
    let start = Instant::now();

    let mut result = i32::MAX;

    let mut p1_len = 0;
    'outer: for p1 in path1 {
        let mut p2_len = 0;
        for p2 in path2 {
            if let Some(p) = p1.min_intersection(p2) {
                let len = p1_len + p2_len + dis(p1.start, p) + dis(p2.start, p);
                result = result.min(len);
                break 'outer;
            }
            p2_len += p2.len();
        }
        p1_len += p1.len();
    }

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

#[derive(Debug, Clone)]
struct Path {
    dir: Coord,
    start: Coord,
    min_dis: i32,
}

impl Path {
    fn new(start: Coord, d: char, step: i32) -> Self {
        let dir = match d {
            'U' => (-step, 0),
            'D' => (step, 0),
            'L' => (0, -step),
            'R' => (0, step),
            _ => unreachable!(),
        };
        Path {
            dir,
            start,
            min_dis: dis((0, 0), start).min(dis((0, 0), (start.0 + dir.0, start.1 + dir.1))),
        }
    }

    fn len(&self) -> i32 {
        self.dir.0.abs() + self.dir.1.abs()
    }

    fn end(&self) -> Coord {
        (self.start.0 + self.dir.0, self.start.1 + self.dir.1)
    }

    fn min_intersection(&self, other: &Path) -> Option<Coord> {
        let r1 = self.interval();
        let r2 = other.interval();
        if let Some(interval_x) = interval_intersection(r1.0, r2.0) {
            if let Some(interval_y) = interval_intersection(r1.1, r2.1) {
                let x = interval_x.0;
                let y = interval_y.0;
                if x != 0 && y != 0 {
                    return Some((x, y));
                }
            }
        }
        None
    }

    fn interval(&self) -> ((i32, i32), (i32, i32)) {
        let (x1, y1) = self.start;
        let (x2, y2) = self.end();
        ((x1.min(x2), x1.max(x2)), (y1.min(y2), y1.max(y2)))
    }
}

fn dis(l1: Coord, l2: Coord) -> i32 {
    (l1.0.abs_diff(l2.0) + l1.1.abs_diff(l2.1)) as i32
}

fn interval_intersection(mut r1: (i32, i32), mut r2: (i32, i32)) -> Option<(i32, i32)> {
    if r1.0 > r2.0 {
        std::mem::swap(&mut r1, &mut r2)
    }
    if r2.0 > r1.1 {
        None
    } else {
        Some((r2.0, r2.1.min(r1.1)))
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
    helper(input, 6, 30);

    let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
    U62,R66,U55,R34,D71,R55,D58,R83";
    helper(input, 159, 610);

    let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
    U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
    helper(input, 135, 410);

    fn helper(input: &str, r1: i32, r2: i32) {
        let (path1, path2) = parse_input(&input).unwrap();
        assert_eq!(part1(&path1, &path2).unwrap(), r1);
        assert_eq!(part2(&path1, &path2).unwrap(), r2);
    }
}
