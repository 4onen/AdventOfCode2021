fn part1(input: &[u8]) -> usize {
    let mut fish = [0usize; 9];
    for &t in input {
        fish[t as usize] += 1usize;
    }

    println!("00: {:?}", fish);

    for _t in 1..=80 {
        let mut new_fish = [0usize; 9];
        for i in 0..8 {
            new_fish[i] = fish[i + 1]
        }
        new_fish[8] = fish[0];
        new_fish[6] += fish[0];
        fish = new_fish;
        println!("{:02}: {:?}", _t, fish);
    }

    fish.into_iter().sum()
}

// 227214 too low
// 372300 correct

fn part2(input: &[u8]) -> usize {
    let mut fish = [0usize; 9];
    for &t in input {
        fish[t as usize] += 1usize;
    }

    for _t in 1..=256 {
        let mut new_fish = [0usize; 9];
        for i in 0..8 {
            new_fish[i] = fish[i + 1]
        }
        new_fish[8] = fish[0];
        new_fish[6] += fish[0];
        fish = new_fish;
    }

    fish.into_iter().sum()
}

// 1675781200288 correct

fn main() {
    let input = include_str!("../input.txt")
        .trim()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    debug_assert!(input.iter().all(|&x| x <= 9));

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
