use std::fmt::Display;

struct BingoCard {
    numbers: [[u8; 5]; 5],
    marked: [[bool; 5]; 5],
    winningnum: Option<u8>,
}

impl Display for BingoCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.numbers {
            for num in row {
                write!(f, "{:2} ", num)?;
            }
            writeln!(f)?;
        }
        for row in &self.marked {
            for marked in row {
                write!(f, "{} ", if *marked { " X" } else { "  " })?;
            }
            writeln!(f)?;
        }
        if let Some(winningnum) = &self.winningnum {
            writeln!(f, "Winning number: {}", winningnum)?;
        }
        Ok(())
    }
}

impl From<&[&str]> for BingoCard {
    fn from(s: &[&str]) -> Self {
        let numbers: Vec<[u8; 5]> = s
            .into_iter()
            .map(|line| {
                let nums = line
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<u8>>();
                assert_eq!(nums.len(), 5);
                [nums[0], nums[1], nums[2], nums[3], nums[4]]
            })
            .collect();
        assert_eq!(numbers.len(), 5);

        BingoCard {
            numbers: [numbers[0], numbers[1], numbers[2], numbers[3], numbers[4]],
            marked: [[false; 5]; 5],
            winningnum: None,
        }
    }
}

trait Bingo {
    fn mark(&mut self, num: u8);
    fn won(&self) -> bool;
    fn score(&self) -> u32;
}

impl Bingo for BingoCard {
    fn mark(&mut self, num: u8) {
        let mut did_mark = false;
        for i in 0..5 {
            for j in 0..5 {
                if self.numbers[i][j] == num {
                    self.marked[i][j] = true;
                    did_mark = true;
                }
            }
        }

        if did_mark && self.winningnum.is_none() && self.won() {
            self.winningnum = Some(num);
        }
    }

    fn won(&self) -> bool {
        if self.winningnum.is_some() {
            return true;
        } else {
            for row in &self.marked {
                if row.iter().all(|&x| x) {
                    return true;
                }
            }

            for col in 0..5 {
                if self.marked.iter().all(|&row| row[col]) {
                    return true;
                }
            }
        }

        false
    }

    fn score(&self) -> u32 {
        self.winningnum
            .map(|winningnum| {
                let nums = self.numbers.into_iter().flatten();
                let marks = self.marked.into_iter().flatten();
                Iterator::zip(nums, marks)
                    .filter_map(|(n, m)| if !m { Some(n as u32) } else { None })
                    .sum::<u32>()
                    * (winningnum as u32)
            })
            .unwrap_or(0)
    }
}

fn part1(input: &str) -> u32 {
    let mut inputlines = input.lines().filter(|&line| !line.is_empty());
    let inputseq = inputlines
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<u8>().unwrap());

    let mut cards: Vec<BingoCard> = inputlines
        .collect::<Vec<&str>>()
        .chunks(5)
        .map(|lines| lines.into())
        .collect();

    for num in inputseq {
        // println!("Calling: {}", num);
        for (_i, card) in (&mut cards).iter_mut().enumerate() {
            card.mark(num);
            if card.won() {
                // println!("Winner: Card {}\n{}", _i + 1, card);
                return card.score();
            }
        }
    }

    panic!("No winning card found.")
}

// 5320 too low -- had condition on unmarked number filter backward
// 12796 correct

fn part2(input: &str) -> u32 {
    let mut inputlines = input.lines().filter(|&line| !line.is_empty());
    let inputseq = inputlines
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<u8>().unwrap());

    let mut cards: Vec<BingoCard> = inputlines
        .collect::<Vec<&str>>()
        .chunks(5)
        .map(|lines| lines.into())
        .collect();

    for num in inputseq {
        for i in 0..cards.len() {
            cards[i].mark(num);
            if cards.iter().all(|c| c.won()) {
                return cards[i].score();
            }
        }
    }

    panic!("Not all cards won!")
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
