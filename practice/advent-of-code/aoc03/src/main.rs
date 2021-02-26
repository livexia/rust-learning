use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut fabric = vec![vec![0; 1000]; 1000];
    for claim in input.lines() {
        let claim: Vec<_> =  claim.split(' ')
         .filter(|&x| x != "@")
         .collect();
         let id = claim[0].strip_prefix("#").unwrap();
         let start: Vec<usize> = claim[1].strip_suffix(':').unwrap().split(',')
         .map(|f| f.parse::<usize>().unwrap()).collect();
        let area: Vec<usize> = claim[2].split('x')
        .map(|f| f.parse::<usize>().unwrap()).collect();
        for i in start[0]..start[0]+area[0] {
            for j in start[1]..start[1]+area[1] {
                fabric[i][j] += 1;
            }
        }

    }

    let mut res = 0;
    fabric.iter().for_each(|f| f.into_iter().for_each(|&f| if f > 1 { res += 1} ));

    writeln!(io::stdout(), "{}", res)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut fabric = vec![vec![0; 1000]; 1000];
    let mut claims: Vec<(&str, usize, usize, usize, usize)> = Vec::new();
    for claim in input.lines() {
        let claim: Vec<_> =  claim.split(' ')
         .filter(|&x| x != "@")
         .collect();
        let id = claim[0].strip_prefix("#").unwrap();
        let start: Vec<usize> = claim[1].strip_suffix(':').unwrap().split(',')
         .map(|f| f.parse::<usize>().unwrap()).collect();
        let area: Vec<usize> = claim[2].split('x')
        .map(|f| f.parse::<usize>().unwrap()).collect();
        claims.push((id, start[0], start[1], area[0], area[1]));
        for i in start[0]..start[0]+area[0] {
            for j in start[1]..start[1]+area[1] {
                fabric[i][j] += 1;
            }
        }
    }
    'outer: for (id, x, y, width, height) in claims {
        for i in x..x+width {
            for j in y..y+height {
                if fabric[i][j] > 1 {
                    continue 'outer;
                }
            }
        }
        writeln!(io::stdout(), "{}", id)?;
        return Ok(());
    }
    Err(From::from("Can not find only claim that doesn't overlap"))
}

