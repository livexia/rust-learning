use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Int = i32;
type Coord = (Int, Int, Int);

// , v=<x,y,z => Coord
fn parse_coord(raw: &str) -> Result<Coord> {
    if let Some((_, raw)) = raw.trim().split_once('<') {
        let raw: Vec<_> = raw.split(',').collect();
        return Ok((
            raw[0].trim().parse()?,
            raw[1].trim().parse()?,
            raw[2].trim().parse()?,
        ));
    }
    err!("Not a valid raw coord: {raw}")
}

fn parse_input(input: &str) -> Result<Vec<(Coord, Coord, Coord)>> {
    let mut r = vec![];
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let raw: Vec<_> = line.trim().split('>').collect();
        r.push((
            parse_coord(raw[0])?,
            parse_coord(raw[1])?,
            parse_coord(raw[2])?,
        ));
    }
    Ok(r)
}

fn inc_v(v: Coord, a: Coord) -> Coord {
    (v.0 + a.0, v.1 + a.1, v.2 + a.2)
}

fn inc_p(p: Coord, v: Coord) -> Coord {
    (p.0 + v.0, p.1 + v.1, p.2 + v.2)
}

fn dis(c1: Coord, c2: Coord) -> Int {
    (c1.0.abs_diff(c2.0) + c1.1.abs_diff(c2.1) + c1.2.abs_diff(c2.2)) as Int
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let particles = parse_input(&input)?;

    part1(&particles)?;
    // part2()?;
    Ok(())
}

fn part1(particles: &[(Coord, Coord, Coord)]) -> Result<usize> {
    let start = Instant::now();

    let mut patricles = particles.to_owned();
    let mut min_dis_times = HashMap::new();

    let result;
    let mut total_count = 0;
    loop {
        total_count += 1;
        let mut min_dis = Int::MAX;
        let mut min_index = 0;
        for (i, particle) in patricles.iter_mut().enumerate() {
            particle.1 = inc_v(particle.1, particle.2);
            particle.0 = inc_p(particle.0, particle.1);
            let d = dis(particle.0, (0, 0, 0));
            if min_dis > d {
                min_dis = d;
                min_index = i;
            };
        }
        *min_dis_times.entry(min_index).or_insert(0) += 1;
        let (&index, &count) = min_dis_times.iter().max_by_key(|(_, a)| *a).unwrap();
        if count > total_count - count && total_count > 500 {
            result = index;
            break;
        }
    }

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

#[test]
fn example_input() {
    let input = "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
        p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>";
    assert_eq!(part1(&parse_input(input).unwrap()).unwrap(), 0);
}
