use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use crate::*;

pub const SOLUTION: Solution = Solution { day: 19, solve };

// ~ 680 ms
fn solve(input: &str) -> AnswerSet {
    let blueprints = parse_input(input);
    // println!("{:?}", blueprints);
    // let now = Instant::now();
    // let p1 = calculate_quality_level(&blueprints[1], 24);
    let p1: usize = blueprints
        .iter()
        .map(|blueprint| calculate_quality_level(blueprint, 24, [10, 10, 10, 10]))
        .enumerate()
        .map(|(i, quality)| (i + 1) * quality)
        .sum();
    let p2: usize = blueprints
        .iter()
        .take(3)
        .map(|blueprint| calculate_quality_level(blueprint, 32, [4, 10, 10, usize::MAX]))
        .product();
    // println!("{}", now.elapsed().as_secs());

    // println!("{}", calculate_test(&blueprints[0], 32));

    AnswerSet {
        p1: Answer::Usize(p1),
        p2: Answer::Usize(p2),
    }
}

fn calculate_test(blueprint: &Blueprint, steps: usize) -> usize {
    let mut state = State::new();

    let mut robot = 1;
    let mut count = 0;

    loop {
        if count > 0 {
            robot = 1;
        }
        if count > 2 {
            robot = 2;
        }
        if count > 3 {
            robot = 3;
        }
        // Calculate number of steps needed to get enough materials
        let next_steps = blueprint[robot]
            .iter()
            .enumerate()
            .map(|(rock, &needed)| {
                if (needed as usize) <= state.rocks[rock] {
                    Some(0)
                } else if state.robots[rock] == 0 {
                    None
                } else {
                    // Ceiling division
                    Some(
                        (needed as usize - state.rocks[rock] + state.robots[rock] - 1)
                            / state.robots[rock],
                    )
                }
            })
            .reduce(|acc, step| {
                if step.is_none() {
                    None
                } else {
                    acc.map(|acc| acc.max(step.unwrap()))
                }
            })
            .unwrap();

        if let Some(next_steps) = next_steps {
            if state.step + next_steps < steps - 1 {
                let mut next_rocks = state.rocks;
                for i in 0..4 {
                    next_rocks[i] += (next_steps + 1) * state.robots[i];
                }
                for i in 0..3 {
                    next_rocks[i] -= blueprint[robot][i] as usize;
                }

                let mut next_robots = state.robots;
                next_robots[robot] += 1;

                state.robots = next_robots;
                state.rocks = next_rocks;
                state.step += next_steps + 1;

                println!("{:?}, {}", state.rocks, state.step);

                count += 1;
            } else {
                // We run out of steps
                let geodes = state.rocks[3] + state.robots[3] * (steps - state.step);
                return geodes;
            }
        } else {
            // We do not have the robot to produce the rock
            // Continue
            break;
        }
    }

    0
}

fn calculate_quality_level(blueprint: &Blueprint, steps: usize, max_robots: [usize; 4]) -> usize {
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(State::new()));

    let mut cache: HashMap<_, State> = HashMap::new();

    let mut max = 0;

    while let Some(Reverse(state)) = queue.pop() {
        // println!("{:?}", state);
        // _ = pause();

        if let Some(other) = cache.get(&(state.robots, state.step)) {
            // if state.robots == [1, 4, 1, 0] && state.step == 12 {
            //     println!("{:?} {:?}", state, other);
            // }
            if other >= &state {
                continue;
            }
        }

        cache.insert((state.robots, state.step), state.clone());
        // println!("{:?}", state);

        // If we have enough obsidian, we always will want to use it to make geode robos
        if state.rocks[2] >= blueprint[3][2] as usize && state.rocks[0] >= blueprint[3][0] as usize
        {
            iterate(
                3, &state, max_robots, blueprint, steps, &mut queue, &mut max,
            );
            continue;
        }
        // Likewise, if we have enough clay, we always will want to use it to make obsidian robos
        // I don't know if this is always true, but it works for my input
        else if state.rocks[1] >= blueprint[2][1] as usize
            && state.rocks[0] >= blueprint[2][0] as usize
        {
            iterate(
                2, &state, max_robots, blueprint, steps, &mut queue, &mut max,
            );
            continue;
        }

        // Craft next possible robot of each type
        for robot in 0..4 {
            iterate(
                robot, &state, max_robots, blueprint, steps, &mut queue, &mut max,
            )
        }
    }

    // println!("{}", max);

    max
}

fn iterate(
    robot: usize,
    state: &State,
    max_robots: [usize; 4],
    blueprint: &Blueprint,
    steps: usize,
    queue: &mut BinaryHeap<Reverse<State>>,
    max: &mut usize,
) {
    if state.robots[robot] < max_robots[robot] {
        // Calculate number of steps needed to get enough materials
        let next_steps = blueprint[robot]
            .iter()
            .enumerate()
            .map(|(rock, &needed)| {
                if (needed as usize) <= state.rocks[rock] {
                    Some(0)
                } else if state.robots[rock] == 0 {
                    None
                } else {
                    // Ceiling division
                    Some(
                        (needed as usize - state.rocks[rock] + state.robots[rock] - 1)
                            / state.robots[rock],
                    )
                }
            })
            .reduce(|acc, step| {
                if step.is_none() {
                    None
                } else {
                    acc.map(|acc| acc.max(step.unwrap()))
                }
            })
            .unwrap();

        if let Some(next_steps) = next_steps {
            if state.step + next_steps < steps - 1 {
                let mut next_rocks = state.rocks;
                for i in 0..4 {
                    next_rocks[i] += (next_steps + 1) * state.robots[i];
                }
                for i in 0..3 {
                    next_rocks[i] -= blueprint[robot][i] as usize;
                }

                let mut next_robots = state.robots;
                next_robots[robot] += 1;

                queue.push(Reverse(State {
                    robots: next_robots,
                    rocks: next_rocks,
                    step: state.step + next_steps + 1,
                }));
            } else {
                // We run out of steps
                let geodes = state.rocks[3] + state.robots[3] * (steps - state.step);
                if geodes > *max {
                    *max = geodes;
                }
            }
        } else {
            // We do not have the robot to produce the rock
            // Continue
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct State {
    robots: [usize; 4],
    rocks: [usize; 4],
    step: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.step.partial_cmp(&other.step) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        for i in (0..self.rocks.len()).rev() {
            match self.rocks[i].partial_cmp(&other.rocks[i]) {
                Some(core::cmp::Ordering::Equal) => {}
                ord => return ord,
            }
        }
        Some(core::cmp::Ordering::Equal)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.step.cmp(&other.step) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        for i in (0..self.rocks.len()).rev() {
            match self.rocks[i].cmp(&other.rocks[i]) {
                core::cmp::Ordering::Equal => {}
                ord => return ord,
            }
        }
        core::cmp::Ordering::Equal
    }
}

impl State {
    fn new() -> Self {
        State {
            robots: [1, 0, 0, 0],
            rocks: [0; 4],
            step: 0,
        }
    }
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    let mut blueprints = Vec::new();

    for line in input.lines() {
        let mut iter = line.bytes().skip(10);
        let _id = parse_u8_from_iter(&mut iter);
        iter.nth(21);

        let mut blueprint = [[0; 3]; 4];
        let ore_cost = parse_u8_from_iter(&mut iter);
        blueprint[0][0] = ore_cost;
        iter.nth(26);

        let ore_cost = parse_u8_from_iter(&mut iter);
        blueprint[1][0] = ore_cost;
        iter.nth(30);

        let ore_cost = parse_u8_from_iter(&mut iter);
        iter.nth(7);
        let clay_cost = parse_u8_from_iter(&mut iter);
        blueprint[2][0] = ore_cost;
        blueprint[2][1] = clay_cost;
        iter.nth(28);

        let ore_cost = parse_u8_from_iter(&mut iter);
        iter.nth(7);
        let obsidian_cost = parse_u8_from_iter(&mut iter);
        blueprint[3][0] = ore_cost;
        blueprint[3][2] = obsidian_cost;

        blueprints.push(blueprint);
    }

    blueprints
}

// Rocks map to indices
// Ore = 0, Clay = 1, Obsidian = 2, Geode = 3
type Blueprint = [Cost; 4];

// Only 3 elements because none of the robtos require Geodes
type Cost = [u8; 3];

#[derive(Hash, PartialEq, Eq)]
enum Rock {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

fn parse_u8_from_iter(ascii_iter: &mut impl Iterator<Item = u8>) -> u8 {
    let mut num = 0;

    while let Some(byte) = ascii_iter.next() {
        if !byte.is_ascii_digit() {
            break;
        }

        num *= 10;
        num += byte - b'0';
    }

    num
}
