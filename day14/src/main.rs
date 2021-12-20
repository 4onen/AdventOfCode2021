use minimax::minimax;
use std::collections::HashMap;

type Rule = ((char, char), char);

fn to_rule(s: &str) -> Result<Rule, String> {
    if !s.is_ascii() {
        Err(format!(
            "Non-ASCII characters are not allowed. Got string: \"{}\"",
            s
        ))
    } else if s.len() != 7 {
        Err(format!(
            "All rules must be exactly 7 characters long. Got string: \"{}\"",
            s
        ))
    } else if s.chars().any(|c| c.is_ascii_lowercase()) {
        Err(format!(
            "All rules must be uppercase. Got string: \"{}\"",
            s
        ))
    } else {
        let mut chars = s.chars();
        let left = chars.next().unwrap();
        let right = chars.next().unwrap();
        let result = chars.nth(4).unwrap();
        Ok(((left, right), result))
    }
}

type Ruleset = HashMap<(char, char), char>;

fn step_polymer(polymer: &str, rules: &Ruleset) -> String {
    [polymer.chars().next().unwrap()]
        .into_iter()
        .chain(
            polymer
                .chars()
                .zip(polymer.chars().skip(1))
                .flat_map(|(a, b)| {
                    if rules.contains_key(&(a, b)) {
                        [rules[&(a, b)], b].into_iter()
                    } else {
                        panic!("No rule for {} {} -> ?", a, b);
                    }
                }),
        )
        .collect()
}

fn run_polymer(polymer: &str, rules: &Ruleset, steps: u8) -> String {
    (0..steps).fold(polymer.to_string(), |polymer, i| {
        if i < 7 {
            println!("{}\t: {}", i, polymer);
        }
        step_polymer(&polymer, rules)
    })
}

fn score_polymer(polymer: String) -> usize {
    let mut cnts = HashMap::new();
    for c in polymer.chars() {
        cnts.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }

    if let Some((min, max)) = minimax(cnts.values()) {
        max - min
    } else {
        panic!("Zero length polymer!");
    }
}

fn part1(polymer: &str, rules: &Ruleset) -> usize {
    score_polymer(run_polymer(polymer, rules, 10))
}

// 3306 correct

fn part2(polymer: &str, rules: &Ruleset) -> usize {
    score_polymer(run_polymer(polymer, rules, 40))
}

fn main() {
    // Get the command line args
    let args: Vec<String> = std::env::args().collect();
    // Get the filename from the command line
    let filename = args
        .get(1)
        .expect("No filename provided. Please provide an input file.");
    // Read the input file
    let (polymer_input, rules) = {
        let input = std::fs::read_to_string(filename).expect("Failed to read input file.");
        let mut lines = input.lines();
        let polymer_input = lines
            .next()
            .expect("No polymer input provided.")
            .to_string();
        let rules = lines
            .skip(1)
            .map(to_rule)
            .collect::<Result<Ruleset, _>>()
            .unwrap();
        (polymer_input, rules)
    };

    println!("Part 1: {}", part1(&polymer_input, &rules));
    println!("Part 2: {}", part2(&polymer_input, &rules));
}
