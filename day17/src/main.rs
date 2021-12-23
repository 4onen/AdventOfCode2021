use std::ops::RangeInclusive;

struct Target {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
}

fn part1(input: Target) -> i32 {
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

fn main() {
    let input = Target {
        x: 287..=309,
        y: -76..=-48,
    };
    println!("Part1: {}", part1(input));
}
