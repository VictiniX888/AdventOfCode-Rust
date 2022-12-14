use std::collections::HashSet;

use crate::*;

pub const SOLUTION: Solution = Solution { day: 14, solve };

// ~120 ms
fn solve(input: &str) -> AnswerSet {
    let start = (500, 0);
    let (mut rocks, y_max) = generate_map(input);

    let mut sand = 0;

    let mut p1 = 0;

    let p2 = loop {
        let rest = drop_sand(&start, &mut rocks, y_max + 2);
        if matches!(rest, Resting::Void) && p1 == 0 {
            p1 = sand;
        } else if matches!(rest, Resting::Ceiling) {
            break sand;
        }
        sand += 1;
    } + 1;

    AnswerSet {
        p1: Answer::U16(0),
        p2: Answer::Usize(p2),
    }
}

fn drop_sand(pos: &(i32, i32), rocks: &mut HashSet<(i32, i32)>, floor: i32) -> Resting {
    let mut x = pos.0;
    let mut y = pos.1;

    while y < floor - 1 {
        if !rocks.contains(&(x, y + 1)) {
            y += 1;
        } else if !rocks.contains(&(x - 1, y + 1)) {
            x -= 1;
            y += 1;
        } else if !rocks.contains(&(x + 1, y + 1)) {
            x += 1;
            y += 1;
        } else if y == pos.1 {
            return Resting::Ceiling;
        } else {
            rocks.insert((x, y));
            return Resting::Floor;
        }
    }

    rocks.insert((x, y));
    Resting::Void
}

#[derive(Debug)]
enum Resting {
    Void,
    Floor,
    Ceiling,
}

fn generate_map(input: &str) -> (HashSet<(i32, i32)>, i32) {
    let mut rocks = HashSet::new();
    let mut y_max = 0;

    for line in input.lines() {
        let mut x_prev = None;
        let mut y_prev = None;
        for pair in line.split(" -> ") {
            let (x, y) = pair.split_once(',').unwrap();
            let x = x.parse::<i32>().unwrap();
            let y = y.parse::<i32>().unwrap();

            if let Some(mut x_prev) = x_prev {
                let mut y_prev = y_prev.unwrap();

                let dx = (x - x_prev as i32).signum();
                let dy = (y - y_prev as i32).signum();

                while x_prev != x || y_prev != y {
                    rocks.insert((x_prev, y_prev));
                    x_prev += dx;
                    y_prev += dy;
                }
            }

            x_prev = Some(x);
            y_prev = Some(y);

            if y > y_max {
                y_max = y;
            }
        }

        rocks.insert((x_prev.unwrap(), y_prev.unwrap()));
    }

    (rocks, y_max)
}
