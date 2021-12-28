use std::collections::{BinaryHeap, HashMap};

type Amphipod = u8;
type Steps = usize;
type Cost = usize;
type CostPerStep = usize;

fn cost_per_step(mover: Amphipod) -> CostPerStep {
    match mover {
        b'A' => 1,
        b'B' => 10,
        b'C' => 100,
        b'D' => 1000,
        _ => panic!("Invalid mover: {}", mover as char),
    }
}

//        _____________
// Hall:  |01.2.3.4.56|
// Rooms: |-|0|0|0|0|-|
//          |1|1|1|1|
//          0-1-2-3-|
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct RoomState<const N: usize> {
    rooms: [[u8; N]; 4],
    hall: [u8; 7],
}

const fn new_state<const N: usize>(rooms: [[u8; N]; 4]) -> RoomState<N> {
    RoomState::<N> {
        rooms: rooms,
        hall: [0; 7],
    }
}

const GOAL_1: RoomState<2> = new_state([[b'A'; 2], [b'B'; 2], [b'C'; 2], [b'D'; 2]]);
const GOAL_2: RoomState<4> = new_state([[b'A'; 4], [b'B'; 4], [b'C'; 4], [b'D'; 4]]);

impl<const N: usize> RoomState<N> {
    fn room_to_entrance(
        &self,
        room_idx: usize,
        room_to_room: bool,
    ) -> Option<(Steps, Amphipod, RoomState<N>)> {
        let room = self.rooms[room_idx];
        let from = room.iter().position(|&x| x != 0)?;
        let mover = room[from];
        if (mover - b'A') as usize == room_idx && (room[N - 1] == mover || room_to_room) {
            // In the right room, have no visitors or we're planning to go directly to
            //  our destination, which we're in. No reason to move.
            None
        } else {
            let steps = from + 1;
            let mut newstate = self.clone();
            newstate.rooms[room_idx][from] = 0;
            Some((steps, mover, newstate))
        }
    }

    fn entrance_to_room(&self, mover: Amphipod) -> Option<(Steps, RoomState<N>)> {
        let room_idx = (mover - b'A') as usize;
        let room = self.rooms[room_idx];
        let tgt = N - 1 - room.iter().rev().position(|&x| x == 0)?;
        assert_eq!(room[tgt], 0);
        if room.iter().any(|&x| x != 0 && x != mover) {
            // Visitors in the room.
            None
        } else {
            let mut newstate = self.clone();
            newstate.rooms[room_idx][tgt] = mover;
            Some((tgt + 1, newstate))
        }
    }

    fn _entrance_to_hall_steps(&self, room: usize, hall: usize) -> Option<Steps> {
        assert!(hall < 7);
        assert!(room < 4);
        let min_hall_coord = if hall < room + 2 { hall } else { room + 2 };
        let max_hall_coord = if hall > room + 1 { hall } else { room + 1 };
        let end_of_hall_step = if hall == 0 || hall == 6 { 1 } else { 0 };
        if (min_hall_coord..=max_hall_coord).all(|x| self.hall[x] == 0) {
            Some((max_hall_coord - min_hall_coord) * 2 + 1 - end_of_hall_step)
        } else {
            None
        }
    }

    fn _hall_to_entrance_steps(&self, room: usize, hall: usize) -> Option<Steps> {
        assert!(hall < 7);
        assert!(room < 4);
        if room + 1 == hall || room + 2 == hall {
            Some(1)
        } else if hall < room + 1 {
            if self.hall[hall + 1] == 0 {
                self._hall_to_entrance_steps(room, hall + 1)
                    .map(|x| x + 1 + ((hall > 0) as usize))
            } else {
                None
            }
        } else {
            // hall > room + 2
            if self.hall[hall - 1] == 0 {
                self._hall_to_entrance_steps(room, hall - 1)
                    .map(|x| x + 1 + ((hall < 6) as usize))
            } else {
                None
            }
        }
    }

    fn room_to_hall(&self, room: usize, hall: usize) -> Option<(Steps, Amphipod, RoomState<N>)> {
        assert!(hall < 7);
        assert!(room < 4);
        self.room_to_entrance(room, false)
            .and_then(|(steps_out, mover, mut newstate)| {
                if (mover - b'A') as usize == room {
                    // We must be expelling a visitor. If that's the case,
                    // we cannot move to block the visitor we're expelling
                    // from getting home.
                    // let visitor = newstate.rooms[room][1];
                    // assert_ne!(visitor, 0);
                    // assert_ne!(visitor, mover);
                    // if visitor < mover && hall <= room + 1 {
                    //     return None;
                    // } else if visitor > mover && hall >= room + 2 {
                    //     return None;
                    // }
                    // Actually we can, if we want to kick them out and go straight home.
                }
                newstate
                    ._entrance_to_hall_steps(room, hall)
                    .map(|steps_across| {
                        newstate.hall[hall] = mover;
                        (steps_out + steps_across, mover, newstate)
                    })
            })
    }

    fn hall_to_room(&self, hall: usize) -> Option<(Steps, Amphipod, RoomState<N>)> {
        assert!(hall < 7);
        let mover = self.hall[hall];
        assert!(mover > 0);
        let room = (mover - b'A') as usize;
        assert!(room < 4);
        self._hall_to_entrance_steps(room, hall)
            .and_then(|steps_across| {
                let mut newstate = self.clone();
                newstate.hall[hall] = 0;
                newstate
                    .entrance_to_room(mover)
                    .map(|(steps_in, newstate)| (steps_across + steps_in, mover, newstate))
            })
    }

    fn _entrance_to_entrance_steps(&self, room1: usize, mover: Amphipod) -> Option<Steps> {
        let room2 = (mover - b'A') as usize;
        assert!(room1 < 4);
        assert!(room2 < 4);
        let max = usize::max(room1, room2);
        let min = usize::min(room1, room2);
        if self.hall[min + 2..=max + 1].iter().all(|&x| x == 0) {
            Some((max - min) * 2)
        } else {
            None
        }
    }

    fn room_to_room(&self, room1: usize) -> Option<(Cost, Amphipod, RoomState<N>)> {
        assert!(room1 < 4);
        self.room_to_entrance(room1, true)
            .and_then(|(steps_out, mover, newstate)| {
                self._entrance_to_entrance_steps(room1, mover)
                    .and_then(|steps_across| {
                        newstate
                            .entrance_to_room(mover)
                            .map(|(steps_in, newstate)| {
                                (steps_out + steps_across + steps_in, mover, newstate)
                            })
                    })
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State<const N: usize> {
    cost: usize,
    state: RoomState<N>,
}

impl<const N: usize> Ord for State<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.state.cmp(&other.state))
    }
}

impl<const N: usize> PartialOrd for State<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path_cost<const N: usize>(
    input: RoomState<N>,
    goal: RoomState<N>,
) -> Result<usize, String> {
    let mut dist: HashMap<RoomState<N>, usize> = HashMap::new();
    let mut heap: BinaryHeap<State<N>> = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        state: input,
    });

    'optloop: while let Some(State { cost, state }) = heap.pop() {
        // println!("{:?}", State { cost, state });
        if state == goal {
            return Ok(cost);
        }
        if dist.get(&state).copied().unwrap_or(usize::MAX) < cost {
            continue;
        }

        // Check for greedy, always-optimal moves. If we find some, only take that move.
        // First, can we move an amphipod from its current room to its destination room?
        for (steps, mover, newstate) in (0..state.rooms.len())
            .into_iter()
            .filter_map(|i| state.room_to_room(i))
        {
            let newcost = cost + steps * cost_per_step(mover);
            if dist.get(&newstate).map(|&x| x <= newcost).unwrap_or(false) {
                continue;
            } else {
                dist.insert(newstate, newcost);
                heap.push(State {
                    cost: newcost,
                    state: newstate,
                });
                continue 'optloop;
            }
        }

        // Next, can we move an amphipod from the hall to its destination room?
        for i in (0..state.hall.len())
            .into_iter()
            .filter(|&i| state.hall[i] != 0)
        {
            if let Some((newsteps, mover, newstate)) = state.hall_to_room(i) {
                let newcost = cost + newsteps * cost_per_step(mover);
                if dist.get(&newstate).map(|&x| x <= newcost).unwrap_or(false) {
                    continue;
                } else {
                    dist.insert(newstate, newcost);
                    heap.push(State {
                        cost: newcost,
                        state: newstate,
                    });
                    continue 'optloop;
                }
            }
        }

        // Our optimal approaches have failed, so now we need to brute force all possible hallway moves.
        for i in 0..state.rooms.len() {
            for j in 0..state.hall.len() {
                if let Some((newsteps, mover, newstate)) = state.room_to_hall(i, j) {
                    let newcost = cost + newsteps * cost_per_step(mover);
                    if dist.get(&newstate).map(|&x| x <= newcost).unwrap_or(false) {
                        continue;
                    } else {
                        dist.insert(newstate, newcost);
                        heap.push(State {
                            cost: newcost,
                            state: newstate,
                        });
                    }
                }
            }
        }
    }

    Err("No path found.".to_string())
}

// INPUT:
// #############
// #...........#
// ###D#C#A#B###
//   #D#C#B#A#
//   #########
const INPUT_1: RoomState<2> = new_state([[b'D', b'D'], [b'C', b'C'], [b'A', b'B'], [b'B', b'A']]);

// PART2 INPUT:
// #############
// #...........#
// ###D#C#A#B###
//   #D#C#B#A#
//   #D#B#A#C#
//   #D#C#B#A#
//   #########
const INPUT_2: RoomState<4> = new_state([
    [b'D', b'D', b'D', b'D'],
    [b'C', b'C', b'B', b'C'],
    [b'A', b'B', b'A', b'B'],
    [b'B', b'A', b'C', b'A'],
]);

fn main() {
    println!("Part 1: {}", shortest_path_cost(INPUT_1, GOAL_1).unwrap());
    println!("Part 2: {}", shortest_path_cost(INPUT_2, GOAL_2).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_path_cost_0() {
        const INPUT: RoomState<2> = new_state([[0, 0], [0, 0], [0, 0], [0, b'A']]);
        const GOAL: RoomState<2> = new_state([[0, b'A'], [0, 0], [0, 0], [0, 0]]);
        assert_eq!(shortest_path_cost(INPUT, GOAL).unwrap(), 10);
    }

    #[test]
    fn test_shortest_path_cost_1() {
        const INPUT: RoomState<2> = new_state([[0, b'A'], [0, 0], [0, 0], [0, b'A']]);
        const GOAL: RoomState<2> = new_state([[b'A', b'A'], [0, 0], [0, 0], [0, 0]]);
        assert_eq!(shortest_path_cost(INPUT, GOAL).unwrap(), 9);
    }

    #[test]
    fn test_visitor_expel() {
        const INPUT: RoomState<2> = new_state([[0, b'A'], [0, 0], [0, 0], [b'D', b'A']]);
        const GOAL: RoomState<2> = new_state([[b'A', b'A'], [0, 0], [0, 0], [0, b'D']]);
        assert_eq!(
            shortest_path_cost(INPUT, GOAL).unwrap(),
            9 + 5 * cost_per_step(b'D')
        );
    }

    #[test]
    fn test_visitor_expel2() {
        const INPUT: RoomState<2> = new_state([[0, 0], [0, 0], [0, 0], [b'D', b'A']]);
        const GOAL: RoomState<2> = new_state([[0, b'A'], [0, 0], [0, 0], [0, b'D']]);
        assert_eq!(
            shortest_path_cost(INPUT, GOAL).unwrap(),
            10 + 5 * cost_per_step(b'D')
        );
    }

    #[test]
    fn test_hall_to_room_1() {
        const INPUT: RoomState<2> = RoomState {
            rooms: [[0, 0], [0, 0], [0, 0], [0, b'D']],
            hall: [0, 0, 0, 0, 0, b'D', 0],
        };
        const GOAL: RoomState<2> = new_state([[0, 0], [0, 0], [0, 0], [b'D', b'D']]);
        assert_eq!(INPUT.hall_to_room(5).unwrap(), (2, b'D', GOAL));
    }

    #[test]
    fn test_shortest_path_hall_to_room_1() {
        const INPUT: RoomState<2> = RoomState {
            rooms: [[0, 0], [0, 0], [0, 0], [0, b'D']],
            hall: [0, 0, 0, 0, 0, b'D', 0],
        };
        const GOAL: RoomState<2> = new_state([[0, 0], [0, 0], [0, 0], [b'D', b'D']]);
        assert_eq!(
            shortest_path_cost(INPUT, GOAL).unwrap(),
            2 * cost_per_step(b'D')
        );
    }

    #[test]
    fn test_hall_to_room_2() {
        const INPUT: RoomState<2> = RoomState {
            rooms: [[0, 0], [0, 0], [0, 0], [0, 0]],
            hall: [0, 0, 0, 0, 0, b'D', 0],
        };
        const GOAL: RoomState<2> = new_state([[0, 0], [0, 0], [0, 0], [0, b'D']]);
        assert_eq!(INPUT.hall_to_room(5).unwrap(), (3, b'D', GOAL));
    }

    #[test]
    fn test_shortest_path_hall_to_room_2() {
        const INPUT: RoomState<2> = RoomState {
            rooms: [[0, 0], [0, 0], [0, 0], [0, 0]],
            hall: [0, 0, 0, 0, 0, b'D', 0],
        };
        const GOAL: RoomState<2> = new_state([[0, 0], [0, 0], [0, 0], [0, b'D']]);
        assert_eq!(
            shortest_path_cost(INPUT, GOAL).unwrap(),
            3 * cost_per_step(b'D')
        );
    }

    #[test]
    fn test_entrance_to_hall_steps() {
        const INPUT: RoomState<2> = RoomState {
            rooms: [[0, 0], [0, 0], [0, 0], [0, 0]],
            hall: [0, 0, 0, 0, 0, 0, 0],
        };
        assert_eq!(INPUT._entrance_to_hall_steps(0, 0).unwrap(), 2);
        assert_eq!(INPUT._entrance_to_hall_steps(0, 1).unwrap(), 1);
        assert_eq!(INPUT._entrance_to_hall_steps(0, 2).unwrap(), 1);
        assert_eq!(INPUT._entrance_to_hall_steps(0, 3).unwrap(), 3);
        assert_eq!(INPUT._entrance_to_hall_steps(0, 4).unwrap(), 5);
        assert_eq!(INPUT._entrance_to_hall_steps(0, 5).unwrap(), 7);
        assert_eq!(INPUT._entrance_to_hall_steps(0, 6).unwrap(), 8);
        assert_eq!(INPUT._entrance_to_hall_steps(1, 0).unwrap(), 4);
        assert_eq!(INPUT._entrance_to_hall_steps(1, 1).unwrap(), 3);
        assert_eq!(INPUT._entrance_to_hall_steps(1, 2).unwrap(), 1);
        assert_eq!(INPUT._entrance_to_hall_steps(1, 3).unwrap(), 1);
        assert_eq!(INPUT._entrance_to_hall_steps(1, 4).unwrap(), 3);
        assert_eq!(INPUT._entrance_to_hall_steps(1, 5).unwrap(), 5);
        assert_eq!(INPUT._entrance_to_hall_steps(1, 6).unwrap(), 6);
        assert_eq!(INPUT._entrance_to_hall_steps(2, 0).unwrap(), 6);
        assert_eq!(INPUT._entrance_to_hall_steps(2, 1).unwrap(), 5);
        assert_eq!(INPUT._entrance_to_hall_steps(2, 2).unwrap(), 3);
        assert_eq!(INPUT._entrance_to_hall_steps(2, 3).unwrap(), 1);
        assert_eq!(INPUT._entrance_to_hall_steps(2, 4).unwrap(), 1);
        assert_eq!(INPUT._entrance_to_hall_steps(2, 5).unwrap(), 3);
        assert_eq!(INPUT._entrance_to_hall_steps(2, 6).unwrap(), 4);
        assert_eq!(INPUT._entrance_to_hall_steps(3, 0).unwrap(), 8);
        assert_eq!(INPUT._entrance_to_hall_steps(3, 1).unwrap(), 7);
        assert_eq!(INPUT._entrance_to_hall_steps(3, 2).unwrap(), 5);
        assert_eq!(INPUT._entrance_to_hall_steps(3, 3).unwrap(), 3);
        assert_eq!(INPUT._entrance_to_hall_steps(3, 4).unwrap(), 1);
        assert_eq!(INPUT._entrance_to_hall_steps(3, 5).unwrap(), 1);
        assert_eq!(INPUT._entrance_to_hall_steps(3, 6).unwrap(), 2);
    }

    #[test]
    fn test_room_to_room_null_move() {
        const INPUT0: RoomState<2> = RoomState {
            rooms: [[0, b'A'], [0, b'B'], [0, b'C'], [0, b'D']],
            hall: [0, 0, 0, 0, 0, 0, 0],
        };
        assert_eq!(INPUT0.room_to_room(0), None);
        assert_eq!(INPUT0.room_to_room(1), None);
        assert_eq!(INPUT0.room_to_room(2), None);
        assert_eq!(INPUT0.room_to_room(3), None);

        const INPUT1: RoomState<2> = RoomState {
            rooms: [[b'A'; 2], [b'B'; 2], [b'C'; 2], [b'D'; 2]],
            hall: [0, 0, 0, 0, 0, 0, 0],
        };
        assert_eq!(INPUT1.room_to_room(0), None);
        assert_eq!(INPUT1.room_to_room(1), None);
        assert_eq!(INPUT1.room_to_room(2), None);
        assert_eq!(INPUT1.room_to_room(3), None);
    }

    #[test]
    fn test_example_input() {
        // EXAMPLE INPUT:
        // #############
        // #...........#
        // ###B#C#B#D###
        //   #A#D#C#A#
        //   #########
        const EXAMPLE_INPUT: RoomState<2> =
            new_state([[b'B', b'A'], [b'C', b'D'], [b'B', b'C'], [b'D', b'A']]);
        assert_eq!(shortest_path_cost(EXAMPLE_INPUT, GOAL_1).unwrap(), 12521);
    }

    #[test]
    fn test_example_paredown1() {
        // EXAMPLE INPUT:
        // #############
        // #...........#
        // ###B#C#B#.###
        //   #A#D#C#A#
        //   #########
        const EXAMPLE_INPUT: RoomState<2> =
            new_state([[b'B', b'A'], [b'C', b'D'], [b'B', b'C'], [0, b'A']]);

        const GOAL_1: RoomState<2> = new_state([[b'A'; 2], [b'B'; 2], [b'C'; 2], [0, b'D']]);
        assert_eq!(shortest_path_cost(EXAMPLE_INPUT, GOAL_1).unwrap(), 8521);
    }

    #[test]
    fn test_part2_example_input() {
        // EXAMPLE INPUT:
        // #############
        // #...........#
        // ###B#C#B#D###
        //   #D#C#B#A#
        //   #D#B#A#C#
        //   #A#D#C#A#
        //   #########
        const EXAMPLE_INPUT: RoomState<4> = new_state([
            [b'B', b'D', b'D', b'A'],
            [b'C', b'C', b'B', b'D'],
            [b'B', b'B', b'A', b'C'],
            [b'D', b'A', b'C', b'A'],
        ]);
        assert_eq!(shortest_path_cost(EXAMPLE_INPUT, GOAL_2).unwrap(), 44169);
    }

    #[test]
    fn test_part2_example_paredown1() {
        // EXAMPLE INPUT:
        // #############
        // #...........#
        // ###.#.#.#.###
        //   #.#.#.#.#
        //   #.#.#.#.#
        //   #A#D#C#A#
        //   #########
        const EXAMPLE_INPUT: RoomState<4> = new_state([
            [0, 0, 0, b'A'],
            [0, 0, 0, b'D'],
            [0, 0, 0, b'C'],
            [0, 0, 0, b'A'],
        ]);
        const GOAL: RoomState<4> =
            new_state([[0, 0, b'A', b'A'], [0; 4], [0, 0, 0, b'C'], [0, 0, 0, b'D']]);
        assert_eq!(
            shortest_path_cost(EXAMPLE_INPUT, GOAL).unwrap(),
            13 * cost_per_step(b'A') + 12 * cost_per_step(b'D')
        );
    }

    #[test]
    fn test_part2_example_paredown2() {
        // EXAMPLE INPUT:
        // #############
        // #...........#
        // ###.#.#.#.###
        //   #.#.#.#.#
        //   #.#.#.#.#
        //   #A#.#D#A#
        //   #########
        const EXAMPLE_INPUT: RoomState<4> = new_state([
            [0, 0, 0, b'A'],
            [0, 0, 0, 0],
            [0, 0, 0, b'D'],
            [0, 0, 0, b'A'],
        ]);
        const GOAL: RoomState<4> = new_state([[0, 0, b'A', b'A'], [0; 4], [0; 4], [0, 0, 0, b'D']]);
        assert_eq!(
            shortest_path_cost(EXAMPLE_INPUT, GOAL).unwrap(),
            13 * cost_per_step(b'A') + 10 * cost_per_step(b'D')
        );
    }
}
