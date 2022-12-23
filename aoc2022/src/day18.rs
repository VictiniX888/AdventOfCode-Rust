use std::collections::VecDeque;

use crate::*;

pub const SOLUTION: Solution = Solution { day: 18, solve };

// Minimum bounds
const N: usize = 22;

// ~ 280 us
fn solve(input: &str) -> AnswerSet {
    let (grid, p1) = parse_input(input);
    let p2 = part_2(grid);

    AnswerSet {
        p1: Answer::Usize(p1),
        p2: Answer::Usize(p2),
    }
}

const DIFFS: [isize; 2] = [1, -1];

fn part_2(grid: [[[bool; N]; N]; N]) -> usize {
    let mut area = 0;

    let mut outside = [[[false; N]; N]; N];
    let mut queue = VecDeque::new();
    for x in [0, N - 1] {
        for y in [0, N - 1] {
            for z in [0, N - 1] {
                if !grid[x][y][z] {
                    queue.push_back([x as isize, y as isize, z as isize]);
                    outside[x][y][z] = true;
                }
            }
        }
    }

    // Flood fill outside region
    while let Some(mut point) = queue.pop_front() {
        for ax in 0..point.len() {
            for d in DIFFS {
                if point[ax] + d >= 0 && point[ax] + d < N as isize {
                    point[ax] += d;
                    // println!(
                    //     "{:?} {}",
                    //     point, grid[point[0] as usize][point[1] as usize][point[2] as usize]
                    // );
                    if grid[point[0] as usize][point[1] as usize][point[2] as usize] {
                        area += 1;
                    } else {
                        if !outside[point[0] as usize][point[1] as usize][point[2] as usize] {
                            outside[point[0] as usize][point[1] as usize][point[2] as usize] = true;
                            queue.push_back(point);
                        }
                    }
                    point[ax] -= d;
                }
            }
        }
    }

    // Find faces at the faces that were not "hit" by the flood fill algorithm
    let mut point = [0; 3];
    for ax in 0..point.len() {
        for ax1 in [0, N - 1] {
            for ax2 in 0..N - 1 {
                for ax3 in 0..N - 1 {
                    point[ax] = ax3;
                    point[(ax + 1) % point.len()] = ax1;
                    point[(ax + 2) % point.len()] = ax2;
                    if grid[point[0]][point[1]][point[2]] {
                        area += 1;
                    }
                }
            }
        }
    }

    area
}

fn parse_input(input: &str) -> ([[[bool; N]; N]; N], usize) {
    let mut grid = [[[false; N]; N]; N];
    let mut area = 0;

    for line in input.lines() {
        let mut iter = line.bytes();
        let x = parse_u8_from_iter(&mut iter) as isize;
        let y = parse_u8_from_iter(&mut iter) as isize;
        let z = parse_u8_from_iter(&mut iter) as isize;
        let mut point = [x, y, z];

        for ax in 0..point.len() {
            for d in DIFFS {
                if point[ax] + d >= 0 && point[ax] + d < N as isize {
                    point[ax] += d;
                    if grid[point[0] as usize][point[1] as usize][point[2] as usize] {
                        area -= 1;
                    } else {
                        area += 1;
                    }
                    point[ax] -= d;
                } else {
                    area += 1;
                }
            }
        }

        grid[point[0] as usize][point[1] as usize][point[2] as usize] = true;
    }

    (grid, area)
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
