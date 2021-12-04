const N_BITS: usize = 12usize;

fn input_to_bools(input: &str) -> Vec<[bool; N_BITS]> {
    input
        .lines()
        .map(|s| {
            let mut bools = [false; N_BITS];
            for (i, c) in s.chars().enumerate() {
                match c {
                    '1' => bools[i] = true,
                    '0' => (),
                    _ => panic!("Invalid input character: {}", c),
                }
            }
            bools
        })
        .collect()
}

fn determine_common(input: &[[bool; N_BITS]]) -> [bool; N_BITS] {
    let mut counts = [0 as i32; N_BITS];
    for &line in input {
        for (i, c) in counts.iter_mut().zip(line) {
            *i += if c { -1 } else { 1 };
        }
    }

    counts.map(|x| x < 0)
}

fn part1(input: &[[bool; N_BITS]]) -> u32 {
    let common = determine_common(input);
    let gamma: u16 = common
        .into_iter()
        .enumerate()
        .map(|(i, b)| if b { 1u16 << (N_BITS - 1 - i) } else { 0u16 })
        .sum();

    let mask: u16 = !((!0u16) << N_BITS);
    let epsilon: u16 = (!gamma) & mask;
    // println!("{:#016b} {:#016b} {:#016b}", gamma, epsilon, mask);

    gamma as u32 * epsilon as u32
}

// 1100 too low
// 3294500 too high
// 2640986 correct

fn rating(mut input: Vec<[bool; N_BITS]>, least_common: bool) -> u16 {
    let mut bit = 0;
    while input.len() > 1 {
        let criteria = {
            let (ones, zeroes) = input.iter().fold((0u32, 0u32), |(a, b), line| {
                if line[bit] {
                    (a + 1, b)
                } else {
                    (a, b + 1)
                }
            });
            // println!("bit {}: {} {}", bit, ones, zeroes);
            if least_common {
                ones < zeroes
            } else {
                ones >= zeroes
            }
        };
        input = input
            .into_iter()
            .filter(|line| line[bit] == criteria)
            .collect();
        bit += 1;
    }

    input[0]
        .into_iter()
        .enumerate()
        .map(|(i, b)| if b { 1u16 << (N_BITS - 1 - i) } else { 0u16 })
        .sum()
}

fn part2(input: Vec<[bool; N_BITS]>) -> u32 {
    let o2gen: u16 = rating(input.clone(), false);
    let co2scrub: u16 = rating(input, true);

    return (o2gen as u32) * (co2scrub as u32);
}

// 7074431 too high
// 6822109 correct

fn main() {
    let input: Vec<[bool; N_BITS]> = {
        let mut i = input_to_bools(include_str!("../input.txt"));
        i.sort_unstable();
        i
    };
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(input));
}
