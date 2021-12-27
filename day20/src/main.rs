use std::{fmt::Display, ops::Index, str::FromStr};

struct Image {
    width: usize,
    pixels: Vec<bool>,
    plane: bool,
}

impl Index<(isize, isize)> for Image {
    type Output = bool;

    fn index(&self, (x, y): (isize, isize)) -> &bool {
        if x < 0 || y < 0 || x >= self.width as isize || y >= self.width as isize {
            &self.plane
        } else {
            &self.pixels[(y as usize) * self.width + (x as usize)]
        }
    }
}

impl Image {
    fn encode_pixel(&self, x: isize, y: isize) -> usize {
        [
            self[(x - 1, y - 1)],
            self[(x, y - 1)],
            self[(x + 1, y - 1)],
            self[(x - 1, y)],
            self[(x, y)],
            self[(x + 1, y)],
            self[(x - 1, y + 1)],
            self[(x, y + 1)],
            self[(x + 1, y + 1)],
        ]
        .into_iter()
        .fold(0, |acc, b| acc << 1 | b as usize)
    }

    fn enhance(&self, alg: &[bool]) -> Image {
        let mut pixels = Vec::new();
        pixels.resize((self.width + 2) * (self.width + 2), false);
        for y in -1..=self.width as isize {
            for x in -1..=self.width as isize {
                let alg_pos = self.encode_pixel(x, y);
                let alg_val = alg[alg_pos];
                pixels[(y + 1) as usize * (self.width + 2) + (x + 1) as usize] = alg_val;
            }
        }
        assert_eq!(pixels.len(), (self.width + 2) * (self.width + 2));
        Image {
            width: self.width + 2,
            pixels,
            plane: if self.plane { alg[0b111111111] } else { alg[0] },
        }
    }

    fn lit(&self) -> usize {
        if self.plane {
            usize::MAX
        } else {
            self.pixels.iter().map(|&b| b as usize).sum()
        }
    }
}

impl FromStr for Image {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s
            .bytes()
            .position(|b| b == b'\n')
            .ok_or("No newline found. Image must have multiple lines.")?;
        let pixels = s
            .lines()
            .flat_map(|line| line.bytes().map(|c| c == b'#'))
            .collect();
        Ok(Image {
            width,
            pixels,
            plane: false,
        })
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in -1..(self.width as isize + 1) {
            for x in -1..(self.width as isize + 1) {
                write!(f, "{}", if self[(x, y)] { '#' } else { '.' })?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn part1(alg: &[bool], image: &Image) -> usize {
    let enhanced = image.enhance(alg);
    let enhanced2 = enhanced.enhance(alg);
    enhanced2.lit()
}

// 5400 correct

fn part2(alg: &[bool], image: &Image) -> usize {
    let mut enhanced = image.enhance(alg);
    for _ in 1..50 {
        enhanced = enhanced.enhance(alg);
    }
    enhanced.lit()
}

// 18989 correct

fn main() {
    // Get filename from command line
    let filename = std::env::args().nth(1).expect("No filename given");
    // Read file
    let (alg, image) = {
        let filestr = std::fs::read_to_string(filename).expect("Could not read file");
        let mut linesiter = filestr.lines();
        let alg: Vec<bool> = linesiter
            .next()
            .expect("No algorithm found")
            .bytes()
            .map(|c| c == b'#')
            .collect();

        assert_eq!(alg.len(), 1 << 9);

        let mut peekable = linesiter.skip(1).peekable();
        let width: usize = peekable.peek().expect("No image found.").len();
        let pixels: Vec<bool> = peekable
            .flat_map(|line| line.bytes().map(|c| c == b'#'))
            .collect();

        assert_eq!(width, pixels.len() / width);

        (
            alg,
            Image {
                width,
                pixels,
                plane: false,
            },
        )
    };

    println!("Part 1: {}", part1(&alg, &image));
    println!("Part 2: {}", part2(&alg, &image));
}
