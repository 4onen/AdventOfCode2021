use std::{collections::HashSet, str::FromStr};

type Cube = [i32; 3];
type CoordRange = [i32; 2];
type Coll = HashSet<Cube>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Instruction {
    state: bool,
    xs: [i32; 2],
    ys: [i32; 2],
    zs: [i32; 2],
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Minimal example:
        // on x=0..2,y=0..2,z=0..2
        if s.len() < 23 {
            return Err(format!("\"{}\" is too short!", s));
        }
        let state = s.starts_with("on");
        let rest = s[(if state { 2 } else { 3 }..)].trim();
        let coordranges: Vec<CoordRange> = rest
            .split(',')
            .map(|s| {
                if let Some((start, end)) = s.split_once("..") {
                    let start = i32::from_str(&start[2..]).map_err(|e| format!("{}: {}", s, e))?;
                    let end = i32::from_str(end).map_err(|e| format!("{}: {}", s, e))?;
                    if start > end {
                        Err(format!("{}: start > end", s))
                    } else {
                        Ok([start, end])
                    }
                } else {
                    Err(format!("{}: expected '..'", s))
                }
            })
            .collect::<Result<Vec<CoordRange>, _>>()?;
        if coordranges.len() < 3 {
            Err(format!("\"{}\" has too few coordinates!", s))
        } else if coordranges.len() > 3 {
            Err(format!("\"{}\" has too many coordinates!", s))
        } else {
            Ok(Self {
                state,
                xs: coordranges[0],
                ys: coordranges[1],
                zs: coordranges[2],
            })
        }
    }
}

fn part1(instructions: &[Instruction]) -> usize {
    let mut coll = Coll::new();
    for instruction in instructions {
        let [x0, x1] = instruction.xs;
        let [y0, y1] = instruction.ys;
        let [z0, z1] = instruction.zs;

        if instruction.state {
            coll.extend((x0..=x1).into_iter().flat_map(|x| {
                (y0..=y1).flat_map(move |y| (z0..=z1).map(move |z: i32| -> Cube { [x, y, z] }))
            }));
        } else {
            for x in x0..=x1 {
                for y in y0..=y1 {
                    for z in z0..=z1 {
                        coll.remove(&[x, y, z]);
                    }
                }
            }
        }
    }
    coll.len()
}

fn main() {
    // Get filename from command line argument
    let filename = std::env::args().nth(1).expect("Missing filename");
    // Read file
    let input = std::fs::read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Instruction>, _>>()
        .expect("Failed to parse file");

    println!("Part1: {}", part1(&input[..20]));
}
