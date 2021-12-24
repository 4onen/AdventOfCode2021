use std::{
    fmt::{Display, Formatter},
    iter::Sum,
    ops::Add,
    str::FromStr,
};

mod scrap {
    use std::{fmt::Display, iter::Sum, ops::Add, str::FromStr};

    #[derive(Debug, PartialEq, Eq, Clone)]
    enum PairSlot {
        Pair(Box<[PairSlot; 2]>),
        Regular(u8),
    }

    impl Display for PairSlot {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                PairSlot::Pair(p) => write!(f, "[{},{}]", p[0], p[1]),
                PairSlot::Regular(n) => write!(f, "{}", n),
            }
        }
    }

    use PairSlot::*;

    fn pairslot_from_iter(iter: &mut impl Iterator<Item = char>) -> Result<PairSlot, String> {
        let c = iter.next().ok_or("Empty input!")?;

        if c.is_ascii_digit() {
            let n = c.to_digit(10).ok_or(format!("Invalid digit: {}", c))? as u8;
            Ok(Regular(n))
        } else if c == '[' {
            let l = pairslot_from_iter(iter)?;
            let comma = iter
                .next()
                .ok_or("Unexpected end of input while expecting comma.")?;
            if comma != ',' {
                return Err(format!("Expected comma, got {}", comma));
            }
            let r = pairslot_from_iter(iter)?;
            let r_close = iter
                .next()
                .ok_or("Unexpected end of input while expecting closing bracket.")?;
            if r_close != ']' {
                return Err(format!("Expected closing bracket, got {}", r_close));
            }
            Ok(Pair(Box::new([l, r])))
        } else {
            Err(format!("Invalid character: {}", c))
        }
    }

    impl FromStr for PairSlot {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            pairslot_from_iter(s.chars().by_ref())
        }
    }

    enum Action {
        Explode(u8, u8),
        Split(u8),
        Leave(PairSlot),
    }

    impl PairSlot {
        fn from_two(arr: [PairSlot; 2]) -> Self {
            Pair(Box::new(arr))
        }

        fn normalize(&self) -> PairSlot {
            fn normalize_rec(slot: &PairSlot, depth: usize) -> PairSlot {
                fn action(slot: &PairSlot, depth: usize) -> Action {
                    match slot {
                        &Regular(n) => {
                            if n >= 10 {
                                Action::Split(n)
                            } else {
                                Action::Leave(Regular(n))
                            }
                        }
                        Pair(dat) => {
                            let (l, r) =
                                (normalize_rec(&dat[0], depth), normalize_rec(&dat[1], depth));
                            if depth >= 3 {
                                if let (Regular(l), Regular(r)) = (l, r) {
                                    Action::Explode(l, r)
                                } else {
                                    panic!(
                                        "Insufficiently reduced number at depth {}: {:?}",
                                        depth, slot
                                    )
                                }
                            } else {
                                Action::Leave(Pair(Box::new([l, r])))
                            }
                        }
                    }
                }

                match slot {
                    &Regular(n) => Regular(n),
                    Pair(dat) => {
                        let [ls, rs] = dat.as_ref();

                        match (action(ls, depth + 1), action(rs, depth + 1)) {
                            (Action::Explode(_l, r), _) => {
                                if let Regular(n) = rs {
                                    normalize_rec(
                                        &PairSlot::from_two([Regular(0), Regular(r + n)]),
                                        depth,
                                    )
                                } else {
                                    normalize_rec(
                                        &PairSlot::from_two([Regular(0), rs.clone()]),
                                        depth,
                                    )
                                }
                            }
                            (_, Action::Explode(l, _r)) => {
                                if let Regular(n) = ls {
                                    normalize_rec(
                                        &PairSlot::from_two([Regular(l + n), Regular(0)]),
                                        depth,
                                    )
                                } else {
                                    normalize_rec(
                                        &PairSlot::from_two([Regular(0), rs.clone()]),
                                        depth,
                                    )
                                }
                            }
                            (Action::Split(n), _) => normalize_rec(
                                &PairSlot::from_two([
                                    PairSlot::from_two([Regular(n >> 1), Regular((n >> 1) + 1)]),
                                    rs.clone(),
                                ]),
                                depth,
                            ),
                            (_, Action::Split(n)) => normalize_rec(
                                &PairSlot::from_two([
                                    ls.clone(),
                                    PairSlot::from_two([Regular(n >> 1), Regular((n >> 1) + 1)]),
                                ]),
                                depth,
                            ),
                            (Action::Leave(l), Action::Leave(r)) => PairSlot::from_two([l, r]),
                        }
                    }
                }
            }
            normalize_rec(self, 1)
        }
    }

    impl Add for PairSlot {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
            Pair(Box::new([self, rhs])).normalize()
        }
    }

    impl Sum for PairSlot {
        fn sum<I>(mut iter: I) -> Self
        where
            I: Iterator<Item = Self>,
        {
            let init = iter.next().unwrap_or(Regular(0));
            iter.fold(init, |acc, x| acc + x)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct SnailNumber(Vec<(usize, u8)>);

#[derive(Debug, Clone, Copy)]
enum SnailTreeNode {
    Leaf(u8),
    Branch(usize, usize),
}

#[derive(Debug, Clone)]
struct SnailTree {
    arena: Vec<SnailTreeNode>,
}

impl TryFrom<SnailNumber> for SnailTree {
    type Error = String;

    fn try_from(num: SnailNumber) -> Result<Self, Self::Error> {
        let mut arena: Vec<SnailTreeNode> = vec![];
        let mut stack: Vec<usize> = vec![];

        for (depth, n) in num.0 {
            while stack.len() < depth - 1 {
                arena.push(SnailTreeNode::Branch(0, 0));
                stack.push(arena.len() - 1);
            }

            let parent = stack.pop().ok_or("Stack underflow")?;
            arena.push(SnailTreeNode::Leaf(n));
            let child = arena.len() - 1;
            match arena[parent] {
                SnailTreeNode::Branch(l, r) => {
                    if l == 0 {
                        arena[parent] = SnailTreeNode::Branch(child, r);
                        stack.push(parent);
                    } else if r == 0 {
                        arena[parent] = SnailTreeNode::Branch(l, child);
                    } else {
                        return Err("Invalid branch".to_string());
                    }
                }
                _ => return Err("Unexpected leaf node".to_string()),
            }
        }

        Ok(SnailTree { arena })
    }
}

impl Display for SnailTree {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        fn show_node(tree: &SnailTree, f: &mut Formatter, id: usize) -> std::fmt::Result {
            match tree.arena[id] {
                SnailTreeNode::Leaf(n) => write!(f, "{}", n),
                SnailTreeNode::Branch(a, b) => {
                    write!(f, "[")?;
                    show_node(tree, f, a)?;
                    write!(f, ",")?;
                    show_node(tree, f, b)?;
                    write!(f, "]")?;
                    Ok(())
                }
            }
        }
        show_node(self, f, 0)
    }
}

impl FromStr for SnailNumber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = Vec::new();
        let mut depth: usize = 0;
        for c in s.chars() {
            if c == '[' {
                depth += 1;
            } else if c == ']' {
                depth -= 1;
            } else if c == ',' {
            } else {
                let n = c.to_digit(10).ok_or(format!("Invalid digit: {}", c))? as u8;
                v.push((depth, n));
            }
        }
        Ok(SnailNumber(v))
    }
}

impl SnailNumber {
    fn normalize(&mut self) {
        if let Some(i) = self.0.iter().position(|&(d, _)| d > 4) {
            // Explode
            let (dl, nl) = self.0[i];
            assert!(i < self.0.len());
            let (dr, nr) = self.0[i + 1];
            assert_eq!(dl, dr);

            if i > 0 {
                let (_, nll) = self.0.get_mut(i - 1).unwrap();
                *nll += nl;
            }
            if let Some((_, nrr)) = self.0.get_mut(i + 2) {
                *nrr += nr;
            }

            self.0.remove(i + 1);
            if let Some((d, n)) = self.0.get_mut(i) {
                *d -= 1;
                *n = 0;
            }
            self.normalize()
        } else if let Some(i) = self.0.iter().position(|&(_, i)| i >= 10) {
            // Split
            let (d, n) = self.0[i];
            self.0[i] = (d + 1, n / 2);
            self.0.insert(i + 1, (d + 1, n / 2 + 1));
            self.normalize()
        } else {
            ()
        }
    }
}

impl Add for SnailNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut s = SnailNumber(
            self.0
                .into_iter()
                .chain(rhs.0.into_iter())
                .map(|(d, n)| (d + 1, n))
                .collect::<Vec<_>>(),
        );
        s.normalize();
        s
    }
}

impl Sum for SnailNumber {
    fn sum<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let init = iter.next().unwrap_or(SnailNumber(Vec::new()));
        iter.fold(init, |acc, x| acc + x)
    }
}

fn main() {
    // Get filename from command line args
    let filename = std::env::args().nth(1).expect("No filename given!");
    // Read file
    let contents = std::fs::read_to_string(filename)
        .expect("Failed to read file!")
        .lines()
        .map(|c| c.parse::<SnailNumber>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse file!");

    let t: SnailTree = contents
        .into_iter()
        .sum::<SnailNumber>()
        .try_into()
        .unwrap();

    println!("Part1: {}", t);
}
