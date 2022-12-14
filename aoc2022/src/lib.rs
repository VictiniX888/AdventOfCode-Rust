#![allow(dead_code)]
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;

use std::{fmt::Display, fs};

pub enum Answer {
    String(String),
    U16(u16),
    U32(u32),
    U64(u64),
    Usize(usize),
    I32(i32),
}

impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::String(str) => str.clone(),
                Self::U16(num) => num.to_string(),
                Self::U32(num) => num.to_string(),
                Self::U64(num) => num.to_string(),
                Self::Usize(num) => num.to_string(),
                Self::I32(num) => num.to_string(),
            }
        )
    }
}

pub struct AnswerSet {
    pub p1: Answer,
    pub p2: Answer,
}

pub struct Solution {
    day: u8,
    pub solve: fn(&str) -> AnswerSet,
}

pub fn run(solution: Solution) -> AnswerSet {
    let input = read_input(solution.day);
    (solution.solve)(&input)
}

pub fn read_input(day: u8) -> String {
    fs::read_to_string(format!("input/day{:02}.txt", day)).expect("Failed to read input file")
}
