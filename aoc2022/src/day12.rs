use std::collections::VecDeque;

use crate::*;

pub const SOLUTION: Solution = Solution { day: 12, solve };

// ~50 us
fn solve(input: &str) -> AnswerSet {
    let (mut map, start) = generate_map(input);

    let (p1, p2) = bfs(&mut map, &start);

    AnswerSet {
        p1: Answer::U16(p1),
        p2: Answer::U16(p2),
    }
}

const DIRS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn bfs(map: &mut Grid<(u8, Option<u16>)>, start: &(usize, usize)) -> (u16, u16) {
    let mut p1 = 0;
    let mut p2 = 0;

    let mut queue = VecDeque::new();
    map.get_mut(start.0, start.1).1 = Some(0);
    queue.push_back(start.clone());

    while let Some(curr) = queue.pop_front() {
        for dir in DIRS.iter() {
            let next_r = curr.0 as isize + dir.0;
            let next_c = curr.1 as isize + dir.1;
            if next_r >= 0
                && next_r < map.rows as isize
                && next_c >= 0
                && next_c < map.cols as isize
            {
                let &curr = map.get(curr.0, curr.1);
                let next = map.get_mut(next_r as usize, next_c as usize);

                if next.0 == b'S' && (curr.0 == b'a' || curr.0 == b'b') {
                    p1 = curr.1.unwrap() + 1;
                    return (p1, p2);
                } else if curr.0 as i8 - next.0 as i8 <= 1 && next.1.is_none() {
                    if next.0 == b'a' && p2 == 0 {
                        p2 = curr.1.unwrap() + 1;
                    }

                    next.1 = Some(curr.1.unwrap() + 1);
                    queue.push_back((next_r as usize, next_c as usize));
                }
            }
        }
    }

    (p1, p2)
}

fn generate_map(input: &str) -> (Grid<(u8, Option<u16>)>, (usize, usize)) {
    let mut start = (0, 0);
    let mut row = 0;
    let mut flatmap = Vec::new();

    for line in input.lines() {
        for (col, byte) in line.bytes().enumerate() {
            if byte == b'E' {
                start = (row, col);
                flatmap.push((b'z', None));
            } else {
                flatmap.push((byte, None));
            }
        }
        row += 1;
    }

    let cols = flatmap.len() / row;

    (Grid::new_with_data(flatmap, cols), start)
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
