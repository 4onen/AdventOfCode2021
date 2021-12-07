use std::str::FromStr;

type Coord = u16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    x: Coord,
    y: Coord,
}

impl FromStr for Point {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s
            .split_once(',')
            .map(|(a, b)| (a.parse::<Coord>(), b.parse::<Coord>()))
            .ok_or_else(|| "Insufficient input numbers")?;
        if let (Ok(a), Ok(b)) = v {
            Ok(Point { x: a, y: b })
        } else {
            Err("Bad parse or too many input numbers")
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Line {
    Horizontal(Coord, (Coord, Coord)),
    Vertical(Coord, (Coord, Coord)),
    Arbitrary(Point, Point),
}

impl FromStr for Line {
    type Err = &'static str;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let unpacked = line
            .split(" -> ")
            .map(|s| s.parse::<Point>())
            .collect::<Result<Vec<_>, _>>()?;
        if let [a, b] = unpacked[..] {
            if a.x == b.x {
                Ok(Line::Vertical(a.x, (a.y.min(b.y), b.y.max(a.y))))
            } else if a.y == b.y {
                Ok(Line::Horizontal(a.y, (a.x.min(b.x), b.x.max(a.x))))
            } else {
                Ok(Line::Arbitrary(
                    if a.x < b.x { a } else { b },
                    if a.x > b.x { a } else { b },
                ))
            }
        } else {
            Err("Bleh")
        }
    }
}

fn gridsize(input: &[Line]) -> (Coord, Coord) {
    let (x, y) = input
        .into_iter()
        .map(|l| match *l {
            Line::Horizontal(y, (_x1, x2)) => (x2, y),
            Line::Vertical(x, (_y1, y2)) => (x, y2),
            Line::Arbitrary(a, b) => (a.x.max(b.x), a.y.max(b.y)),
        })
        .fold((0, 0), |(x, y), (x2, y2)| (x.max(x2), y.max(y2)));

    (x + 1, y + 1)
}

fn part1(input: &[Line], maxx: Coord, maxy: Coord) -> usize {
    let mut grid = vec![vec![0u16; maxx as usize]; maxy as usize];

    let input_no_arbitrary = input.into_iter().filter(|l| match l {
        Line::Arbitrary(_, _) => false,
        _ => true,
    });

    for line in input_no_arbitrary {
        match *line {
            Line::Horizontal(y, (x1, x2)) => {
                for x in x1..=x2 {
                    grid[y as usize][x as usize] += 1;
                }
            }
            Line::Vertical(x, (y1, y2)) => {
                for y in y1..=y2 {
                    grid[y as usize][x as usize] += 1;
                }
            }
            _ => panic!("Arbitrary line not expected in part 1"),
        }
    }

    grid.into_iter().flatten().filter(|&x| x >= 2).count()
}

// 4728 correct

fn part2(input: &[Line], maxx: Coord, maxy: Coord) -> usize {
    let mut grid = vec![vec![0u16; maxx as usize]; maxy as usize];

    for &line in input {
        match line {
            Line::Horizontal(y, (x1, x2)) => {
                for x in x1..=x2 {
                    grid[y as usize][x as usize] += 1;
                }
            }
            Line::Vertical(x, (y1, y2)) => {
                for y in y1..=y2 {
                    grid[y as usize][x as usize] += 1;
                }
            }
            Line::Arbitrary(a, b) => {
                // x values are always in order a -> b.
                let mut y = a.y;
                for x in a.x..=b.x {
                    grid[y as usize][x as usize] += 1;
                    if a.y > b.y {
                        y -= 1;
                    } else {
                        y += 1;
                    }
                }
            }
        }
    }

    grid.into_iter().flatten().filter(|&x| x >= 2).count()
}

// 6729 too low (wasn't iterating any points that went right-to-left or upward)
// 11051 too low (wasn't iterating any points that went upward)
// 17717 correct

fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .map(|l| l.parse::<Line>())
        .collect::<Result<Vec<Line>, _>>()
        .unwrap();

    let (x, y) = gridsize(&input);

    println!("Part 1: {}", part1(&input, x, y));
    println!("Part 2: {}", part2(&input, x, y));
}
