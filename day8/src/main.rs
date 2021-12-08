fn part1(input: &[&str]) -> usize {
    input
        .into_iter()
        .map(|&s| s.split_whitespace().skip_while(|&s| s != "|").skip(1))
        .flatten()
        .filter(|s| match s.len() {
            2 => true,
            3 => true,
            4 => true,
            7 => true,
            _ => false,
        })
        .count()
}

// 448 too high because I thought a 4 used 5 segments
// 264 correct

fn main() {
    let input: Vec<&str> = include_str!("../input.txt").lines().collect();
    println!("{}", part1(input.as_slice()));
}
