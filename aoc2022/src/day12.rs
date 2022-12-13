use std::collections::VecDeque;

use crate::*;

pub const SOLUTION: Solution = Solution { day: 12, solve };

fn solve(input: &str) -> AnswerSet {
    let (mut map, start, lowests) = generate_map(input);

    //let p1 = bfs(&mut map, &start, &lowests, false);
    let (p1, p2) = bfs(&mut map, &start, &lowests);

    AnswerSet {
        p1: Answer::U16(p1),
        p2: Answer::U16(p2),
    }
}

const DIRS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn bfs(
    map: &mut Grid<(u8, Option<u16>, Option<u16>)>,
    start: &(usize, usize),
    lowests: &Vec<(usize, usize)>,
) -> (u16, u16) {
    let mut p1 = 0;
    let mut p2 = 0;

    let mut queue = VecDeque::new();
    map.get_mut(start.0, start.1).2 = Some(0);
    queue.push_back(start.clone());
    for lowest in lowests.iter() {
        map.get_mut(lowest.0, lowest.1).2 = Some(0);
        queue.push_back(lowest.clone());
    }

    // Part 2
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

                if next.0 == b'E' && (curr.0 == b'z' || curr.0 == b'y') {
                    p2 = curr.2.unwrap() + 1;
                    break;
                } else if next.0 as i16 - curr.0 as i16 <= 1 && next.2.is_none() {
                    next.2 = Some(curr.2.unwrap() + 1);
                    queue.push_back((next_r as usize, next_c as usize));
                }
            }
        }
    }

    map.get_mut(start.0, start.1).1 = Some(0);
    queue.clear();
    queue.push_back(start.clone());
    // Part 1
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

                if next.0 == b'E' && (curr.0 == b'z' || curr.0 == b'y') {
                    p1 = curr.1.unwrap() + 1;
                    break;
                } else if next.0 as i16 - curr.0 as i16 <= 1 && next.1.is_none() {
                    next.1 = Some(curr.1.unwrap() + 1);
                    queue.push_back((next_r as usize, next_c as usize));
                }
            }
        }
    }

    (p1, p2)
}

fn generate_map(
    input: &str,
) -> (
    Grid<(u8, Option<u16>, Option<u16>)>,
    (usize, usize),
    Vec<(usize, usize)>,
) {
    let mut start = (0, 0);
    let mut lowests = Vec::new();
    let mut row = 0;
    let mut flatmap = Vec::new();

    for line in input.lines() {
        for (col, byte) in line.bytes().enumerate() {
            if byte == b'S' {
                start = (row, col);
                flatmap.push((b'a', None, None));
            } else {
                if byte == b'a' {
                    lowests.push((row, col));
                }
                flatmap.push((byte, None, None));
            }
        }
        row += 1;
    }

    let cols = flatmap.len() / row;

    (Grid::new_with_data(flatmap, cols), start, lowests)
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
