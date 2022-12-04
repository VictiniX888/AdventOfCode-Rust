use crate::{Answer, AnswerSet, Solution};

pub const SOLUTION: Solution<fn(&str) -> AnswerSet> = Solution { day: 1, solve };

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
