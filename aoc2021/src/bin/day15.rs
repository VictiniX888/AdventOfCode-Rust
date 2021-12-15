use std::collections::HashMap;
use std::collections::HashSet;

use aoc;

fn main() {
    let input = aoc::read_input(15);
    let input: Vec<Vec<usize>> = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

// PART 1
fn part_1(grid: &Vec<Vec<usize>>) -> usize {
    dijkstra(grid, Point(0, 0), Point(grid.len() - 1, grid[0].len() - 1))
}

// PART 2
fn part_2(grid: &Vec<Vec<usize>>) -> usize {
    dijkstra(
        grid,
        Point(0, 0),
        Point(grid.len() * 5 - 1, grid[0].len() * 5 - 1),
    )
}

// COMMON
fn dijkstra(grid: &Vec<Vec<usize>>, start: Point, end: Point) -> usize {
    let mut unvisited: HashMap<Point, usize> = HashMap::from([(start, 0)]);
    let mut visited: HashSet<Point> = HashSet::new();

    loop {
        let (current, &steps) = unvisited.iter().min_by_key(|&(_, risk)| risk).unwrap();
        let current = current.clone();

        if current == end {
            break steps;
        }

        for neighbor in current.neighbors(end.0 + 1, end.1 + 1) {
            if visited.contains(&neighbor) {
                continue;
            }

            let steps = steps + get_risk(&neighbor, grid);

            match unvisited.get_mut(&neighbor) {
                Some(cached_steps) if steps < *cached_steps => *cached_steps = steps,
                Some(_) => continue,
                None => {
                    unvisited.insert(neighbor.clone(), steps);
                }
            }
        }

        unvisited.remove(&current);
        visited.insert(current);
    }
}

fn get_risk(point: &Point, grid: &Vec<Vec<usize>>) -> usize {
    let &Point(r, c) = point;

    let risk = grid[r % grid.len()][c % grid[0].len()];
    let dr = r / grid.len() + c / grid[0].len();

    let risk = risk + dr;
    if risk > 9 {
        risk - 9
    } else {
        risk
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Point(/* r: */ usize, /* c: */ usize);

impl Point {
    fn neighbors(&self, rows: usize, cols: usize) -> Vec<Point> {
        let &Point(r, c) = self;
        [
            (r > 0, (-1, 0)),
            (c > 0, (0, -1)),
            (c < cols - 1, (0, 1)),
            (r < rows - 1, (1, 0)),
        ]
        .into_iter()
        .filter(|(p, _)| *p)
        .map(|(_, (dr, dc))| Point((r as i32 + dr) as usize, (c as i32 + dc) as usize))
        .collect()
    }
}
