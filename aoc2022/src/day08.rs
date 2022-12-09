use crate::*;

pub const SOLUTION: Solution = Solution { day: 8, solve };

/* ======== O(N SQRT N) SOLUTION ======== */
// ~430 us
fn solve(input: &str) -> AnswerSet {
    let map = generate_map(input);

    let mut p1 = 0;
    let mut p2 = 0;

    for r in 1..map.rows - 1 {
        for c in 1..map.cols - 1 {
            let mut scenic_score = 1;
            let mut visible = false;

            for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let height = *map.get(r, c);
                let mut seen = 0;
                let mut r = r as isize + dr;
                let mut c = c as isize + dc;

                while r >= 0 && r < map.rows as isize && c >= 0 && c < map.cols as isize {
                    seen += 1;

                    if height <= *map.get(r as usize, c as usize) {
                        break;
                    }

                    r = r + dr;
                    c = c + dc;
                }
                scenic_score *= seen;

                if (r < 0 || r >= map.rows as isize || c < 0 || c >= map.cols as isize) && !visible
                {
                    visible = true;
                    p1 += 1;
                }
            }

            if scenic_score > p2 {
                p2 = scenic_score;
            }
        }
    }

    p1 += (2 * map.rows + 2 * (map.cols - 2)) as u16;

    AnswerSet {
        p1: Answer::U16(p1),
        p2: Answer::U32(p2),
    }
}

fn calculate_scenic_score(r: usize, c: usize, map: &Grid<u8>) -> u32 {
    let mut scenic_score = 1;

    for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
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
        scenic_score *= visible as u32;
    }

    scenic_score
}

/* ======== O(N) SOLUTION ======== */
// ~575 us
fn solve_linear(input: &str) -> AnswerSet {
    let map = generate_map(input);

    let dirs = [
        IterDirection {
            outer_range: Range::new(0, map.rows as isize, 1),
            inner_range: Range::new(0, map.cols as isize, 1),
            row_major: true,
        },
        IterDirection {
            outer_range: Range::new(0, map.rows as isize, 1),
            inner_range: Range::new(map.cols as isize - 1, -1, -1),
            row_major: true,
        },
        IterDirection {
            outer_range: Range::new(0, map.cols as isize, 1),
            inner_range: Range::new(0, map.rows as isize, 1),
            row_major: false,
        },
        IterDirection {
            outer_range: Range::new(0, map.cols as isize, 1),
            inner_range: Range::new(map.rows as isize - 1, -1, -1),
            row_major: false,
        },
    ];

    let mut scores = Grid::new(map.rows, map.cols, (false, 1));

    let mut visible = 0;
    let mut max_scenic_score = 0;

    for dir in dirs {
        for x in dir.outer_range {
            let mut cache = [-1; 10];
            let mut tallest = -1;
            for y in dir.inner_range.clone() {
                let r = if dir.row_major { x } else { y } as usize;
                let c = if dir.row_major { y } else { x } as usize;

                // Get viewing distance
                let &height = map.get(r, c);
                let distance = (cache[height as usize] + 1) as u32;
                let score = scores.get_mut(r, c);
                (*score).1 *= distance;

                if (*score).1 > max_scenic_score {
                    max_scenic_score = (*score).1;
                }

                // Check if visible from edge
                if height as i8 > tallest {
                    if !(*score).0 {
                        (*score).0 = true;
                        visible += 1;
                    }
                    tallest = height as i8;
                }

                // Update cache
                for i in 0..=height as usize {
                    cache[i] = 0;
                }

                for i in height as usize + 1..cache.len() {
                    cache[i] += 1;
                }
            }
        }
    }

    AnswerSet {
        p1: Answer::U16(visible),
        p2: Answer::U32(max_scenic_score),
    }
}
struct IterDirection {
    outer_range: Range,
    inner_range: Range,
    row_major: bool,
}

#[derive(Clone)]
struct Range {
    start: isize, // inclusive
    end: isize,   // exclusive
    step: isize,
    curr: isize,
}

impl Range {
    fn new(start: isize, end: isize, step: isize) -> Self {
        Range {
            start,
            end,
            step,
            curr: start,
        }
    }
}

impl Iterator for Range {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr != self.end {
            let next = Some(self.curr);
            self.curr += self.step;
            next
        } else {
            None
        }
    }
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
