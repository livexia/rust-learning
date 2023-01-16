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
    let image = parse_input(&input, 25, 6);

    part1(&image)?;
    // part2()?;
    Ok(())
}

fn part1(image: &Image) -> Result<usize> {
    let start = Instant::now();

    let layer = (0..image.raw.len())
        .map(|i| (image.layer_zero_count(i), i))
        .min()
        .unwrap()
        .1;
    let result = image.layer_one_count(layer) * image.layer_two_count(layer);

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

struct Image {
    raw: Vec<Vec<u8>>,
    tall: usize,
    wide: usize,
}

impl Image {
    fn new(input: &[u8], wide: usize, tall: usize) -> Self {
        let mut raw = vec![];

        for l in 0..input.len() / (tall * wide) {
            let mut layer = vec![0; tall * wide];
            for i in 0..tall {
                for j in 0..wide {
                    layer[i * wide + j] = input[l * wide * tall + i * wide + j];
                }
            }
            raw.push(layer)
        }
        Self { raw, tall, wide }
    }

    fn layer_zero_count(&self, layer: usize) -> usize {
        self.raw[layer].iter().filter(|&&b| b == 0).count()
    }

    fn layer_one_count(&self, layer: usize) -> usize {
        self.raw[layer].iter().filter(|&&b| b == 1).count()
    }

    fn layer_two_count(&self, layer: usize) -> usize {
        self.raw[layer].iter().filter(|&&b| b == 2).count()
    }
}

fn parse_input(input: &str, wide: usize, tall: usize) -> Image {
    Image::new(
        &input.trim().bytes().map(|b| b - b'0').collect::<Vec<u8>>(),
        wide,
        tall,
    )
}

#[test]
fn example_input() {
    let input = "123456789012";
    let image = parse_input(input, 3, 2);
    assert_eq!(
        image.raw,
        vec![vec![1, 2, 3, 4, 5, 6], vec![7, 8, 9, 0, 1, 2]]
    );
}
