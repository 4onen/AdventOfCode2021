fn part1(input: &[u32]) -> u32 {
    let (cnt, _) = input.iter().fold((0, u32::MAX), |(cnt, last), x| {
        (if *x > last { cnt + 1 } else { cnt }, *x)
    });
    cnt
}

fn part2(input: &[u32]) -> u32 {
    let windows = input
        .windows(3)
        .map(|window| window.iter().sum::<u32>())
        .collect::<Vec<_>>();
    windows
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count() as u32
}

// 1876 too high
// 1852 too high

fn main() {
    let input: Vec<u32> = include_str!("input.txt")
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    println!("Part 1: {}", part1(input.as_slice()));
    println!("Part 2: {}", part2(input.as_slice()));
}
