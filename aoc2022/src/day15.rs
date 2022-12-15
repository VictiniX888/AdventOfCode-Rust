use std::str::FromStr;

use crate::*;

pub const SOLUTION: Solution = Solution { day: 15, solve };

const HEIGHT: isize = 4_000_000;

//~ 540 ms
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
            let mut end = sensor.x + rem;

            if start <= end {
                if ranges.is_empty() {
                    ranges.push(Range { start, end });
                } else {
                    let mut i = 0;
                    while i < ranges.len() {
                        let (front, back) = ranges.split_at_mut(i + 1);
                        let mut range = &mut front[i];

                        if end >= range.start - 1 {
                            if end <= range.end {
                                if start < range.start {
                                    range.start = start;
                                }
                                end = start - 1;
                                break;
                            } else {
                                if start < range.start {
                                    range.start = start;
                                }
                                if start <= range.end + 1 {
                                    let next_range = back.get_mut(0);
                                    if let Some(next_range) = next_range {
                                        if next_range.start <= end - 1 {
                                            next_range.start = range.start;
                                            start = next_range.end + 1;
                                            ranges.remove(i);
                                            // end = start - 1;
                                            // break;
                                        } else {
                                            range.end = end;
                                            end = start - 1;
                                            break;
                                        }
                                    } else {
                                        range.end = end;
                                        end = start - 1;
                                        break;
                                    }
                                }
                            }
                        } else {
                            if start <= end {
                                ranges.insert(i, Range { start, end });
                            }
                            end = start - 1;
                            break;
                        }

                        i += 1;
                    }

                    if start <= end {
                        ranges.push(Range { start, end });
                    }
                }
            }
        }

        let mut last = -1;
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
        for range in ranges {
            if range.start > last + 1 {
                p2 = ((last + 1) * 4_000_000 + y) as usize;
                break 'main;
            }

            if range.end >= HEIGHT {
                break;
            } else {
                last = range.end;
            }
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
