#[allow(dead_code)]
pub mod day01;

use std::{fmt::Display, fs};

pub enum Answer {
    String(String),
    U32(u32),
}

impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::String(str) => str.clone(),
                Self::U32(num) => num.to_string(),
            }
        )
    }
}

pub struct AnswerSet {
    pub p1: Answer,
    pub p2: Answer,
}

pub struct Solution<F>
where
    F: Fn(&str) -> AnswerSet,
{
    day: u8,
    pub solve: F,
}

pub fn run<F>(solution: Solution<F>) -> AnswerSet
where
    F: Fn(&str) -> AnswerSet,
{
    let input = read_input(solution.day);
    (solution.solve)(&input)
}

pub fn read_input(day: u8) -> String {
    fs::read_to_string(format!("input/day{:02}.txt", day)).expect("Failed to read input file")
}
