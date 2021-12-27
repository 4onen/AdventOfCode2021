use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    num::ParseIntError,
    ops::{Add, AddAssign, Index, Mul, Sub},
    str::FromStr,
};

type Coord = i32;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point3D {
    x: Coord,
    y: Coord,
    z: Coord,
}

impl Index<usize> for Point3D {
    type Output = Coord;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl Display for Point3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add for Point3D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a> Add<&'a Point3D> for Point3D {
    type Output = Self;

    fn add(self, other: &'a Point3D) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point3D {
    type Output = Point3D;

    fn sub(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<'a> Sub<&'a Point3D> for Point3D {
    type Output = <Self as Sub<Self>>::Output;

    fn sub(self, other: &'a Point3D) -> Self::Output {
        self - *other
    }
}

impl Mul for Point3D {
    type Output = Point3D;

    fn mul(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<'a> Mul<&'a Point3D> for Point3D {
    type Output = <Self as Mul<Point3D>>::Output;

    fn mul(self, other: &'_ Point3D) -> Self::Output {
        self * *other
    }
}

impl Point3D {
    const fn new(x: Coord, y: Coord, z: Coord) -> Point3D {
        Point3D { x, y, z }
    }

    fn manhattan_distance(&self) -> Coord {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

type Rotation = [(usize, Coord); 3];

const ROTATIONS: [Rotation; 24] = {
    const X: usize = 0;
    const Y: usize = 1;
    const Z: usize = 2;
    const POS: Coord = 1;
    const NEG: Coord = -1;
    [
        [(X, POS), (Y, POS), (Z, POS)],
        [(X, POS), (Z, POS), (Y, NEG)],
        [(X, POS), (Y, NEG), (Z, NEG)],
        [(X, POS), (Z, NEG), (Y, POS)],
        [(X, NEG), (Y, POS), (Z, NEG)],
        [(X, NEG), (Z, NEG), (Y, NEG)],
        [(X, NEG), (Y, NEG), (Z, POS)],
        [(X, NEG), (Z, POS), (Y, POS)],
        [(Y, POS), (X, POS), (Z, NEG)],
        [(Y, POS), (Z, NEG), (X, NEG)],
        [(Y, POS), (X, NEG), (Z, POS)],
        [(Y, POS), (Z, POS), (X, POS)],
        [(Y, NEG), (X, POS), (Z, POS)],
        [(Y, NEG), (Z, POS), (X, NEG)],
        [(Y, NEG), (X, NEG), (Z, NEG)],
        [(Y, NEG), (Z, NEG), (X, POS)],
        [(Z, POS), (X, POS), (Y, POS)],
        [(Z, POS), (Y, POS), (X, NEG)],
        [(Z, POS), (X, NEG), (Y, NEG)],
        [(Z, POS), (Y, NEG), (X, POS)],
        [(Z, NEG), (X, POS), (Y, NEG)],
        [(Z, NEG), (Y, NEG), (X, NEG)],
        [(Z, NEG), (X, NEG), (Y, POS)],
        [(Z, NEG), (Y, POS), (X, POS)],
    ]
};

impl Mul<Rotation> for Point3D {
    type Output = Point3D;

    fn mul(self, rotation: Rotation) -> Point3D {
        Point3D {
            x: rotation[0].1 * self[rotation[0].0],
            y: rotation[1].1 * self[rotation[1].0],
            z: rotation[2].1 * self[rotation[2].0],
        }
    }
}

impl FromStr for Point3D {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',');
        let x: Coord = iter
            .next()
            .ok_or(format!("Missing Point3D x coordinate in string \"{}\"", s))?
            .parse()
            .map_err(|c: ParseIntError| c.to_string())?;
        let y: Coord = iter
            .next()
            .ok_or(format!("Missing Point3D y coordinate in string \"{}\"", s))?
            .parse()
            .map_err(|c: ParseIntError| c.to_string())?;
        let z: Coord = iter
            .next()
            .ok_or(format!("Missing Point3D z coordinate in string \"{}\"", s))?
            .parse()
            .map_err(|c: ParseIntError| c.to_string())?;
        if let Some(_) = iter.next() {
            Err(format!(
                "Unexpected excess values in expected Point3D string \"{}\"",
                s
            ))
        } else {
            Ok(Point3D::new(x, y, z))
        }
    }
}

type Beacon = Point3D;
type Scanner = HashSet<Beacon>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct ScannerOrientation {
    rotation: Rotation,
    position: Point3D,
}

fn read_scanners(input: &str) -> Vec<Scanner> {
    let mut scanners = Vec::new();
    let mut lines = input.lines();
    while let Some(scanner_header) = lines.next() {
        assert!(scanner_header.starts_with("--- scanner "));
        scanners.push(
            lines
                .by_ref()
                .take_while(|l| !l.is_empty())
                .map(|l| l.parse())
                .collect::<Result<_, String>>()
                .unwrap(),
        );
    }
    scanners
}

fn overlaps(
    threshold: usize,
    scannera: &Scanner,
    scannerb: &Scanner,
) -> Option<ScannerOrientation> {
    for rotation in ROTATIONS {
        let mut cnts: HashMap<Beacon, usize> = HashMap::new();
        for a in scannera {
            for b in scannerb {
                let pos = *a - *b * rotation;
                if pos == Point3D::new(0, 0, 0) {}
                cnts.entry(pos).or_insert(0).add_assign(1);
            }
        }

        for (delta, cnt) in cnts.into_iter() {
            if cnt >= threshold {
                return Some(ScannerOrientation {
                    rotation: rotation,
                    position: delta,
                });
            }
        }
    }
    None
}

fn normalize(to: ScannerOrientation, scanner: Scanner) -> Scanner {
    scanner
        .into_iter()
        .map(|b| b * to.rotation + to.position)
        .collect()
}

fn normalize_scanners(
    threshold: usize,
    scanners: &[Scanner],
) -> (Scanner, Vec<ScannerOrientation>) {
    let mut confirmed_scanner = scanners[0].clone();
    let mut unconfirmed_scanners = scanners[1..].to_vec();
    let mut confirmed_orientations = Vec::with_capacity(scanners.len());
    confirmed_orientations.push(ScannerOrientation {
        rotation: ROTATIONS[0],
        position: Point3D::new(0, 0, 0),
    });

    loop {
        if unconfirmed_scanners.is_empty() {
            break (confirmed_scanner, confirmed_orientations);
        }

        let unconfirmed_scanner_count = unconfirmed_scanners.len();

        for i in 0..unconfirmed_scanners.len() {
            if let Some(orientation) =
                overlaps(threshold, &confirmed_scanner, &unconfirmed_scanners[i])
            {
                println!("Scanner confirmed.");
                confirmed_scanner.extend(normalize(orientation, unconfirmed_scanners.remove(i)));
                confirmed_orientations.push(orientation);
                break;
            }
        }

        if unconfirmed_scanner_count == unconfirmed_scanners.len() {
            panic!("No scanner overlaps found.");
        }
    }
}

fn part1(scanner: &Scanner) -> usize {
    scanner.len()
}

// 403 correct

fn part2(orientations: &[ScannerOrientation]) -> Coord {
    orientations
        .iter()
        .flat_map(|o1| {
            orientations
                .iter()
                .map(|o2| (o1.position - o2.position).manhattan_distance())
        })
        .max()
        .expect("No orientations?!")
}

// 10569 correct

fn main() {
    // Get filename from command-line args
    let filename = std::env::args().nth(1).expect("Missing filename");
    // Read input file
    let input =
        read_scanners(&std::fs::read_to_string(filename).expect("Failed to read input file"));

    let (scanner, orientations) = normalize_scanners(12, &input);

    println!("Part 1: {}", part1(&scanner));
    println!("Part 2: {}", part2(&orientations));
}

mod tests {
    #[test]
    fn test_samescanner_overlaps() {
        let input = super::read_scanners(
            "--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7

--- scanner 0 ---
1,-1,1
2,-2,2
3,-3,3
2,-1,3
-5,4,-6
-8,-7,0

--- scanner 0 ---
-1,-1,-1
-2,-2,-2
-3,-3,-3
-1,-3,-2
4,6,5
-7,0,8

--- scanner 0 ---
1,1,-1
2,2,-2
3,3,-3
1,3,-2
-4,-6,5
7,0,8

--- scanner 0 ---
1,1,1
2,2,2
3,3,3
3,1,2
-6,-4,-5
0,7,-8",
        );
        let (scanner, _orientations) = super::normalize_scanners(6, &input);
        assert_eq!(
            scanner,
            std::collections::HashSet::from_iter(vec![
                super::Point3D::new(-1, -1, 1),
                super::Point3D::new(-2, -2, 2),
                super::Point3D::new(-3, -3, 3),
                super::Point3D::new(-2, -3, 1),
                super::Point3D::new(5, 6, -4),
                super::Point3D::new(8, 0, 7),
            ])
        );
    }
}
