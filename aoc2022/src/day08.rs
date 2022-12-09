use crate::*;

pub const SOLUTION: Solution = Solution { day: 8, solve };

// ~405 us
fn solve(input: &str) -> AnswerSet {
    let map = generate_map(input);
    let p1 = find_visible_trees(&map);
    let p2 = find_max_scenic_score(&map);

    AnswerSet {
        p1: Answer::U16(p1),
        p2: Answer::U32(p2),
    }
}

fn find_max_scenic_score(map: &Grid<u8>) -> u32 {
    let mut max_score = 0;

    for r in 0..map.rows {
        for c in 0..map.cols {
            let score = calculate_scenic_score(r, c, map);
            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score
}

fn calculate_scenic_score(r: usize, c: usize, map: &Grid<u8>) -> u32 {
    let mut scenic_score = 1;

    for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
        let mut visible = 0;
        let mut r = r;
        let mut c = c;
        let height = *map.get(r, c);

        while r > 0 && r < map.rows - 1 && c > 0 && c < map.cols - 1 {
            r = (r as isize + dr) as usize;
            c = (c as isize + dc) as usize;

            visible += 1;

            if height <= *map.get(r, c) {
                break;
            }
        }

        scenic_score *= visible;
    }

    scenic_score
}

fn find_visible_trees(map: &Grid<u8>) -> u16 {
    let mut count = 0;

    let mut seen = Grid::new(map.rows, map.cols, false);

    let mut tallest;

    // Outer wall is always visible so we skip it
    for r in 1..map.rows - 1 {
        tallest = *map.get(r, 0);

        for c in 1..map.cols - 1 {
            let height = *map.get(r, c);
            if height > tallest {
                tallest = height;
                count += 1;
                *seen.get_mut(r, c) = true;

                if tallest == 9 {
                    break;
                }
            }
        }

        tallest = *map.get(r, map.cols - 1);

        for c in (1..map.cols).rev() {
            let height = *map.get(r, c);
            if height > tallest {
                tallest = height;
                let seen = seen.get_mut(r, c);
                if *seen {
                    break;
                } else {
                    count += 1;
                    *seen = true;
                }

                if tallest == 9 {
                    break;
                }
            }
        }
    }

    for c in 1..map.cols - 1 {
        tallest = *map.get(0, c);

        for r in 1..map.rows - 1 {
            let height = *map.get(r, c);
            if height > tallest {
                tallest = height;
                let seen = seen.get_mut(r, c);
                if !*seen {
                    count += 1;
                    *seen = true;
                }

                if tallest == 9 {
                    break;
                }
            }
        }

        tallest = *map.get(map.rows - 1, c);

        for r in (1..map.rows - 1).rev() {
            let height = *map.get(r, c);
            if height > tallest {
                tallest = height;
                let seen = seen.get_mut(r, c);
                if !*seen {
                    count += 1;
                    *seen = true;
                }

                if tallest == 9 {
                    break;
                }
            }
        }
    }

    (count + 2 * map.rows + 2 * (map.cols - 2)) as u16
}

fn generate_map(input: &str) -> Grid<u8> {
    let mut rows = 0;
    let flatmap: Vec<u8> = input
        .lines()
        .flat_map(|line| {
            rows += 1;
            line.bytes().map(|byte| byte - b'0')
        })
        .collect();

    let cols = flatmap.len() / rows;

    Grid::new_with_data(flatmap, cols)
}

struct Grid<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Grid<T> {
    fn new(rows: usize, cols: usize, init: T) -> Self
    where
        T: Clone,
    {
        Grid {
            data: vec![init; rows * cols],
            rows,
            cols,
        }
    }

    fn new_with_data(data: Vec<T>, cols: usize) -> Self {
        let rows = data.len() / cols;
        Grid { data, cols, rows }
    }

    fn push(&mut self, value: T) {
        self.data.push(value);
    }

    fn get(&self, row: usize, col: usize) -> &T {
        &self.data[row * self.cols + col]
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.data[row * self.cols + col]
    }
}
