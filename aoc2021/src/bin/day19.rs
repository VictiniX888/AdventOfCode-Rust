use std::collections::HashSet;
use std::error::Error;
use std::hash::Hash;
use std::ops::{Add, Sub};
use std::str::FromStr;

use aoc;

fn main() {
    let input = aoc::read_input(19);
    let mut input: Vec<Scanner> = input.split("\n\n").map(aoc::parse_str).collect();
    input[0].pos = Point::new(0, 0, 0);

    let (part_1, part_2) = solve(input);
    println!("{}", part_1);
    println!("{}", part_2);
}

fn solve(mut scanners: Vec<Scanner>) -> (usize, i32) {
    let mut done = vec![];

    let mut stack = vec![scanners.remove(0)];

    while let Some(scanner_a) = stack.pop() {
        let mut b = 0;
        while b < scanners.len() {
            let scanner_b = &mut scanners[b];
            let (intersection, orientation, translation) = scanner_a.intersect(scanner_b);
            if intersection.len() >= 12 {
                scanner_b.pos = translation;
                scanner_b.beacons = orientation;
                stack.push(scanners.remove(b));
            } else {
                b += 1;
            }
        }
        done.push(scanner_a);
    }

    // PART 1
    let sum = done
        .iter()
        .map(|s| s.beacons.to_owned())
        .reduce(|acc, beacons| acc.union(&beacons).cloned().collect())
        .unwrap()
        .len();

    // PART 2
    let mut max_dist = 0;
    for (i, scanner_a) in done.iter().enumerate() {
        for scanner_b in done.iter().skip(i + 1) {
            let dist = (&scanner_a.pos - &scanner_b.pos).manhattan_dist();
            if dist > max_dist {
                max_dist = dist;
            }
        }
    }

    (sum, max_dist)
}

// COMMON
#[derive(Debug)]
struct Scanner {
    pos: Point,
    beacons: HashSet<Point>,
}

impl Scanner {
    fn orientations(&self) -> Vec<HashSet<Point>> {
        transpose(self.beacons.iter().map(|p| p.orientations()).collect())
            .into_iter()
            .map(|v| HashSet::from_iter(v))
            .collect()
    }

    fn intersect(&self, other: &Scanner) -> (HashSet<Point>, HashSet<Point>, Point) {
        let mut intersection = HashSet::new();
        let mut orientaion = HashSet::new();
        let mut translation = Point::new(0, 0, 0);
        for new_orientation in other.orientations() {
            for beacon in &self.beacons {
                for point in new_orientation.iter() {
                    let translate = beacon - point;
                    let new_orientation = new_orientation.iter().map(|p| p + &translate).collect();
                    let new_intersection: HashSet<Point> = self
                        .beacons
                        .intersection(&new_orientation)
                        .cloned()
                        .collect();
                    if new_intersection.len() > intersection.len() {
                        intersection = new_intersection;
                        orientaion = new_orientation;
                        translation = translate;
                    }
                }
            }
        }

        (intersection, orientaion, translation)
    }
}

impl FromStr for Scanner {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s.lines().skip(1).map(aoc::parse_str).collect();
        Ok(Scanner {
            pos: Point::new(0, 0, 0),
            beacons: points,
        })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }

    // https://stackoverflow.com/a/16467849
    fn roll(&self) -> Point {
        Point::new(self.x, self.z, -self.y)
    }

    fn turn(&self) -> Point {
        Point::new(-self.y, self.x, self.z)
    }

    fn orientations(&self) -> Vec<Point> {
        let mut orientations = Vec::with_capacity(24);

        let mut point = self.clone();

        for _ in 0..2 {
            for _ in 0..3 {
                point = point.roll();
                orientations.push(point.clone());
                for _ in 0..3 {
                    point = point.turn();
                    orientations.push(point.clone());
                }
            }
            point = point.roll().turn().roll();
        }

        orientations
    }

    fn manhattan_dist(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl FromStr for Point {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',');
        Ok(Point::new(
            iter.next().ok_or("Failed to parse point!")?.parse()?,
            iter.next().ok_or("Failed to parse point!")?.parse()?,
            iter.next().ok_or("Failed to parse point!")?.parse()?,
        ))
    }
}

// https://stackoverflow.com/a/64499219
fn transpose<T: Hash + Eq>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
