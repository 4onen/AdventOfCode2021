fn part1(input: &str) -> Result<usize, String> {
    input
        .lines()
        .map(|line| {
            let mut charstack: Vec<char> = Vec::new();
            for c in line.chars() {
                match c {
                    '[' => charstack.push(']'),
                    '(' => charstack.push(')'),
                    '{' => charstack.push('}'),
                    '<' => charstack.push('>'),
                    _ => {
                        if charstack.pop() != Some(c) {
                            match c {
                                ')' => return Ok(3),
                                ']' => return Ok(57),
                                '}' => return Ok(1197),
                                '>' => return Ok(25137),
                                _ => return Err(format!("Unexpected character {}", c)),
                            }
                        }
                    }
                }
            }
            return Ok(0);
        })
        .sum()
}

// 193275 correct

fn part2(input: &str) -> Result<usize, String> {
    let mut scores: Vec<usize> = input
        .lines()
        .filter_map(|line| {
            let mut charstack: Vec<char> = Vec::new();
            for c in line.chars() {
                match c {
                    '[' => charstack.push(']'),
                    '(' => charstack.push(')'),
                    '{' => charstack.push('}'),
                    '<' => charstack.push('>'),
                    _ => {
                        if charstack.pop() != Some(c) {
                            match c {
                                ')' | ']' | '}' | '>' => return None,
                                _ => return Some(Err(format!("Unexpected character {}", c))),
                            }
                        }
                    }
                }
            }

            Some(charstack.into_iter().rev().fold(Ok(0usize), |racc, c| {
                racc.and_then(|acc| match c {
                    ')' => Ok(5 * acc + 1),
                    ']' => Ok(5 * acc + 2),
                    '}' => Ok(5 * acc + 3),
                    '>' => Ok(5 * acc + 4),
                    _ => Err(format!("Unexpected character {}", c)),
                })
            }))
        })
        .collect::<Result<Vec<usize>, String>>()?;

    scores.sort_unstable();
    Ok(scores[scores.len() >> 1])
}

// 739239 too low (forgot to discard 0s from bad lines)
// 745936 too low (forgot to discard 0s from bad lines)
// 2429644557 is correct

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {:?}", part1(input));
    println!("Part 2: {:?}", part2(input));
}
