use std::fmt::{Debug, Display};
use std::iter::Sum;
use std::ops::Add;
use std::str::{Chars, FromStr};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum SnailNumberNode {
    Leaf(u8),
    Node(usize, usize),
}

#[derive(Clone)]
struct SnailNumber {
    nodes: Vec<SnailNumberNode>,
    root: usize,
}

impl PartialEq for SnailNumber {
    fn eq(&self, other: &Self) -> bool {
        fn cmp(a: &[SnailNumberNode], b: &[SnailNumberNode], i: usize, j: usize) -> bool {
            match (a.get(i), b.get(j)) {
                (Some(&SnailNumberNode::Leaf(n1)), Some(&SnailNumberNode::Leaf(n2))) => n1 == n2,
                (Some(&SnailNumberNode::Node(ll, lr)), Some(&SnailNumberNode::Node(rl, rr))) => {
                    cmp(a, b, ll, rl) && cmp(a, b, lr, rr)
                }
                _ => false,
            }
        }
        cmp(&self.nodes, &other.nodes, self.root, other.root)
    }
}

impl Eq for SnailNumber {}

impl Display for SnailNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fn rec(nodes: &[SnailNumberNode], i: usize) -> String {
            match nodes[i] {
                SnailNumberNode::Leaf(n) => n.to_string(),
                SnailNumberNode::Node(l, r) => format!("[{},{}]", rec(nodes, l), rec(nodes, r)),
            }
        }
        write!(f, "{}", rec(&self.nodes, self.root))
    }
}

impl Debug for SnailNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SnailNumber")
            .field("nodes", &self.to_string())
            .finish()
    }
}

impl FromStr for SnailNumber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes: Vec<SnailNumberNode> = Vec::new();
        fn rec(nodes: &mut Vec<SnailNumberNode>, cs: &mut Chars) -> Result<usize, String> {
            let first = cs.next().ok_or("Expected number or [, got empty string.")?;
            if first.is_digit(10) {
                let idx = nodes.len();
                nodes.push(SnailNumberNode::Leaf(
                    first
                        .to_digit(10)
                        .ok_or(format!("Expected number, got {}.", first))?
                        as u8,
                ));
                Ok(idx)
            } else if first == '[' {
                let left = rec(nodes, cs)?;
                if let Some(comma) = cs.next() {
                    if comma != ',' {
                        return Err(format!("Expected comma, got {}.", comma));
                    }
                } else {
                    return Err("Expected comma.".to_string());
                }
                let right = rec(nodes, cs)?;
                if let Some(close) = cs.next() {
                    if close != ']' {
                        return Err(format!("Expected ], got {}.", close));
                    }
                } else {
                    return Err("Expected ].".to_string());
                }
                let id = nodes.len();
                nodes.push(SnailNumberNode::Node(left, right));
                Ok(id)
            } else {
                Err(format!("Expected number or [, got {}.", first))
            }
        }

        rec(&mut nodes, &mut s.chars())?;
        Ok(SnailNumber {
            root: nodes.len() - 1,
            nodes,
        })
    }
}

impl SnailNumberNode {
    fn is_leaf(&self) -> bool {
        match self {
            SnailNumberNode::Leaf(_) => true,
            SnailNumberNode::Node(_, _) => false,
        }
    }
}

impl SnailNumber {
    fn magnitude(&self) -> u64 {
        fn rec(nodes: &[SnailNumberNode], id: usize) -> u64 {
            match nodes[id] {
                SnailNumberNode::Leaf(v) => v as u64,
                SnailNumberNode::Node(left, right) => 3 * rec(nodes, left) + 2 * rec(nodes, right),
            }
        }
        rec(&self.nodes, self.root)
    }

    fn splits(&mut self) -> bool {
        fn rec(nodes: &mut Vec<SnailNumberNode>, id: usize) -> bool {
            match nodes[id] {
                SnailNumberNode::Leaf(0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9) => false,
                SnailNumberNode::Leaf(n) => {
                    let left = nodes.len();
                    let right = nodes.len() + 1;
                    nodes.extend([
                        SnailNumberNode::Leaf(n / 2),
                        SnailNumberNode::Leaf((n + 1) / 2),
                    ]);
                    nodes[id] = SnailNumberNode::Node(left, right);
                    true
                }
                SnailNumberNode::Node(left, right) => rec(nodes, left) || rec(nodes, right),
            }
        }
        rec(&mut self.nodes, self.root)
    }

    fn explodes(&mut self) -> bool {
        fn rec(
            nodes: &mut Vec<SnailNumberNode>,
            id: usize,
            depth: usize,
            last_value: &mut Option<usize>,
            spill: &mut Option<u8>,
        ) -> bool {
            match &mut nodes[id] {
                SnailNumberNode::Leaf(v) => {
                    if let Some(spill_n) = *spill {
                        *v += spill_n;
                        *spill = None;
                        true
                    } else {
                        *last_value = Some(id);
                        false
                    }
                }
                &mut SnailNumberNode::Node(left, right) => {
                    if depth < 4
                        || spill.is_some()
                        || !nodes[left].is_leaf()
                        || !nodes[right].is_leaf()
                    {
                        rec(nodes, left, depth + 1, last_value, spill)
                            || rec(nodes, right, depth + 1, last_value, spill)
                    } else if let (
                        SnailNumberNode::Leaf(left_value),
                        SnailNumberNode::Leaf(right_value),
                    ) = (nodes[left], nodes[right])
                    {
                        if let Some(SnailNumberNode::Leaf(last_value_ref)) =
                            last_value.and_then(|i| nodes.get_mut(i))
                        {
                            *last_value_ref += left_value;
                        }
                        *spill = Some(right_value);
                        nodes[id] = SnailNumberNode::Leaf(0);
                        false
                    } else {
                        unreachable!()
                    }
                }
            }
        }

        let mut last_value: Option<usize> = None;
        let mut spill: Option<u8> = None;
        rec(&mut self.nodes, self.root, 0, &mut last_value, &mut spill)
    }
}

impl Add for SnailNumber {
    type Output = SnailNumber;

    fn add(mut self, other: SnailNumber) -> SnailNumber {
        let right_offset = self.nodes.len();
        let right_root = other.root + right_offset;
        self.nodes
            .extend(other.nodes.iter().cloned().map(|n| match n {
                SnailNumberNode::Leaf(v) => SnailNumberNode::Leaf(v),
                SnailNumberNode::Node(left, right) => {
                    SnailNumberNode::Node(left + right_offset, right + right_offset)
                }
            }));
        let root = self.nodes.len();
        self.nodes
            .push(SnailNumberNode::Node(self.root, right_root));
        self.root = root;
        while self.explodes() || self.splits() {}
        self
    }
}

impl Sum for SnailNumber {
    fn sum<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        if let Some(first) = iter.next() {
            iter.fold(first, |acc, next| acc + next)
        } else {
            panic!("Snail numbers have no mempty.");
        }
    }
}

fn _lines_sum(input: &str) -> Result<SnailNumber, String> {
    input
        .lines()
        .map(|line| line.parse::<SnailNumber>())
        .collect::<Result<Vec<_>, _>>()
        .map(|numbers| numbers.into_iter().sum::<SnailNumber>())
}

fn part1(input: &[SnailNumber]) -> u64 {
    input.iter().cloned().sum::<SnailNumber>().magnitude()
}

fn part2(input: &[SnailNumber]) -> u64 {
    input
        .iter()
        .flat_map(|a| input.iter().map(|b| (a.clone() + b.clone()).magnitude()))
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::SnailNumberNode::*;
    use super::*;

    #[test]
    fn test_minimum_parse() {
        let input = "[1,2]";
        let result = SnailNumber::from_str(input).expect("Failed to parse input.");
        assert_eq!(
            result,
            SnailNumber {
                nodes: vec![Leaf(1), Leaf(2), Node(0, 1)],
                root: 2
            }
        );
    }

    #[test]
    fn test_minimum_parse_nest() {
        let input = "[[1,2],3]";
        let result = SnailNumber::from_str(input).expect("Failed to parse input.");
        assert_eq!(
            result,
            SnailNumber {
                nodes: vec![Leaf(1), Leaf(2), Node(0, 1), Leaf(3), Node(2, 3)],
                root: 4
            }
        );
    }

    #[test]
    fn test_minimum_parse_nest2() {
        let input = "[9,[8,7]]";
        let result = SnailNumber::from_str(input).expect("Failed to parse input.");
        assert_eq!(
            result,
            SnailNumber {
                nodes: vec![Leaf(9), Leaf(8), Leaf(7), Node(1, 2), Node(0, 3)],
                root: 4
            }
        );
    }

    #[test]
    fn test_minimum_parse_nest3() {
        let input = "[[1,9],[8,5]]";
        let result = SnailNumber::from_str(input).expect("Failed to parse input.");
        assert_eq!(
            result,
            SnailNumber {
                nodes: vec![
                    Leaf(1),
                    Leaf(9),
                    Node(0, 1),
                    Leaf(8),
                    Leaf(5),
                    Node(3, 4),
                    Node(2, 5)
                ],
                root: 6
            }
        );
    }

    #[test]
    fn test_parse1() {
        let input = "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]";
        let result = SnailNumber::from_str(input).expect("Failed to parse input.");
        let expected = vec![
            Leaf(1),
            Leaf(2),
            Node(0, 1),
            Leaf(3),
            Leaf(4),
            Node(3, 4),
            Node(2, 5),
            Leaf(5),
            Leaf(6),
            Node(7, 8),
            Leaf(7),
            Leaf(8),
            Node(10, 11),
            Node(9, 12),
            Node(6, 13),
            Leaf(9),
            Node(14, 15),
        ];
        let expected = SnailNumber {
            root: expected.len() - 1,
            nodes: expected,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_addition1() {
        let input = "[1,2]\n[[3,4],5]";
        let result = _lines_sum(input).expect("Failed to parse input.");
        assert_eq!(
            result,
            SnailNumber::from_str("[[1,2],[[3,4],5]]").expect("Failed to parse expected result.")
        );
    }

    #[test]
    fn test_addition_example1() {
        let input = "[1,1]\n[2,2]\n[3,3]\n[4,4]";
        let result = _lines_sum(input).expect("Failed to parse input.");
        assert_eq!(
            result,
            SnailNumber::from_str("[[[[1,1],[2,2]],[3,3]],[4,4]]")
                .expect("Failed to parse expected result.")
        );
    }

    #[test]
    fn test_addition_example2() {
        let input = "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]";
        let result = _lines_sum(input).expect("Failed to parse input.");
        assert_eq!(
            result,
            SnailNumber::from_str("[[[[3,0],[5,3]],[4,4]],[5,5]]")
                .expect("Failed to parse expected result.")
        );
    }

    #[test]
    fn test_addition_example3() {
        let input = "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]";
        let result = _lines_sum(input).expect("Failed to parse input.");
        assert_eq!(
            result,
            SnailNumber::from_str("[[[[5,0],[7,4]],[5,5]],[6,6]]")
                .expect("Failed to parse expected result.")
        );
    }

    #[test]
    fn test_addition2() {
        let input = "[[2,2],[[3,6],[[4,4],[5,5]]]]\n[1,1]";
        let result = _lines_sum(input).expect("Failed to parse input.");
        assert_eq!(
            result,
            SnailNumber::from_str("[[[2,2],[[8,7],[0,7]]],[6,1]]")
                .expect("Failed to parse expected result.")
        );
    }

    #[test]
    fn test_addition_example4() {
        let input = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";
        let result = _lines_sum(input).expect("Failed to parse input.");
        assert_eq!(
            result,
            SnailNumber::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
                .expect("Failed to parse expected result.")
        );
    }

    #[test]
    fn test_addition_example5() {
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        let result = _lines_sum(input).expect("Failed to parse input.");
        assert_eq!(
            result,
            SnailNumber::from_str("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")
                .expect("Failed to parse expected result.")
        );
        assert_eq!(result.magnitude(), 4140)
    }
}

fn main() {
    // Get filename from command line args
    let filename = std::env::args().nth(1).expect("No filename given!");
    // Read file
    let input = std::fs::read_to_string(filename)
        .expect("Failed to read file!")
        .lines()
        .map(|line| line.parse::<SnailNumber>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse input.");

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}
