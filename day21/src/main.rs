fn part1(mut state: [u8; 2]) -> usize {
    let mut rolls: usize = 0;
    let mut playerscores = [0usize; 2];

    let mut roll = || {
        let roll = ((rolls % 100) as u8) + 1;
        rolls += 1;
        roll
    };

    let mut step = |state: u8| -> u8 {
        ((state as u16 + roll() as u16 + roll() as u16 + roll() as u16) % 10) as u8
    };

    while !playerscores.iter().any(|&s| s >= 1000usize) {
        // player 0 move:
        state[0] = step(state[0]);
        playerscores[0] += state[0] as usize;

        if playerscores[0] >= 1000 {
            break;
        }

        // player 1 move:
        state[1] = step(state[1]);
        playerscores[1] += state[1] as usize;
    }

    playerscores.iter().min().unwrap() * rolls
}

// 897396 too low (accidentally added 1 to each state)
// 998088 correct

fn die_spaces_moved() -> [u8; 7] {
    let mut state = [0u8; 7];
    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1u8..=3 {
                state[(i + j + k) as usize - 3] += 1;
            }
        }
    }
    state
}

fn part2(starting_state: [u8; 2]) -> u128 {
    // Each roll of the die creates 3 universes: 1, 2, and 3.
    // Each player move rolls the die 3 times, creating 3*3*3=27 possible universes.
    // These universes are divided thusly:
    //  Spaces moved: 0 1 2 3 4 5 6 7 8 9
    //     Universes: 0 0 0 1 3 6 7 6 3 1
    const UNIVERSES: [u128; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];
    // All universes on each space end exactly the same way and are thus equivalent.
    // How we represent them is important, though. Moving 3 then 7 is not the same as moving 7 then 3.
    // Ah! However, there are only 22 states for each player score and 10 states for each player position.
    // 22*22*10*10 = 484000 states. Anything landing on an identical state here is a duplicate.
    // In addition, if a player reaches 21, it doesn't matter which universes led to it, just their winning state.
    // So 44,100 states remain. That's a lot, but it's doable compared to where we started.
    const STATE_COUNT: usize = 21 * 21 * 10 * 10;
    let mut states = [0u128; STATE_COUNT];

    fn id_to_state(id: usize) -> ([u8; 2], [u8; 2]) {
        let player0score = (id % 21) as u8;
        let player1score = ((id / 21) % 21) as u8;
        let player0pos = ((id / 441) % 10) as u8 + 1;
        let player1pos = (id / 4410) as u8 + 1;
        assert!(player1pos <= 10);
        ([player0score, player1score], [player0pos, player1pos])
    }

    fn state_to_id(state: ([u8; 2], [u8; 2])) -> usize {
        let [player0score, player1score] = state.0;
        let [player0pos, player1pos] = state.1;
        assert!(player1pos <= 10);
        (player0score as usize)
            + (player1score as usize) * 21
            + (player0pos as usize - 1) * 441
            + (player1pos as usize - 1) * 4410
    }

    // Enter the first universe:
    states[state_to_id(([0u8, 0], starting_state))] = 1;

    let mut wins: [u128; 2] = [0, 0];
    let mut turn: usize = 0;

    // The shape of this while loop has the effect of sweeping lower states upward, hopefully.
    while states.iter().any(|&s| s > 0) {
        let mut newstates = [0u128; STATE_COUNT];

        for id in (0..STATE_COUNT).filter(|&id| states[id] > 0) {
            let (scores, positions) = id_to_state(id);

            for movement in 3..=9 {
                let newposition = ((positions[turn] + movement - 1) % 10) + 1;
                let newscore = scores[turn] + newposition;
                if newscore >= 21 {
                    wins[turn] += states[id] * UNIVERSES[movement as usize];
                } else {
                    let newid = if turn == 0 {
                        state_to_id(([newscore, scores[1]], [newposition, positions[1]]))
                    } else {
                        state_to_id(([scores[0], newscore], [positions[0], newposition]))
                    };
                    newstates[newid] += states[id] * UNIVERSES[movement as usize];
                }
            }
        }

        states = newstates;
        turn = (turn + 1) % 2;
    }

    wins.into_iter().max().unwrap()
}

// 100587444684005 too low
// 7969212689404986386228535 too high (accidentally reduced players with score 20 back to 0 in some cases.)
// 1213276123124888 too high (accidentally moved all players one space too many each step.)
// 306621346123766 correct.

fn main() {
    let input = [9u8, 4u8];

    println!("Part 1: {}", part1(input.clone()));
    println!("Die spaces moved: {:?}", die_spaces_moved());
    println!("Part 2: {}", part2(input));
}
