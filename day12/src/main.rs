use std::{collections::BTreeSet, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CaveCellType {
    BigCave,
    SmallCave,
}

struct CaveCell {
    cavename: String,
    cavetype: CaveCellType,
    links: BTreeSet<usize>,
}

impl PartialEq for CaveCell {
    fn eq(&self, other: &Self) -> bool {
        self.cavename == other.cavename
    }
}

impl Eq for CaveCell {}

struct CaveGraph {
    cells: Vec<CaveCell>,
}

type CaveId = usize;

impl CaveGraph {
    fn new() -> CaveGraph {
        CaveGraph { cells: Vec::new() }
    }

    fn get_cell(&self, cavename: &str) -> Option<CaveId> {
        self.cells.iter().position(|cell| cell.cavename == cavename)
    }

    fn get_cell_data(&self, caveid: CaveId) -> Option<&CaveCell> {
        self.cells.get(caveid)
    }

    fn get_or_create_cell(&mut self, cavename: &str) -> CaveId {
        if let Some(i) = self.cells.iter().position(|cell| cell.cavename == cavename) {
            i
        } else {
            let cell = CaveCell {
                cavename: cavename.to_string(),
                cavetype: if cavename.chars().all(|c| c.is_uppercase()) {
                    CaveCellType::BigCave
                } else {
                    CaveCellType::SmallCave
                },
                links: BTreeSet::new(),
            };
            self.cells.push(cell);
            self.cells.len() - 1
        }
    }

    fn add_cell_link(&mut self, cave_a: &str, cave_b: &str) {
        let a = self.get_or_create_cell(cave_a);
        let b = self.get_or_create_cell(cave_b);
        self.cells[a].links.insert(b);
        self.cells[b].links.insert(a);
    }
}

impl FromStr for CaveGraph {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut graph = CaveGraph::new();

        for line in input.lines() {
            let (a, b) = line
                .split_once("-")
                .ok_or(format!("Invalid line: {}", line))?;
            graph.add_cell_link(a, b);
        }

        Ok(graph)
    }
}

fn part1(graph: &CaveGraph) -> usize {
    let start = graph.get_cell("start").unwrap();
    let end = graph.get_cell("end").unwrap();

    let mut unique_paths = 0;

    let mut path = vec![start];
    let mut next_options: Vec<Vec<usize>> =
        vec![graph.cells[start].links.iter().cloned().collect()];

    let can_visit = |path: &Vec<usize>, tgt: usize| -> bool {
        let d = graph.get_cell_data(tgt).unwrap();
        match d.cavetype {
            CaveCellType::BigCave => true,
            CaveCellType::SmallCave => !path.contains(&tgt),
        }
    };

    loop {
        if path.is_empty() {
            break;
        }

        if let Some(next) = next_options.last_mut().unwrap().pop() {
            if next == end {
                unique_paths += 1;
            } else if can_visit(&path, next) {
                path.push(next);
                next_options.push(graph.cells[next].links.iter().cloned().collect());
            }
        } else {
            next_options.pop();
            path.pop();
        }
    }

    unique_paths
}

// 4186 correct

fn part2(graph: &CaveGraph) -> usize {
    let start = graph.get_cell("start").unwrap();
    let end = graph.get_cell("end").unwrap();

    let mut doublevisit = false;

    let mut unique_paths = 0;

    let mut path = vec![start];
    let mut next_options: Vec<Vec<usize>> =
        vec![graph.cells[start].links.iter().cloned().collect()];

    let can_visit = |path: &Vec<usize>, doublevisit: bool, tgt: usize| -> bool {
        let d = graph.get_cell_data(tgt).unwrap();
        match d.cavetype {
            CaveCellType::BigCave => true,
            CaveCellType::SmallCave => {
                (doublevisit == false && tgt != start) || !path.contains(&tgt)
            }
        }
    };

    loop {
        if path.is_empty() {
            break;
        }

        if let Some(next) = next_options.last_mut().unwrap().pop() {
            if next == end {
                unique_paths += 1;
            } else if can_visit(&path, doublevisit, next) {
                if graph.get_cell_data(next).unwrap().cavetype == CaveCellType::SmallCave
                    && path.contains(&next)
                {
                    doublevisit = true;
                }
                path.push(next);
                next_options.push(graph.cells[next].links.iter().cloned().collect());
            }
        } else {
            next_options.pop();
            let id = path.pop().unwrap();
            if graph.get_cell_data(id).unwrap().cavetype == CaveCellType::SmallCave
                && path.contains(&id)
            {
                doublevisit = false;
            }
        }
    }

    unique_paths
}

// 118890 too high because I double-visited start
// 92111 correct

fn main() {
    // Get the command line args:
    let args: Vec<String> = std::env::args().collect();
    // Get the filename from the command line:
    let filename = &args
        .get(1)
        .expect("No filename provided. Please provide an input file.");
    // Read the input file:
    let input: CaveGraph = std::fs::read_to_string(filename)
        .expect("Something went wrong reading the given file")
        .as_str()
        .parse()
        .unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
