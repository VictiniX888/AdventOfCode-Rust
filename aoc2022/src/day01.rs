use crate::*;

pub const SOLUTION: Solution = Solution {
    day: 1,
    solve: solve_optimized,
};

/* ======== OPTIMIZED ======== */
// (~50 us)
fn solve_optimized(input: &str) -> AnswerSet {
    let iter = input
        .split_terminator("\n\n")
        .map(|group| group.lines().map(|n| n.parse::<u32>().unwrap()).sum());

    let (a, b, c) = iter.fold((0, 0, 0), |acc, n| rank(n, acc));

    AnswerSet {
        p1: Answer::U32(a),
        p2: Answer::U32(a + b + c),
    }
}

fn rank(n: u32, (a, b, c): (u32, u32, u32)) -> (u32, u32, u32) {
    match n {
        n if n >= a => (n, a, b),
        n if n >= b => (a, n, b),
        n if n >= c => (a, b, n),
        _ => (a, b, c),
    }
}

/* ======== FIRST ATTEMPT ======== */
// (~300 us)
fn solve(input: &str) -> AnswerSet {
    let input = parse_input(input);
    AnswerSet {
        p1: Answer::U32(part_1(&input)),
        p2: Answer::U32(part_2(&input)),
    }
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .split_terminator("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|n| n.parse::<u32>().expect("Failed to parse input as number"))
                .collect()
        })
        .collect()
}

fn part_1(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|group| group.iter().sum())
        .max()
        .expect("Empty input iterator")
}

fn part_2(input: &[Vec<u32>]) -> u32 {
    let mut calories: Vec<u32> = input.iter().map(|group| group.iter().sum()).collect();
    calories.sort();

    calories.iter().rev().take(3).sum()
}
