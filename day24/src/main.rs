use std::{collections::HashMap, str::Lines};

type Aluwidth = isize;

const CHUNK_COUNT: usize = 14;

const fn chunk(params: [i8; 3], input: i8, mut z: Aluwidth) -> Aluwidth {
    // inp w // Here, w remains input the whole time.
    // mul x 0
    // add x z
    // mod x 26
    let mut x = z % 26;

    // div z param0
    z /= params[0] as Aluwidth;

    // add x param1
    // eql x w
    // eql x 0
    x = (x + params[1] as Aluwidth != input as Aluwidth) as Aluwidth;

    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y
    z *= x * 25 + 1;
    // mul y 0
    // add y w
    // add y param2
    // mul y x
    // add z y
    z += x * (input + params[2]) as Aluwidth;
    z
}

const fn chunk_v2(params: [i8; 3], input: i8, z: Aluwidth) -> Aluwidth {
    if (z % 26) as i8 + params[1] != input {
        z / params[0] as Aluwidth * 26 + (input + params[2]) as Aluwidth
    } else {
        z / params[0] as Aluwidth
    }
}

fn parse_params_from_chunk(input: &mut Lines) -> Result<[i8; 3], String> {
    let result = Ok([
        input
            .skip(4)
            .next()
            .map(|s| s[6..].parse::<i8>().map_err(|e| e.to_string()))
            .unwrap_or(Err("Parameter 0 not in expected location.".to_owned()))?,
        input
            .next()
            .map(|s| s[6..].parse::<i8>().map_err(|e| e.to_string()))
            .unwrap_or(Err("Parameter 1 not in expected location.".to_owned()))?,
        input
            .skip(9)
            .next()
            .map(|s| s[6..].parse::<i8>().map_err(|e| e.to_string()))
            .unwrap_or(Err("Parameter 2 not in expected location.".to_owned()))?,
    ]);
    assert_eq!(input.next(), Some("mul y x"));
    assert_eq!(input.next(), Some("add z y"));

    result
}

fn parse_paramlist(input: &mut Lines) -> Result<[[i8; 3]; 14], String> {
    let mut result = [[0i8; 3]; CHUNK_COUNT];
    for i in 0..CHUNK_COUNT {
        result[i] = parse_params_from_chunk(input)?;
    }
    Ok(result)
}

fn _part1_backward(params: [[i8; 3]; CHUNK_COUNT]) {
    let mut acceptable_zs: HashMap<(u8, Aluwidth), Vec<i8>> = HashMap::new();

    for chunk_idx in (0..CHUNK_COUNT as u8).rev() {
        for i in 1..9 {
            for z_in in if chunk_idx > 0 { -256..=255 } else { 0..=0 } {
                let z_out = chunk(params[chunk_idx as usize], i, z_in);
                if chunk_idx as usize >= CHUNK_COUNT - 1 {
                    if z_out == 0 {
                        acceptable_zs
                            .entry((chunk_idx, z_in))
                            .or_insert(vec![])
                            .push(i);
                    }
                } else {
                    if acceptable_zs.contains_key(&(chunk_idx + 1, z_out)) {
                        acceptable_zs
                            .entry((chunk_idx, z_in))
                            .or_insert(vec![])
                            .push(i);
                    }
                }
            }
        }
    }

    let mut results = acceptable_zs
        .into_iter()
        .flat_map(|((chunk_idx, z_in), inputs)| {
            inputs.into_iter().map(move |i| (chunk_idx, i, z_in))
        })
        .collect::<Vec<_>>();
    results.sort();
    println!("{:?}", results);
}

fn _part1_brute_force(params: [[i8; 3]; CHUNK_COUNT]) -> String {
    let mut input = [9i8; CHUNK_COUNT];
    loop {
        let z = {
            let mut z: Aluwidth = 0;
            let mut i = 0;
            loop {
                z = chunk(params[i], input[i], z);
                i += 1;
                if i >= CHUNK_COUNT {
                    break z;
                }
            }
        };
        if z == 0 {
            break;
        }
        println!("Tried: {:?}, got {}", input, z);
        for i in input.iter_mut().rev() {
            *i -= 1;
            if *i > 0 {
                break;
            }
        }

        if input[0] < 1 {
            return "No solution found.".to_owned();
        } else {
            for i in input.iter_mut() {
                if *i < 1 {
                    *i = 9;
                }
            }
        }
    }

    input
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join("")
}

fn _part1_forward(params: [[i8; 3]; CHUNK_COUNT]) -> Aluwidth {
    let mut z_values: HashMap<Aluwidth, Aluwidth> = [(0, 0)].into_iter().collect();
    let mut next_z_values: HashMap<Aluwidth, Aluwidth> = HashMap::new();

    for index in 0..CHUNK_COUNT {
        println!("Chunk {}: {} inputs", index, z_values.len());
        if !z_values.keys().any(|&z| z <= 26) {
            println!("Warning: No likely z values found.");
        }
        for (z_in, candidate_prev) in z_values.drain() {
            for digit in 1..=9 {
                let z = chunk_v2(params[index], digit, z_in);
                let candidate = candidate_prev * 10 + digit as isize;
                next_z_values
                    .entry(z)
                    .and_modify(|c| *c = std::cmp::max(*c, candidate))
                    .or_insert(candidate);
            }
        }
        z_values = std::mem::take(&mut next_z_values);
    }
    return z_values[&0];
}

fn part1(params: [[i8; 3]; CHUNK_COUNT]) -> String {
    // The chunks with param[0] == 1 are effectively "pushing" a base 26 number to a stack.
    // The chunks with param[0] == 26 pop from that stack if the condition is met.
    // We want the stack to be 0 at the end, which only happens if the stack is empty.
    // Ergo, we only need to optimize each related pair (push and pop) of chunks.
    let pairs = [[0, 3], [1, 5], [2, 9], [4, 10], [6, 11], [7, 12], [8, 13]];
    let mut stack_head = 0;
    for [push, pop] in pairs {
        assert_eq!(params[push][0], 1);
        assert_eq!(params[pop][0], 26);
    }

    "Unimplemented.".to_owned()
}

// 99299513899971 correct

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    let params = parse_paramlist(INPUT.lines().by_ref()).expect("Failed to parse input.");
    println!("Part 1: {}", _part1_forward(params));
}

#[cfg(test)]
mod test {
    #[test]
    fn test_chunkv2_vs_chunk() {
        use super::*;
        for i in [1, 26] {
            for j in -9..16 {
                for k in 1..14 {
                    let params = [i, j, k];
                    for input in 1..9 {
                        for z in 0..26 {
                            assert_eq!(
                                chunk(params, input, z),
                                chunk_v2(params, input, z),
                                "{:?}, input={}, z={}",
                                params,
                                input,
                                z
                            );
                        }
                    }
                }
            }
        }
    }
}
