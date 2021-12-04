#[derive(Debug)]
enum Movement {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl std::str::FromStr for Movement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let direction = words.next().unwrap();
        let distance = words.next().unwrap().parse::<u32>().unwrap();
        match direction {
            "forward" => Ok(Movement::Forward(distance)),
            "down" => Ok(Movement::Down(distance)),
            "up" => Ok(Movement::Up(distance)),
            _ => Err(format!(
                "Unknown direction {} with distance {}",
                direction, distance
            )),
        }
    }
}

fn part1(input: &[Movement]) -> u32 {
    let mut pos: u32 = 0;
    let mut depth: i32 = 0;
    for movement in input {
        match movement {
            Movement::Forward(distance) => pos += *distance,
            Movement::Down(distance) => depth += *distance as i32,
            Movement::Up(distance) => depth -= *distance as i32,
        }
    }

    return pos * (depth as u32);
}

fn part2(input: &[Movement]) -> u32 {
    let mut pos: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for movement in input {
        match movement {
            Movement::Down(distance) => aim += *distance as i32,
            Movement::Up(distance) => aim -= *distance as i32,
            Movement::Forward(distance) => {
                depth += aim * *distance as i32;
                pos += *distance as i32;
            }
        }
    }

    return (pos * depth) as u32;
}

fn main() {
    let input: Vec<Movement> = include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<Movement>().unwrap())
        .collect();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
