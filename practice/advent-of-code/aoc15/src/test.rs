use std::io::{self, Read, Write};
use std::error::Error;
use std::result;
use crate::Caves;
use crate::Coordinate;

type Result<T> = result::Result<T, Box<dyn Error>>;

#[test]
fn dstance_map_test() -> Result<()> {
    
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let caves: Caves = input.parse()?;
    let c = Coordinate { x: 4, y: 4 };
    caves.distance_map(&c);
    Ok(())
}