use std::{collections::BTreeMap, ops::RangeInclusive};

type TargetRange = RangeInclusive<i16>;

#[derive(Clone, Debug)]
struct Target {
    x: TargetRange,
    y: TargetRange,
}

impl Target {
    fn contains(&self, x: i16, y: i16) -> bool {
        self.x.contains(&x) && self.y.contains(&y)
    }
}

fn part1(input: Target) -> i16 {
    // First, check for some integer $v_x$ such that $\frac{v_x^2+v_x}{2}$ falls in the range `input.x`. This is necessary to give us arbitrary time with which to check for collisions. (If this condition is not found, then this solution does not operate.)
    let mut v_x = 1;
    while (v_x * v_x - v_x) / 2 < *input.x.start() {
        v_x += 1;
    }
    assert!(
        input.x.contains(&((v_x * v_x - v_x) / 2)),
        "Could not find a suitable v_x"
    );

    // Because the probe launches upward with speed $v_y$, when it returns to height 0 it will be going downward with speed $v_y$. We then know that it will move the entire remaining distance downward in one step, meaning we need a speed such that it moves \emph{exactly} the distance to the end of the range -- counterintutively the "start" because we want the most negative value.
    let v_y = input.y.start().abs();

    // Now we can compute the maximum height obtained by the probe.
    (v_y * v_y - v_y) / 2
}

// 1176 too low (used wrong end of target range)
// 2926 too high (used + instead of - in drag equation)
// 2850 correct

// fn part2overcomplicated(input: Target) -> usize {
//     let xmap = {
//         let min_settle_vx = {
//             let mut vx = 1;
//             while (vx * vx - vx) / 2 < *input.x.start() {
//                 vx += 1;
//             }
//             vx
//         };
//         let max_settle_vx = {
//             let mut vx = min_settle_vx;
//             while (vx * vx - vx) / 2 < *input.x.end() {
//                 vx += 1;
//             }
//             vx
//         };
//         let max_vx = *input.x.end();
//         let mut xmap_temp: HashMap<Option<i16>, Vec<i16>> = HashMap::new();
//         ((max_settle_vx + 1)..(max_vx))
//             .flat_map(|vx| (1..vx).map(|t| (t,vx)))
//             .filter(|(vx, t)| {
//                 let x = vx * t - t * (t - 1) / 2;
//                 input.x.contains(&x)
//             }).for_each(|(vx, t)| {
//                 xmap_temp.entry(Some(t)).or_insert(vec![]).push(vx);
//             });
//         xmap_temp
//     };

//     let ymap = {
//         let min_vy = *input.y.start();
//         let max_vy = *input.y.start().abs();

//     }
// }

fn part2(input: Target) -> usize {
    let min_vx: i16 = {
        let mut vx: i16 = 1;
        while vx * vx - vx * (vx - 1) / 2 < *input.x.start() {
            vx += 1;
        }
        vx
    };
    let max_vx: i16 = *input.x.end();
    let min_vy: i16 = *input.y.start();
    let max_vy: i16 = input.y.start().abs();

    // println!("{}..{} {}..{}", min_vx, max_vx, min_vy, max_vy);

    let mut velocities: BTreeMap<(i16, i16), Vec<i16>> = BTreeMap::new();
    for vx in min_vx..=max_vx {
        'yloop: for vy in min_vy..=max_vy {
            for t in 1.. {
                let tx = vx.min(t);
                let x = vx * tx - tx * (tx - 1) / 2;
                let y = vy * t - t * (t - 1) / 2;
                if input.contains(x, y) {
                    velocities.entry((vx, vy)).or_insert(vec![]).push(t);
                } else if y < *input.y.start() || x > *input.x.end() {
                    continue 'yloop;
                }
            }
        }
    }

    velocities.keys().count()
}

// 980 too low (used non-inclusive velocity ranges)
// 1009 too low (broke out of y loop too early)
// 1062 too low (incorrect estimate of min_vx)
// 1132 wrong (forgot to make velocities distinct)
// 1117 correct.

fn main() {
    // let exampleinput = Target {
    //     x: 20..=30,
    //     y: -10..=-5,
    // };
    let input = Target {
        x: 287..=309,
        y: -76..=-48,
    };
    println!("Part1: {}", part1(input.clone()));
    println!("Part2: {}", part2(input));
}
