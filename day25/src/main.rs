use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum CucumberCell {
    Empty,
    South,
    East,
}

impl CucumberCell {
    fn is_south(self) -> bool {
        match self {
            CucumberCell::South => true,
            _ => false,
        }
    }

    fn is_east(self) -> bool {
        match self {
            CucumberCell::East => true,
            _ => false,
        }
    }

    fn is_empty(self) -> bool {
        match self {
            CucumberCell::Empty => true,
            _ => false,
        }
    }
}

impl TryFrom<u8> for CucumberCell {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'.' => Ok(CucumberCell::Empty),
            b'v' => Ok(CucumberCell::South),
            b'>' => Ok(CucumberCell::East),
            _ => Err(format!("Unknown cell: '{}'", value as char)),
        }
    }
}

impl Display for CucumberCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CucumberCell::Empty => write!(f, "."),
            CucumberCell::South => write!(f, "v"),
            CucumberCell::East => write!(f, ">"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Map {
    width: usize,
    cells: Vec<CucumberCell>,
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.lines().peekable();
        let width = iter.peek().map(|line| line.len()).ok_or("Empty map?")?;
        let cells = iter
            .flat_map(|line| line.bytes().map(|c| CucumberCell::try_from(c)))
            .collect::<Result<Vec<CucumberCell>, Self::Err>>()?;
        Ok(Map { width, cells })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for (i, cell) in self.cells.iter().enumerate() {
            if i % self.width == 0 {
                writeln!(f)?;
            }
            write!(f, "{}", cell)?;
        }
        writeln!(f)
    }
}

impl Map {
    fn step(&self) -> (bool, Map) {
        let mut moved = false;
        let mut eastmoved_cells = self.cells.clone();

        for i in (0..self.cells.len()).filter(|&i| self.cells[i].is_east()) {
            let x = i % self.width;
            let linestart = i - x;
            let newpos = linestart + (x + 1) % self.width;
            if self.cells[newpos].is_empty() {
                eastmoved_cells.swap(newpos, i);
                moved = true;
            }
        }

        let mut southmoved_cells = eastmoved_cells.clone();

        for i in (0..eastmoved_cells.len()).filter(|&i| eastmoved_cells[i].is_south()) {
            let newpos = (i + self.width) % eastmoved_cells.len();
            if eastmoved_cells[newpos].is_empty() {
                southmoved_cells.swap(newpos, i);
                moved = true;
            }
        }

        (
            moved,
            Map {
                width: self.width,
                cells: southmoved_cells,
            },
        )
    }
}

fn part1(mut map: Map) -> usize {
    let mut count = 1;
    while let (true, newmap) = map.step() {
        map = newmap;
        count += 1;
    }
    count
}

// 292 too low (flipped east and south cucumbers)
// 378 correct

fn main() {
    let input: Map = include_str!("../input.txt").parse().unwrap();
    println!("Part 1: {}", part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input: Map = include_str!("../example1.txt").parse().unwrap();
        assert_eq!(part1(input), 58);
    }

    #[test]
    fn test_part1_by_steps() {
        let input: Map = include_str!("../example1.txt").parse().unwrap();
        assert_eq!(input.width, 10);
        assert_eq!(input.cells.len(), 90);
        let onestep: Map = include_str!("../example1_step1.txt").parse().unwrap();
        let (moved, newmap) = input.step();
        assert!(moved, "Example should move on step 1");
        assert_eq!(
            newmap, onestep,
            "Maps did not match: {} {}",
            newmap, onestep
        );
    }
}
