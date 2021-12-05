use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

use aoc;

fn main() {
    let input = aoc::read_input(5);
    let input: Vec<Line> = input.lines().map(|s| s.parse().unwrap()).collect();

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

// PART 1
fn part_1(lines: &Vec<Line>) -> usize {
    count_overlaps(lines, false)
}

// PART 2
fn part_2(lines: &Vec<Line>) -> usize {
    count_overlaps(lines, true)
}

// COMMON
fn count_overlaps(lines: &Vec<Line>, count_diagonals: bool) -> usize {
    let mut point_map: HashMap<Point, u32> = HashMap::new();

    for line in lines {
        if count_diagonals || line.is_straight() {
            for point in line.points() {
                let entry = point_map.entry(point).or_insert(0);
                *entry += 1;
            }
        }
    }

    point_map
        .values()
        .filter(|&&occurances| occurances > 1)
        .count()
}

#[derive(PartialEq, Eq, Hash)]
struct Point(u32, u32);

impl FromStr for Point {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const ERROR: &str = "Failed to parse Point from string";
        let mut iter = s.split(',');

        Ok(Point(
            iter.next().ok_or(ERROR)?.parse()?,
            iter.next().ok_or(ERROR)?.parse()?,
        ))
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point(self.0, self.1)
    }
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_straight(&self) -> bool {
        let Point(x1, y1) = self.start;
        let Point(x2, y2) = self.end;

        x1 == x2 || y1 == y2
    }

    fn points(&self) -> Vec<Point> {
        let mut points = vec![];

        let Point(x1, y1) = self.start;
        let Point(x2, y2) = self.end;

        let mut x_next = x1;
        let mut y_next = y1;

        points.push(Point(x_next, y_next));

        while x_next != x2 || y_next != y2 {
            if y_next < y2 {
                y_next += 1;
            } else if y_next > y2 {
                y_next -= 1;
            }

            if x_next < x2 {
                x_next += 1;
            } else if x_next > x2 {
                x_next -= 1;
            }

            points.push(Point(x_next, y_next));
        }

        points
    }
}

impl FromStr for Line {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const ERROR: &str = "Failed to parse Line from string";
        let mut iter = s.split(" -> ");

        Ok(Line {
            start: iter.next().ok_or(ERROR)?.parse()?,
            end: iter.next().ok_or(ERROR)?.parse()?,
        })
    }
}
