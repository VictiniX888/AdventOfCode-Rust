use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;

use aoc;

fn main() {
    let input = aoc::read_input(13);
    let (points, folds) = input.split_once("\n\n").unwrap();
    let points: Vec<Point> = points.lines().map(aoc::parse_str).collect();
    let folds: Vec<FoldInstruction> = folds.lines().map(aoc::parse_str).collect();

    println!("{}", part_1(&points, &folds));
    println!("{}", part_2(&points, &folds));
}

// PART 1
fn part_1(points: &[Point], folds: &[FoldInstruction]) -> usize {
    let mut dgrid = DGrid::new(points);
    dgrid.fold(&folds[0]);

    dgrid.0.iter().map(|(_, ys)| ys.len()).sum()
}

// PART 2
fn part_2(points: &[Point], folds: &[FoldInstruction]) -> String {
    let mut dgrid = DGrid::new(points);
    for instruction in folds {
        dgrid.fold(instruction);
    }

    // Generate ASCII art
    let &x_max = dgrid.0.keys().max().unwrap();
    let &y_max = dgrid.1.keys().max().unwrap();
    let mut grid: Vec<Vec<bool>> = vec![vec![false; x_max + 1]; y_max + 1];
    for (&y, xs) in dgrid.1.iter() {
        for &x in xs {
            grid[y][x] = true;
        }
    }
    let mut str = String::new();
    for row in grid.iter() {
        for b in row {
            str.push(match b {
                true => '\u{2588}',
                false => ' ',
            })
        }
        str.push('\n');
    }
    str
}

// COMMON
struct Point(usize, usize);

impl FromStr for Point {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or("Failed to parse Point")?;
        Ok(Point(x.parse()?, y.parse()?))
    }
}

struct FoldInstruction(char, usize);

impl FromStr for FoldInstruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (axis, value) = s
            .split_whitespace()
            .nth(2)
            .ok_or("Failed to parse FoldInstruction")?
            .split_once('=')
            .ok_or("Failed to parse FoldInstruction")?;

        Ok(FoldInstruction(
            axis.chars().nth(0).ok_or("Failed to parse axis")?,
            value.parse()?,
        ))
    }
}

type Grid = HashMap<usize, HashSet<usize>>;

struct DGrid(Grid, Grid);

impl DGrid {
    fn new(points: &[Point]) -> DGrid {
        let mut grid_x: Grid = HashMap::new();
        let mut grid_y: Grid = HashMap::new();

        for &Point(x, y) in points.iter() {
            grid_x.entry(x).or_default().insert(y);
            grid_y.entry(y).or_default().insert(x);
        }

        DGrid(grid_x, grid_y)
    }

    fn fold(&mut self, &FoldInstruction(axis, value): &FoldInstruction) {
        let DGrid(grid_x, grid_y) = self;

        match axis {
            'x' => {
                for x in grid_x.keys().cloned().collect::<Vec<usize>>() {
                    if x > value {
                        if let Some(ys) = grid_x.remove(&x) {
                            for y in ys {
                                let x_new = value - (x - value);

                                let y_set = grid_y.get_mut(&y).unwrap();
                                y_set.remove(&x);
                                y_set.insert(x_new);
                                grid_x.entry(x_new).or_default().insert(y);
                            }
                        }
                    }
                }
            }

            'y' => {
                for y in grid_y.keys().cloned().collect::<Vec<usize>>() {
                    if y > value {
                        if let Some(xs) = grid_y.remove(&y) {
                            for x in xs {
                                let y_new = value - (y - value);

                                let x_set = grid_x.get_mut(&x).unwrap();
                                x_set.remove(&y);
                                x_set.insert(y_new);
                                grid_y.entry(y_new).or_default().insert(x);
                            }
                        }
                    }
                }
            }

            _ => panic!("Unknown axis {} found!", axis),
        }
    }
}
