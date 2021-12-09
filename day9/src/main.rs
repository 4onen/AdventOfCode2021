use std::collections::HashMap;

type EType = String;
type MyResult<T> = Result<T, EType>;

fn is_low_point(input: &[Vec<u8>], y: usize, x: usize) -> bool {
    let lower_than_above = y < 1 || input[y][x] < input[y - 1][x];
    let lower_than_left = x < 1 || input[y][x] < input[y][x - 1];
    let lower_than_right = x >= input.len() - 1 || input[y][x] < input[y][x + 1];
    let lower_than_below = y >= input.len() - 1 || input[y][x] < input[y + 1][x];
    lower_than_above && lower_than_left && lower_than_right && lower_than_below
}

fn part1(input: &[Vec<u8>]) -> u32 {
    let mut total_risk: u32 = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if is_low_point(input, y, x) {
                total_risk += input[y][x] as u32 + 1u32;
            }
        }
    }
    total_risk
}

// 494 correct

fn part2(input: &[Vec<u8>]) -> usize {
    let mut basins: HashMap<(usize, usize), usize> = HashMap::new();
    (0..input.len())
        .map(|y| (0..input[0].len()).map(move |x| (x, y)))
        .flatten()
        .filter(|(x, y)| input[*y][*x] < 9)
        .map(|(mut x, mut y)| {
            while !is_low_point(input, y, x) {
                if x < input[0].len() - 1 && input[y][x] > input[y][x + 1] {
                    x += 1;
                } else if y < input.len() - 1 && input[y][x] > input[y + 1][x] {
                    y += 1;
                } else if x > 0 && input[y][x] > input[y][x - 1] {
                    x -= 1;
                } else if y > 0 && input[y][x] > input[y - 1][x] {
                    y -= 1;
                }
            }
            (x, y)
        })
        .for_each(|pt| {
            let pt_entry = basins.entry(pt).or_insert(0);
            *pt_entry += 1;
        });

    let mut vs: Vec<usize> = basins.values().map(|x| *x).collect();
    vs.sort_unstable();
    vs[(vs.len() - 3)..]
        .into_iter()
        .fold(1usize, |acc, &x| acc * x)
}

// 1048128 correct

fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    c.to_digit(10)
                        .map(|d| d as u8)
                        .ok_or(format!("Invalid char: {}", c))
                })
                .collect::<MyResult<Vec<u8>>>()
        })
        .collect::<MyResult<Vec<Vec<u8>>>>()
        .unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
