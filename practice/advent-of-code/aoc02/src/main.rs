use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
} 

fn part1(input: &str) -> Result<()> {
    let mut frequencies = [0u8; 256];
    let mut two_times= 0;
    let mut three_times = 0;

    for line in input.lines() {
        if !line.is_ascii() {
            return Err(From::from("part1 only supports ASCII"));
        }

        for f in frequencies.iter_mut() {
            *f = 0;
        }
        for b in line.as_bytes().iter().map(|&b| b as usize) {
            frequencies[b] = frequencies[b].saturating_add(1);
        }
        if frequencies.iter().any(|&f| f == 2) {
            two_times += 1;
        }
        if frequencies.iter().any(|&f| f == 3) {
            three_times += 1;
        }
    }

    writeln!(io::stdout(), "{}", two_times * three_times)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let ids: Vec<_> = input.lines().collect();
    let n = ids.len();
    for i in 0..n {
        for j in i+1..n {
            if let Some(common) = common_correct_letters(&ids[i], &ids[j]) {
                writeln!(io::stdout(), "{}", common)?;
                return Ok(());
            }
        }
    }
    Err(From::from("Could not find two correct box ids"))
}

fn common_correct_letters(id1: &str, id2: &str) -> Option<String> {
    if id1.len() != id2.len() {
        return None;
    }

    let mut found_one_differ = false;
    for (c1, c2) in id1.chars().zip(id2.chars()) {
        if c1 != c2 {
            if found_one_differ {
                return  None;
            }
            found_one_differ = true;
        }
    }
    Some (
        id1.chars().zip(id2.chars())
         .filter(|&(c1, c2)| c1 == c2)
         .map(|(c, _)| c)
         .collect()
    )
}
