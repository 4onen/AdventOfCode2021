use minimax::minimax;
use std::collections::HashMap;
use std::str::FromStr;

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

#[derive(Debug, Clone)]
struct Polymer {
    pairs: HashMap<(char, char), usize>,
    first: char,
}

impl FromStr for Polymer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pairs = HashMap::new();

        if let Some(first) = s.chars().next() {
            s.chars().zip(s.chars().skip(1)).for_each(|c| {
                pairs.entry(c).and_modify(|e| *e += 1).or_insert(1);
            });

            Ok(Polymer { pairs, first })
        } else {
            Err("No characters in polymer string.".to_string())
        }
    }
}

fn step_polymer(polymer: &Polymer, rules: &Ruleset) -> Result<Polymer, String> {
    let mut pairs = HashMap::new();

    for (&(left, right), &count) in &polymer.pairs {
        if let Some(&result) = rules.get(&(left, right)) {
            pairs
                .entry((left, result))
                .and_modify(|e| *e += count)
                .or_insert(count);
            pairs
                .entry((result, right))
                .and_modify(|e| *e += count)
                .or_insert(count);
        } else {
            return Err(format!("Ruleset does not contain key: {:?}", (left, right)));
        }
    }

    let first = polymer.first;

    Ok(Polymer { pairs, first })
}

fn score_polymer(polymer: Polymer) -> usize {
    let mut cnts = HashMap::new();

    for ((_l, r), &cnt) in polymer.pairs.iter() {
        cnts.entry(r).and_modify(|e| *e += cnt).or_insert(cnt);
    }

    cnts.entry(&polymer.first)
        .and_modify(|e| *e += 1)
        .or_insert(1);

    if let Some((min, max)) = minimax(cnts.values()) {
        max - min
    } else {
        panic!("Zero length polymer!");
    }
}

fn part1(polymer: Polymer, rules: &Ruleset) -> Result<usize, String> {
    Ok(score_polymer(
        (0..10).try_fold(polymer, |p, _| step_polymer(&p, rules))?,
    ))
}

// 3306 correct

fn part2(polymer: Polymer, rules: &Ruleset) -> Result<usize, String> {
    Ok(score_polymer(
        (0..40).try_fold(polymer, |p, _| step_polymer(&p, rules))?,
    ))
}

// 3760312702877 correct

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
            .parse::<Polymer>()
            .expect(format!("Failed to parse polymer input: \"{}\"", input).as_str());
        let rules = lines
            .skip(1)
            .map(to_rule)
            .collect::<Result<Ruleset, _>>()
            .unwrap();
        (polymer_input, rules)
    };

    println!("Part 1: {:?}", part1(polymer_input.clone(), &rules));
    println!("Part 2: {:?}", part2(polymer_input, &rules));
}
