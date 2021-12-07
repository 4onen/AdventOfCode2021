type Coord = u32;

fn part1(input: &[Coord]) -> Coord {
    let min = *(input.iter().min().unwrap());
    let max = *(input.iter().max().unwrap());
    (min..=max)
        .map(|xtarget| {
            input
                .iter()
                .map(|&xstart| Coord::max(xtarget, xstart) - Coord::min(xtarget, xstart))
                .sum()
        })
        .min()
        .unwrap()
}

// 328187 correct

fn part2(input: &[Coord]) -> Coord {
    let min = *(input.iter().min().unwrap());
    let max = *(input.iter().max().unwrap());
    (min..=max)
        .map(|xtarget| {
            input
                .iter()
                .map(|&xstart| {
                    let distance = Coord::max(xtarget, xstart) - Coord::min(xtarget, xstart);
                    (distance * distance + distance) >> 1
                })
                .sum()
        })
        .min()
        .unwrap()
}

// 91257582 correct

fn main() {
    let input = include_str!("../input.txt")
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<Coord>>();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
