fn part1(mut grid: Vec<Vec<u8>>) -> usize {
    let mut total_flash_count: usize = 0;

    for _ in 0..100 {
        let mut check_stack = (0..grid.len())
            .flat_map(|y| (0..grid[y].len()).map(move |x| (x, y)))
            .collect::<Vec<(usize, usize)>>();

        let mut flashes: Vec<(usize, usize)> = Vec::new();

        while let Some((x, y)) = check_stack.pop() {
            grid[y][x] += 1;
            if grid[y][x] > 9 {
                grid[y][x] = 0;
                flashes.push((x, y));

                if y > 0 {
                    check_stack.push((x, y - 1));
                    if x > 0 {
                        check_stack.push((x - 1, y - 1));
                    }
                    if x < grid[y].len() - 1 {
                        check_stack.push((x + 1, y - 1));
                    }
                }
                if y < grid.len() - 1 {
                    check_stack.push((x, y + 1));
                    if x > 0 {
                        check_stack.push((x - 1, y + 1));
                    }
                    if x < grid[y].len() - 1 {
                        check_stack.push((x + 1, y + 1));
                    }
                }
                if x > 0 {
                    check_stack.push((x - 1, y));
                }
                if x < grid[y].len() - 1 {
                    check_stack.push((x + 1, y));
                }
            }
        }

        total_flash_count += flashes.len();

        flashes.into_iter().for_each(|(x, y)| {
            grid[y][x] = 0;
        });
    }

    total_flash_count
}

// 1665 correct

fn part2(mut grid: Vec<Vec<u8>>) -> usize {
    let mut step: usize = 0;

    let mut check_stack: Vec<(usize, usize)>;
    let mut flashes: Vec<(usize, usize)> = Vec::new();

    loop {
        step += 1;
        check_stack = (0..grid.len())
            .flat_map(|y| (0..grid[y].len()).map(move |x| (x, y)))
            .collect::<Vec<(usize, usize)>>();

        flashes.clear();

        while let Some((x, y)) = check_stack.pop() {
            grid[y][x] += 1;
            if grid[y][x] > 9 {
                grid[y][x] = 0;
                flashes.push((x, y));

                if y > 0 {
                    check_stack.push((x, y - 1));
                    if x > 0 {
                        check_stack.push((x - 1, y - 1));
                    }
                    if x < grid[y].len() - 1 {
                        check_stack.push((x + 1, y - 1));
                    }
                }
                if y < grid.len() - 1 {
                    check_stack.push((x, y + 1));
                    if x > 0 {
                        check_stack.push((x - 1, y + 1));
                    }
                    if x < grid[y].len() - 1 {
                        check_stack.push((x + 1, y + 1));
                    }
                }
                if x > 0 {
                    check_stack.push((x - 1, y));
                }
                if x < grid[y].len() - 1 {
                    check_stack.push((x + 1, y));
                }
            }
        }

        if flashes.len() == grid.len() * grid[0].len() {
            break;
        }

        flashes.iter().for_each(|&(x, y)| {
            grid[y][x] = 0;
        });
    }

    step
}

// 235 correct

fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - '0' as u8).collect::<Vec<u8>>())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}
