use std::{collections::VecDeque, str::from_utf8};

use crate::*;

pub const SOLUTION: Solution = Solution { day: 5, solve };

// ~ 58 us
fn solve(input: &str) -> AnswerSet {
    let (mut stack_p1, instructions) = parse_input(input);
    let mut stack_p2 = stack_p1.clone();

    for (amount, src, dest) in instructions {
        // Part 1
        for _ in 0..amount {
            let item = stack_p1[src as usize].pop_back().unwrap();
            stack_p1[dest as usize].push_back(item);
        }

        // Part 2
        let len = stack_p2[src as usize].len();
        for i in 0..amount {
            let item = stack_p2[src as usize][len - (amount - i) as usize];
            stack_p2[dest as usize].push_back(item);
        }
        stack_p2[src as usize].truncate(len - amount as usize);
    }

    let p1: Vec<u8> = stack_p1.iter().map(|col| *(col.back().unwrap())).collect();
    let p1 = from_utf8(&p1).unwrap().to_string();

    let p2: Vec<u8> = stack_p2.iter().map(|col| *(col.back().unwrap())).collect();
    let p2 = from_utf8(&p2).unwrap().to_string();

    AnswerSet {
        p1: Answer::String(p1),
        p2: Answer::String(p2),
    }
}

fn parse_input<'a>(input: &'a str) -> (Vec<VecDeque<u8>>, impl Iterator<Item = (u8, u8, u8)> + 'a) {
    let mut stack = Vec::new();

    let mut lines = input.lines();

    // Parse stack
    'lines: while let Some(row) = lines.next() {
        for (i, byte) in row.bytes().enumerate() {
            if byte.is_ascii_uppercase() {
                let i_stack = i / 4;
                if stack.len() <= i_stack {
                    stack.resize(i_stack + 1, VecDeque::new());
                }
                stack[i_stack].push_front(byte);
            } else if byte.is_ascii_digit() {
                break 'lines;
            }
        }
    }

    // Skip empty line
    lines.next();

    // Parse instructions
    let instructions = lines.map(|line| {
        let mut ascii_iter = line.bytes().skip(5);
        let amount = parse_u8_from_ascii_iter(&mut ascii_iter);
        let src = ascii_iter.nth(5).unwrap() - b'0' - 1;
        let dest = ascii_iter.nth(4).unwrap() - b'0' - 1;

        (amount, src, dest)
    });

    (stack, instructions)
}

fn parse_u8_from_ascii_iter(ascii_iter: &mut impl Iterator<Item = u8>) -> u8 {
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
