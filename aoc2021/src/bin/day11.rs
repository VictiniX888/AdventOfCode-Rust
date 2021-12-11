use aoc;

fn main() {
    let input = aoc::read_input(11);
    let input: Vec<Vec<u32>> = input
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    println!("{}", part_1(input.to_vec()));
    println!("{}", part_2(input));
}

// PART 1
fn part_1(mut map: Vec<Vec<u32>>) -> usize {
    const STEPS: u32 = 100;

    let mut flashes = 0;

    let rows = map.len();
    let cols = map[0].len();

    for _ in 0..STEPS {
        flashes += step(&mut map, rows, cols);
    }

    flashes
}

// PART 2
fn part_2(mut map: Vec<Vec<u32>>) -> usize {
    let mut steps = 0;

    let rows = map.len();
    let cols = map[0].len();

    loop {
        steps += 1;

        if step(&mut map, rows, cols) >= rows * cols {
            break steps;
        }
    }
}

// COMMON
fn step(map: &mut Vec<Vec<u32>>, rows: usize, cols: usize) -> usize {
    let mut flashes = 0;

    let mut flashing: Vec<Point> = vec![];

    for r in 0..rows {
        for c in 0..cols {
            if map[r][c] >= 9 {
                flashes += 1;
                flashing.push(Point(r, c));
                map[r][c] = 0;
            } else {
                map[r][c] += 1;
            }
        }
    }

    while let Some(point) = flashing.pop() {
        for Point(r, c) in point.neighbors(rows, cols) {
            if map[r][c] >= 9 {
                flashes += 1;
                flashing.push(Point(r, c));
                map[r][c] = 0;
            } else if map[r][c] > 0 {
                map[r][c] += 1;
            }
        }
    }

    flashes
}

struct Point(/* r: */ usize, /* c: */ usize);

impl Point {
    fn neighbors(&self, rows: usize, cols: usize) -> Vec<Point> {
        let &Point(r, c) = self;
        [
            (r > 0 && c > 0, (-1, -1)),
            (r > 0, (-1, 0)),
            (r > 0 && c < cols - 1, (-1, 1)),
            (c > 0, (0, -1)),
            (c < cols - 1, (0, 1)),
            (r < rows - 1 && c > 0, (1, -1)),
            (r < rows - 1, (1, 0)),
            (r < rows - 1 && c < cols - 1, (1, 1)),
        ]
        .into_iter()
        .filter(|(p, _)| *p)
        .map(|(_, (dr, dc))| Point((r as i32 + dr) as usize, (c as i32 + dc) as usize))
        .collect()
    }
}
