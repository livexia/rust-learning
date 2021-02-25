use std::io::{self, Read, Write};
use std::collections::HashMap;

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()>{
    let mut freq = 0;

    for line in input.lines() {
        freq += line.parse::<i32>()?;
    }
    writeln!(io::stdout(), "{}", freq)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut freq = 0;
    let mut freq_hashmap: HashMap<i32, u8> = HashMap::new();
    
    loop {
        for line in input.lines() {
            freq += line.parse::<i32>()?;
            if freq_hashmap.contains_key(&freq) {
                writeln!(io::stdout(), "{}", freq)?;
                return Ok(());
            } 
            freq_hashmap.entry(freq).or_insert(1);
        }
    }
}
