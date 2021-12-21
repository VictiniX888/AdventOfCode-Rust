use std::collections::HashMap;

use aoc;

fn main() {
    let input = aoc::read_input(21);
    let mut input = input
        .lines()
        .map(|s| s.split_whitespace().last().unwrap().parse().unwrap());
    let p1_start: usize = input.next().unwrap();
    let p2_start: usize = input.next().unwrap();

    println!("{}", part_1(p1_start, p2_start));
    println!("{}", part_2(p1_start, p2_start));
}

// PART 1
fn part_1(p1_start: usize, p2_start: usize) -> usize {
    let mut dice = 0;
    let mut pos = [p1_start - 1, p2_start - 1];
    let mut points = [0, 0];
    let mut i = 0;

    loop {
        for _ in 0..3 {
            pos[i] = (pos[i] + (dice % 100) + 1) % 10;
            dice += 1;
        }

        points[i] += pos[i] + 1;
        i = (i + 1) % 2;

        if points[(i + 1) % 2] >= 1000 {
            return points[i] * dice;
        }
    }
}

// PART 2
fn part_2(p1_start: usize, p2_start: usize) -> usize {
    step(
        State {
            pos: [p1_start - 1, p2_start - 1],
            points: [0, 0],
            i: 0,
        },
        &mut HashMap::new(),
    )
    .into_iter()
    .max()
    .unwrap()
}

fn step(state: State, cache: &mut HashMap<State, [usize; 2]>) -> [usize; 2] {
    if let Some(&outcomes) = cache.get(&state) {
        return outcomes;
    }

    // sum up all outcomes of rolling 3-sided die 3 times
    let outcomes = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]
        .iter()
        .map(|&(roll, times)| {
            let mut pos = state.pos;
            pos[state.i] = (pos[state.i] + roll) % 10;
            let mut points = state.points;
            points[state.i] += pos[state.i] + 1;

            if points[state.i] >= 21 {
                let mut outcome = [0, 0];
                outcome[state.i] = times;
                outcome
            } else {
                scalar_mul(
                    times,
                    step(
                        State {
                            pos,
                            points,
                            i: (state.i + 1) % 2,
                        },
                        cache,
                    ),
                )
            }
        })
        .reduce(|acc, outcomes| zip_add(acc, outcomes))
        .unwrap();

    cache.insert(state, outcomes);

    outcomes
}

// these functions are not generic because they don't have to be
fn zip_add(a: [usize; 2], b: [usize; 2]) -> [usize; 2] {
    [a[0] + b[0], a[1] + b[1]]
}

fn scalar_mul(scalar: usize, array: [usize; 2]) -> [usize; 2] {
    [array[0] * scalar, array[1] * scalar]
}

#[derive(Hash, PartialEq, Eq)]
struct State {
    pos: [usize; 2],
    points: [usize; 2],
    i: usize,
}
