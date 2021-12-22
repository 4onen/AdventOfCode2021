use std::fmt::Display;
use std::str::FromStr;

type Tile = u8;
type Coord = usize;

struct Map {
    sidelen: usize,
    tiles: Vec<Tile>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..self.sidelen {
            for x in 0..self.sidelen {
                let tile = self.tiles[y * self.sidelen + x];
                if tile < 1 || tile > 9 {
                    panic!("Invalid tile: {} at position {},{}", tile, x, y);
                }
                write!(f, "{} ", tile)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles: Vec<Tile> = Vec::new();
        let mut sidelen = 0;
        for line in s.lines() {
            tiles.extend(
                line.chars()
                    .map(|c| {
                        c.to_digit(10)
                            .map(|d| d as Tile)
                            .ok_or(format!("Invalid tile encountered: '{}'", c))
                    })
                    .collect::<Result<Vec<_>, Self::Err>>()?,
            );

            if sidelen == 0 {
                sidelen = tiles.len();
            } else if sidelen != line.len() {
                return Err(format!(
                    "Invalid map: row has length {} but previous rows have length {}",
                    line.len(),
                    sidelen
                ));
            }
        }
        Ok(Map {
            sidelen: sidelen,
            tiles: tiles,
        })
    }
}

// See https://doc.rust-lang.org/std/collections/binary_heap/index.html for the Dijkstra's
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    cost: usize,
    position: Coord,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Map {
    fn shortest_path_cost(&self, start: usize, goal: usize) -> Result<usize, String> {
        let mut dist = vec![std::usize::MAX; self.tiles.len()];
        let mut heap = std::collections::BinaryHeap::new();

        dist[start] = 0;
        heap.push(State {
            cost: 0,
            position: start,
        });

        while let Some(State { cost, position }) = heap.pop() {
            if position == goal {
                return Ok(cost);
            }

            if dist[position] < cost {
                continue;
            }

            let x = position % self.sidelen;
            // Check whether we can now get to any neighbors at less cost
            if x > 0 {
                let left = position - 1;
                let left_cost = dist[position] + self.tiles[left] as usize;
                if dist[left] > left_cost {
                    dist[left] = left_cost;
                    heap.push(State {
                        cost: left_cost,
                        position: left,
                    });
                }
            }
            if x < self.sidelen - 1 {
                let right = position + 1;
                let right_cost = dist[position] + self.tiles[right] as usize;
                if dist[right] > right_cost {
                    dist[right] = right_cost;
                    heap.push(State {
                        cost: right_cost,
                        position: right,
                    });
                }
            }
            if position >= self.sidelen {
                let up = position - self.sidelen;
                let up_cost = dist[position] + self.tiles[up] as usize;
                if dist[up] > up_cost {
                    dist[up] = up_cost;
                    heap.push(State {
                        cost: up_cost,
                        position: up,
                    });
                }
            }
            let down = position + self.sidelen;
            if down < self.tiles.len() {
                let down_cost = dist[position] + self.tiles[down] as usize;
                if dist[down] > down_cost {
                    dist[down] = down_cost;
                    heap.push(State {
                        cost: down_cost,
                        position: down,
                    });
                }
            }
        }

        Err("No path found.".to_string())
    }

    fn inc_tile(self, sidemult: u8) -> Map {
        let mut newtiles = Vec::with_capacity(self.tiles.len() * (sidemult * sidemult) as usize);
        for y in 0..sidemult {
            for line in self.tiles.chunks(self.sidelen) {
                for x in 0..sidemult {
                    newtiles.extend(line.iter().map(|&t| (t + x + y - 1) % 9 + 1));
                }
            }
        }

        Map {
            sidelen: self.sidelen * sidemult as usize,
            tiles: newtiles,
        }
    }
}

fn part1(map: &Map) -> Result<usize, String> {
    map.shortest_path_cost(0, map.tiles.len() - 1)
}

// 390 correct

fn part2(map: Map) -> Result<usize, String> {
    let newmap = map.inc_tile(5);
    println!("{}", newmap);
    newmap.shortest_path_cost(0, newmap.tiles.len() - 1)
}

// 2388 too low because I was wrapping numbers one value too soon
// 2814 correct

fn main() {
    // Get filename from system args
    let filename = std::env::args().nth(1).expect("Please provide a filename");
    // Read file
    let contents: Map = std::fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .parse()
        .unwrap();

    println!("Part 1: {}", part1(&contents).unwrap());
    println!("Part 2: {}", part2(contents).unwrap());
}
