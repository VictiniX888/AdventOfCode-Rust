use std::collections::HashSet;

use crate::*;

pub const SOLUTION: Solution = Solution { day: 9, solve };

// 1 ms
fn solve(input: &str) -> AnswerSet {
    let mut knots = [(0, 0); 10];

    let mut visited1 = HashSet::new();
    visited1.insert(knots[1]);

    let mut visited2 = HashSet::new();
    visited2.insert(knots[9]);

    for instruction in input.lines() {
        let mut iter = instruction.bytes();

        let dir = iter.next().unwrap();
        let (dx, dy) = parse_dir(dir);

        iter.next();
        let steps = parse_u8_from_ascii_iter(&mut iter);

        for _ in 0..steps {
            knots[0].0 += dx;
            knots[0].1 += dy;

            for i in 1..10 {
                let diff_x = knots[i - 1].0 - knots[i].0;
                let diff_y = knots[i - 1].1 - knots[i].1;

                let knot = &mut knots[i];

                if diff_x.abs() > 1 || diff_y.abs() > 1 {
                    knot.0 += diff_x.signum();
                    knot.1 += diff_y.signum();
                } else {
                    // Rest of rope does not move
                    break;
                }
            }

            visited1.insert(knots[1]);
            visited2.insert(knots[9]);
        }
    }

    let p1 = visited1.len();
    let p2 = visited2.len();

    AnswerSet {
        p1: Answer::Usize(p1),
        p2: Answer::Usize(p2),
    }
}

fn parse_dir(char: u8) -> (i16, i16) {
    match char {
        b'R' => (1, 0),
        b'L' => (-1, 0),
        b'U' => (0, 1),
        b'D' => (0, -1),
        _ => panic!("Invalid direction"),
    }
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
