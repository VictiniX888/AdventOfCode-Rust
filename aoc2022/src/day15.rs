use std::{collections::BTreeSet, str::FromStr};

use crate::*;

pub const SOLUTION: Solution = Solution {
    day: 15,
    solve: solve_optimized,
};

const HEIGHT: isize = 4_000_000;

// ~8 us
fn solve_optimized(input: &str) -> AnswerSet {
    let sensors = parse_sensors(input);

    // Part 1; same as before
    let mut ranges: Vec<Range> = Vec::with_capacity(sensors.len());
    for sensor in sensors.iter() {
        let orth = (sensor.y - HEIGHT / 2).abs();
        let rem = sensor.radius - orth;
        let mut start = sensor.x - rem;
        let end = sensor.x + rem;

        if start <= end {
            // println!("{:?}", ranges);
            // println!("{} {}", start, end);
            let mut i = 0;
            while i < ranges.len() {
                let range = &mut ranges[i];

                if end >= range.start - 1 && end <= range.end {
                    if start < range.start {
                        range.start = start;
                    }
                    break;
                } else if end < range.start - 1 {
                    ranges.insert(i, Range { start, end });
                    break;
                } else {
                    if start >= range.start && start <= range.end {
                        start = range.end + 1;
                    } else if start < range.start {
                        let range_start = range.start;
                        let range_end = range.end;
                        ranges.insert(
                            i,
                            Range {
                                start,
                                end: range_start - 1,
                            },
                        );
                        start = range_end + 1;
                    }
                }

                i += 1;
            }

            if i >= ranges.len() {
                ranges.push(Range { start, end });
            }
        }
    }

    let p1 = ranges
        .iter()
        .map(|range| (range.end - range.start + 1) as usize)
        .sum::<usize>()
        - sensors
            .iter()
            .fold(Vec::new(), |mut acc, sensor| {
                if sensor.y_beacon == HEIGHT / 2 && !acc.contains(&sensor.x_beacon) {
                    acc.push(sensor.x_beacon)
                };
                acc
            })
            .len();

    // Part 2
    let mut p2 = 0;

    // We store edges only by their y-intercepts (and assume they span the entire plane)
    let mut top_left_edges = BTreeSet::<isize>::new();
    let mut top_right_edges = BTreeSet::<isize>::new();

    for sensor in sensors.iter() {
        // Each sensor has 2 edges going from top-left to bottom-right
        top_left_edges.insert(sensor.y - sensor.radius - sensor.x);
        top_left_edges.insert(sensor.y + sensor.radius - sensor.x);

        // And 2 edges going from top-right to bottom-left
        top_right_edges.insert(sensor.y - sensor.radius + sensor.x);
        top_right_edges.insert(sensor.y + sensor.radius + sensor.x);
    }

    // We try to find edges that are exactly 2 blocks away from each other (1 line gap)
    let mut top_left_gaps = Vec::<isize>::new();
    let mut top_right_gaps = Vec::<isize>::new();

    for edge in top_left_edges.iter() {
        if top_left_edges.get(&(edge + 2)).is_some() {
            top_left_gaps.push(edge + 1);
        }
    }

    for edge in top_right_edges.iter() {
        if top_right_edges.get(&(edge + 2)).is_some() {
            top_right_gaps.push(edge + 1);
        }
    }

    if top_left_gaps.len() == 1 && top_right_gaps.len() == 1 {
        let x = (top_right_gaps[0] - top_left_gaps[0]) / 2;
        let y = (top_left_gaps[0] + top_right_gaps[0]) / 2;

        p2 = x * 4_000_000 + y;
    } else {
        // We find intersections between all pairs of lines
        'outer: for top_left in top_left_gaps.iter() {
            for top_right in top_right_gaps.iter() {
                let x = (top_right - top_left) / 2;
                let y = (top_left + top_right) / 2;

                if x >= 0 && x <= HEIGHT && y >= 0 && y <= HEIGHT {
                    // Check if beacon is actually hidden
                    if sensors
                        .iter()
                        .all(|sensor| (x - sensor.x).abs() + (y - sensor.y).abs() > sensor.radius)
                    {
                        p2 = x * 4_000_000 + y;

                        break 'outer;
                    }
                }
            }
        }
    }

    AnswerSet {
        p1: Answer::Usize(p1),
        p2: Answer::Usize(p2 as usize),
    }
}

//~ 520 ms
fn solve(input: &str) -> AnswerSet {
    let sensors = parse_sensors(input);

    // let y = 10;
    // let y = 2_000_000;

    let mut p1 = 0;
    let mut p2 = 0;

    'main: for y in 0..=HEIGHT {
        let mut ranges: Vec<Range> = Vec::with_capacity(sensors.len());
        for sensor in sensors.iter() {
            let orth = (sensor.y - y).abs();
            let rem = sensor.radius - orth;
            let mut start = sensor.x - rem;
            let end = sensor.x + rem;

            if start <= end {
                // println!("{:?}", ranges);
                // println!("{} {}", start, end);
                let mut i = 0;
                while i < ranges.len() {
                    let range = &mut ranges[i];

                    if end >= range.start - 1 && end <= range.end {
                        if start < range.start {
                            range.start = start;
                        }
                        break;
                    } else if end < range.start - 1 {
                        ranges.insert(i, Range { start, end });
                        break;
                    } else {
                        if start >= range.start && start <= range.end {
                            start = range.end + 1;
                        } else if start < range.start {
                            let range_start = range.start;
                            let range_end = range.end;
                            ranges.insert(
                                i,
                                Range {
                                    start,
                                    end: range_start - 1,
                                },
                            );
                            start = range_end + 1;
                        }
                    }

                    i += 1;
                }

                if i >= ranges.len() {
                    ranges.push(Range { start, end });
                }
            }
        }

        // println!("{:?}", ranges);

        if y == HEIGHT / 2 {
            p1 = ranges
                .iter()
                .map(|range| (range.end - range.start + 1) as usize)
                .sum();
            p1 -= sensors
                .iter()
                .fold(Vec::new(), |mut acc, sensor| {
                    if sensor.y_beacon == y && !acc.contains(&sensor.x_beacon) {
                        acc.push(sensor.x_beacon)
                    };
                    acc
                })
                .len()
        }
        let mut last = -1;
        for range in ranges {
            if range.start > last + 1 {
                p2 = ((last + 1) * 4_000_000 + y) as usize;
                break 'main;
            }

            if range.end >= HEIGHT {
                last = range.end;
                break;
            } else {
                last = range.end;
            }
        }
        if last < HEIGHT {
            p2 = ((last + 1) * 4_000_000 + y) as usize;
            break 'main;
        }
    }

    AnswerSet {
        p1: Answer::Usize(p1),
        p2: Answer::Usize(p2),
    }
}

#[derive(Debug)]
struct Range {
    start: isize,
    end: isize,
}

fn parse_sensors(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Sensor>, _>>()
        .unwrap()
}

struct Sensor {
    x: isize,
    y: isize,
    x_beacon: isize,
    y_beacon: isize,
    radius: isize,
}

impl FromStr for Sensor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.bytes();

        iter.nth(11);
        let x = parse_isize_from_ascii_iter(&mut iter);

        iter.nth(2);
        let y = parse_isize_from_ascii_iter(&mut iter);

        iter.nth(23);
        let x_beacon = parse_isize_from_ascii_iter(&mut iter);

        iter.nth(2);
        let y_beacon = parse_isize_from_ascii_iter(&mut iter);

        let radius = (x - x_beacon).abs() + (y - y_beacon).abs();

        Ok(Self {
            x,
            y,
            x_beacon,
            y_beacon,
            radius,
        })
    }
}

fn parse_isize_from_ascii_iter(ascii_iter: &mut impl Iterator<Item = u8>) -> isize {
    let mut num = 0;
    let mut sign = true;

    if let Some(byte) = ascii_iter.next() {
        if byte == b'-' {
            sign = false;
        } else {
            num = (byte - b'0') as isize;
        }
    }

    while let Some(byte) = ascii_iter.next() {
        if !byte.is_ascii_digit() {
            break;
        }

        num *= 10;
        num += (byte - b'0') as isize;
    }

    if sign {
        num
    } else {
        num * -1
    }
}
