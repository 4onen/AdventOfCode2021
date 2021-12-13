use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum FoldInstruction {
    AlongY(i32),
    AlongX(i32),
}

impl FromStr for FoldInstruction {
    type Err = ();

    // Format in the file:
    // "fold along y=7"
    // "fold along x=5"
    // etc.

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, value) = s.split_once('=').unwrap();
        let value = value.parse::<i32>().unwrap();
        match direction {
            "fold along y" => Ok(FoldInstruction::AlongY(value)),
            "fold along x" => Ok(FoldInstruction::AlongX(value)),
            _ => Err(()),
        }
    }
}

type Dot = (i32, i32);
type Dots = Vec<Dot>;

fn do_fold(mut dots: Dots, instr: FoldInstruction) -> Dots {
    match instr {
        FoldInstruction::AlongY(py) => {
            dots.sort_unstable_by_key(|&dot| dot.1);
            let partition = dots
                .binary_search_by_key(&py, |&dot| dot.1)
                .unwrap_or_else(|err| err);
            let (above_fold, below_fold) = dots.split_at_mut(partition);
            let mut above_fold = above_fold.to_vec();
            below_fold
                .into_iter()
                .map(|(x, y)| (*x, 2 * py - *y))
                .for_each(|dot| {
                    if !above_fold.contains(&dot) {
                        above_fold.push(dot);
                    }
                });
            above_fold
        }

        FoldInstruction::AlongX(px) => {
            dots.sort_unstable_by_key(|&dot| dot.0);
            let partition = dots
                .binary_search_by_key(&px, |&dot| dot.0)
                .unwrap_or_else(|err| err);
            let (left_fold, right_fold) = dots.split_at_mut(partition);
            let mut left_fold = left_fold.to_vec();
            right_fold
                .into_iter()
                .map(|(x, y)| (2 * px - *x, *y))
                .for_each(|dot| {
                    if !left_fold.contains(&dot) {
                        left_fold.push(dot);
                    }
                });
            left_fold
        }
    }
}

fn part1(dots: Dots, instr: FoldInstruction) -> usize {
    do_fold(dots, instr).len()
}

// 839 too high because I wasn't folding correctly
// 704 correct

fn part2(dots: Dots, instrs: Vec<FoldInstruction>) -> String {
    let dots: Dots = instrs.into_iter().fold(dots, do_fold);
    assert!(dots.iter().all(|dot| dot.0 >= 0 && dot.1 >= 0));
    let max_x = dots.iter().map(|dot| dot.0).max().unwrap();
    let max_y = dots.iter().map(|dot| dot.1).max().unwrap();
    let mut grid = vec![vec!['.'; max_x as usize + 1]; max_y as usize + 1];
    for dot in dots {
        grid[dot.1 as usize][dot.0 as usize] = '#';
    }
    [String::default()]
        .into_iter()
        .chain(
            grid.into_iter()
                .map(|row| row.into_iter().collect::<String>()),
        )
        .collect::<Vec<_>>()
        .join("\n")
}

// #..#..##...##....##.###..####.#..#..##.
// #..#.#..#.#..#....#.#..#.#....#..#.#..#
// ####.#....#..#....#.###..###..####.#...
// #..#.#.##.####....#.#..#.#....#..#.#...
// #..#.#..#.#..#.#..#.#..#.#....#..#.#..#
// #..#..###.#..#..##..###..####.#..#..##.
// HGAJBEHC correct

fn main() {
    // Get the command line args:
    let args: Vec<String> = std::env::args().collect();
    // Get the filename from the command line:
    let filename = &args
        .get(1)
        .expect("No filename provided. Please provide an input file.");
    // Read the input file:
    let input = std::fs::read_to_string(filename).expect("Failed to read input file.");
    let dots: Dots = input
        .lines()
        .take_while(|&l| !l.is_empty())
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .collect();
    let folds: Vec<FoldInstruction> = input
        .lines()
        .skip_while(|&l| !l.is_empty())
        .skip(1)
        .map(|l| l.parse::<FoldInstruction>())
        .collect::<Result<_, _>>()
        .unwrap();

    println!("Part 1: {}", part1(dots.clone(), folds[0]));
    println!("Part 2: {}", part2(dots, folds));
}
