use std::str::FromStr;

type CoordRange = [i32; 2];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Instruction {
    state: bool,
    xs: CoordRange,
    ys: CoordRange,
    zs: CoordRange,
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

impl Instruction {
    fn intersect(&self, other: &Self) -> Option<Instruction> {
        let min_x = self.xs[0].max(other.xs[0]);
        let max_x = self.xs[1].min(other.xs[1]);
        let min_y = self.ys[0].max(other.ys[0]);
        let max_y = self.ys[1].min(other.ys[1]);
        let min_z = self.zs[0].max(other.zs[0]);
        let max_z = self.zs[1].min(other.zs[1]);

        if min_x > max_x || min_y > max_y || min_z > max_z {
            None
        } else {
            Some(Instruction {
                state: !self.state,
                xs: [min_x, max_x],
                ys: [min_y, max_y],
                zs: [min_z, max_z],
            })
        }
    }
}

impl From<Instruction> for i64 {
    fn from(instruction: Instruction) -> Self {
        let [x0, x1] = instruction.xs;
        let [y0, y1] = instruction.ys;
        let [z0, z1] = instruction.zs;
        assert!(x0 <= x1);
        assert!(y0 <= y1);
        assert!(z0 <= z1);
        let neg = if instruction.state { 1 } else { -1 };
        neg * (x1 - x0 + 1) as i64 * (y1 - y0 + 1) as i64 * (z1 - z0 + 1) as i64
    }
}

fn gen_overlapped_instrs(instructions: &[Instruction]) -> Vec<Instruction> {
    let mut cubes: Vec<Instruction> = Vec::new();
    let mut merge: Vec<Instruction> = Vec::new();
    for instr in instructions {
        if instr.state {
            merge.push(*instr);
        }
        for c in cubes.iter() {
            if let Some(new) = c.intersect(instr) {
                merge.push(new);
            }
        }
        cubes.extend(merge.drain(..));
    }
    cubes
}

fn part1(instructions: &[Instruction]) -> i64 {
    let overlapped_instrs = gen_overlapped_instrs(instructions);
    overlapped_instrs.into_iter().map(i64::from).sum()
}

fn part2(instructions: &[Instruction]) -> i64 {
    let overlapped_instrs = gen_overlapped_instrs(instructions);
    overlapped_instrs.into_iter().map(i64::from).sum()
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
    println!("Part2: {}", part2(&input));
}
