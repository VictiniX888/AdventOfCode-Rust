use std::collections::HashSet;

use aoc;

fn main() {
    let input = aoc::read_input(9);
    let input: Vec<Vec<u32>> = input
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

// PART 1
fn part_1(heightmap: &[Vec<u32>]) -> u32 {
    find_low_points(heightmap)
        .iter()
        .map(|&Point(x, y)| heightmap[y][x] + 1)
        .sum()
}

// PART_2
fn part_2(heightmap: &[Vec<u32>]) -> usize {
    let mut basin_sizes = find_low_points(heightmap)
        .into_iter()
        .map(|p| get_basin_size(p, heightmap))
        .collect::<Vec<usize>>();

    basin_sizes.sort();

    basin_sizes.iter().rev().take(3).product()
}

fn get_basin_size(low_point: Point, heightmap: &[Vec<u32>]) -> usize {
    let map_height = heightmap.len();
    let map_width = heightmap[0].len();

    let mut points: HashSet<Point> = HashSet::new();

    let mut new_points: HashSet<Point> = HashSet::new();
    new_points.insert(low_point);
    while !new_points.is_empty() {
        let mut new_new_points: HashSet<Point> = HashSet::new();
        for point in new_points.iter() {
            for neighbor in get_neighbors(point, map_height, map_width) {
                let Point(x, y) = neighbor;
                if heightmap[y][x] != 9
                    && !points.contains(&neighbor)
                    && !new_points.contains(&neighbor)
                {
                    new_new_points.insert(neighbor);
                }
            }
        }

        points.extend(new_points);
        new_points = new_new_points;
    }

    points.len()
}

// COMMON
#[derive(PartialEq, Eq, Hash)]
struct Point(usize, usize);

fn find_low_points(heightmap: &[Vec<u32>]) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];

    let map_height = heightmap.len();
    let map_width = heightmap[0].len();

    for y in 0..map_height {
        for x in 0..map_width {
            let point = Point(x, y);
            let height = heightmap[y][x];

            if get_neighbors(&point, map_height, map_width)
                .iter()
                .all(|&Point(x, y)| height < heightmap[y][x])
            {
                points.push(point);
            }
        }
    }

    points
}

fn get_neighbors(point: &Point, map_height: usize, map_width: usize) -> Vec<Point> {
    let Point(x, y) = point;

    let mut neighbors: Vec<Point> = vec![];

    if *y > 0 {
        neighbors.push(Point(*x, *y - 1));
    }

    if *y < map_height - 1 {
        neighbors.push(Point(*x, *y + 1));
    }

    if *x > 0 {
        neighbors.push(Point(*x - 1, *y));
    }

    if *x < map_width - 1 {
        neighbors.push(Point(*x + 1, *y));
    }

    neighbors
}
