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
    part2(&image)?;
    Ok(())
}

fn part1(image: &Image) -> Result<usize> {
    let start = Instant::now();

    let (_, one, two) = (0..image.raw.len())
        .map(|i| image.layer_pixel_count(i))
        .min()
        .unwrap();

    let result = one * two;

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(image: &Image) -> Result<String> {
    let start = Instant::now();

    let rendered_image = image.render();
    let result = drew(rendered_image);

    writeln!(io::stdout(), "Part 2: \n{result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

struct Image {
    raw: Vec<Vec<u8>>,
    tall: usize,
    wide: usize,
}

impl Image {
    fn new(input: &str, wide: usize, tall: usize) -> Self {
        let mut input = input.trim().bytes().map(|b| b - b'0');
        let size = tall * wide;
        let mut raw = Vec::with_capacity(input.len() / size);
        for _ in 0..input.len() / size {
            let mut layer = Vec::with_capacity(size);
            for _ in 0..size {
                layer.push(input.next().unwrap());
            }
            raw.push(layer)
        }
        Self { raw, tall, wide }
    }

    fn render(&self) -> Vec<Vec<u8>> {
        let mut result = vec![vec![0; self.wide]; self.tall];
        for (i, row) in result.iter_mut().enumerate() {
            for (j, pixel) in row.iter_mut().enumerate() {
                *pixel = self.redner_pixel(i * self.wide + j);
            }
        }
        result
    }

    fn redner_pixel(&self, p: usize) -> u8 {
        let layer = self.raw.iter().map(|l| l[p]);
        for pixel in layer {
            match pixel {
                0 => return 0,
                1 => return 1,
                2 => continue,
                _ => unreachable!(),
            }
        }
        unreachable!()
    }

    fn layer_pixel_count(&self, layer: usize) -> (usize, usize, usize) {
        self.raw[layer]
            .iter()
            .fold((0, 0, 0), |(zero, one, two), &b| {
                if b == 0 {
                    (zero + 1, one, two)
                } else if b == 1 {
                    (zero, one + 1, two)
                } else if b == 2 {
                    (zero, one, two + 1)
                } else {
                    (zero, one, two)
                }
            })
    }
}

fn drew(image: Vec<Vec<u8>>) -> String {
    let mut s = String::new();
    for row in image {
        for pixel in row {
            if pixel == 1 {
                s.push('#');
            } else if pixel == 0 {
                s.push(' ')
            } else {
                unreachable!()
            }
        }
        s.push('\n');
    }
    s
}

fn parse_input(input: &str, wide: usize, tall: usize) -> Image {
    Image::new(input, wide, tall)
}

#[test]
fn example_input() {
    let input = "123456789012";
    let image = parse_input(input, 3, 2);
    assert_eq!(
        image.raw,
        vec![vec![1, 2, 3, 4, 5, 6], vec![7, 8, 9, 0, 1, 2]]
    );

    let input = "0222112222120000";
    let image = parse_input(input, 2, 2);
    assert_eq!(image.render(), vec![vec![0, 1], vec![1, 0]])
}
