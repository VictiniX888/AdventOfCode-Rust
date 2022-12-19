use std::collections::HashMap;

use crate::*;

pub const SOLUTION: Solution = Solution { day: 17, solve };

// ~ 2.3 ms
fn solve(input: &str) -> AnswerSet {
    let dir_iter = input.bytes();
    let iter_len = dir_iter.len();
    let mut dir_iter = dir_iter.cycle();

    let mut grid = Grid::new();
    let mut cache: HashMap<(Vec<u8>, usize, usize), (usize, usize)> = HashMap::new();
    let mut iter_i = 0;
    let mut rock_i = 0;

    let (cycle, offset, height_cycle, height_offset) = loop {
        grid.falling_skip(&mut dir_iter);
        iter_i += 3;
        while !grid.step(dir_iter.next().unwrap()) {
            iter_i += 1;
        }
        iter_i += 1;
        rock_i += 1;

        iter_i %= iter_len;

        if let Some((prev_i, prev_height)) = cache.insert(
            (grid.clone_state(), iter_i, rock_i % 5),
            (rock_i, grid.height()),
        ) {
            break (
                rock_i - prev_i,
                prev_i,
                grid.height() - prev_height,
                prev_height,
            );
        }
    };

    // Clone grid
    let mut grid_clone = Grid {
        data: grid.clone_state(),
        falling: &ROCKS[rock_i % 5],
        falling_i: grid.falling_i,
        pos: grid.pos,
        offset: grid.offset,
    };

    let mut dir_iter_clone = dir_iter.clone();

    let rem = (2022 - offset) % cycle;
    for _ in 0..rem {
        grid_clone.falling_skip(&mut dir_iter_clone);
        while !grid_clone.step(dir_iter_clone.next().unwrap()) {}
    }

    let p1 = ((2022 - offset) / cycle) * height_cycle
        + height_offset
        + (grid_clone.height() - height_offset - height_cycle);

    let rem = (1000000000000 - offset) % cycle;
    for _ in 0..rem {
        grid.falling_skip(&mut dir_iter);
        while !grid.step(dir_iter.next().unwrap()) {}
    }

    let p2 = ((1000000000000 - offset) / cycle) * height_cycle
        + height_offset
        + (grid.height() - height_offset - height_cycle);

    AnswerSet {
        p1: Answer::Usize(p1),
        p2: Answer::Usize(p2),
    }
}

const ROCKS: [Rock; 5] = [
    Rock::Horizontal,
    Rock::Plus,
    Rock::L,
    Rock::Vertical,
    Rock::Square,
];

enum Rock {
    Horizontal, // center: left-most block
    Plus,       // center: center block
    L,          // center: corner block
    Vertical,   // center: bottom-most block
    Square,     // center: bottom-left block
}

impl Rock {
    fn initial_pos(&self, height: usize) -> (usize, usize) {
        match self {
            Rock::Horizontal => (2, height + 4),
            Rock::Plus => (3, height + 5),
            Rock::L => (4, height + 4),
            Rock::Vertical => (2, height + 4),
            Rock::Square => (2, height + 4),
        }
    }
}

struct Grid {
    // We represent each row as a bitset
    // Bit 0 is left column, bit 6 is right column (bit 7 left unused)
    data: Vec<u8>,
    falling: &'static Rock,
    falling_i: usize,
    pos: (usize, usize),
    offset: usize,
}

impl Grid {
    // x is 0-indexed
    // y is 1-indexed (0 is floor)
    fn new() -> Self {
        Grid {
            data: vec![0xFE],
            falling: &ROCKS[0],
            falling_i: 0,
            pos: (&ROCKS[0]).initial_pos(0),
            offset: 0,
        }
    }

    fn falling_new(&mut self) {
        self.falling_i = (self.falling_i + 1) % 5;
        self.falling = &ROCKS[self.falling_i];
        self.pos = self.falling.initial_pos(self.raw_height());
    }

    fn falling_skip(&mut self, dir_iter: &mut impl Iterator<Item = u8>) {
        // Fast computation for first few cycles where we are certain that there are no adjacent rocks
        for _ in 0..3 {
            match dir_iter.next().unwrap() {
                b'<' => self.move_left(),
                b'>' => self.move_right(),
                _ => panic!("Invalid movement"),
            }
        }

        self.pos.1 -= 3;
    }

    fn clone_state(&self) -> Vec<u8> {
        self.data.clone()
    }

    // True if old rock si fixed and new rock created
    fn step(&mut self, dir: u8) -> bool {
        match dir {
            b'<' => self.move_left(),
            b'>' => self.move_right(),
            _ => panic!("Invalid movement"),
        }

        if !self.move_down() {
            // Cannot move down anymore
            self.insert_rock();
            self.truncate_stack();
            self.falling_new();
            true
        } else {
            false
        }
    }

    fn truncate_stack(&mut self) {
        let offset = match self.falling {
            Rock::Plus => 1,
            _ => 0,
        };

        let add = match self.falling {
            Rock::Vertical => 3,
            Rock::Square => 1,
            _ => 0,
        };

        for i in (self.pos.1 - offset..=self.pos.1 + add).rev() {
            if self.data[i] == 0x7F {
                // Full row
                self.offset += i;
                self.data.drain(..i);
                break;
            }
        }
    }

    fn height(&self) -> usize {
        self.raw_height() + self.offset
    }

    fn raw_height(&self) -> usize {
        self.data.len() - 1
    }

    fn move_right(&mut self) {
        if self.check_right() {
            self.pos.0 += 1;
        }
    }

    fn move_left(&mut self) {
        if self.check_left() {
            self.pos.0 -= 1;
        }
    }

    // True if rock can move down
    fn move_down(&mut self) -> bool {
        if self.check_down() {
            self.pos.1 -= 1;
            true
        } else {
            false
        }
    }

    fn insert_rock(&mut self) {
        match self.falling {
            Rock::Horizontal => {
                if self.pos.1 > self.raw_height() {
                    self.data.resize(self.pos.1 + 1, 0);
                }
                self.data[self.pos.1] |= 0x0F << (self.pos.0);
            }
            Rock::Plus => {
                if self.pos.1 + 1 > self.raw_height() {
                    self.data.resize(self.pos.1 + 1 + 1, 0);
                }
                self.data[self.pos.1 - 1] |= 1 << self.pos.0;
                self.data[self.pos.1] |= 0x07 << (self.pos.0 - 1);
                self.data[self.pos.1 + 1] |= 1 << self.pos.0;
            }
            Rock::L => {
                if self.pos.1 + 2 > self.raw_height() {
                    self.data.resize(self.pos.1 + 2 + 1, 0);
                }
                self.data[self.pos.1] |= 0x07 << (self.pos.0 - 2);
                self.data[self.pos.1 + 1] |= 1 << (self.pos.0);
                self.data[self.pos.1 + 2] |= 1 << (self.pos.0);
            }
            Rock::Vertical => {
                if self.pos.1 + 3 > self.raw_height() {
                    self.data.resize(self.pos.1 + 3 + 1, 0);
                }
                self.data[self.pos.1] |= 1 << self.pos.0;
                self.data[self.pos.1 + 1] |= 1 << self.pos.0;
                self.data[self.pos.1 + 2] |= 1 << self.pos.0;
                self.data[self.pos.1 + 3] |= 1 << self.pos.0;
            }
            Rock::Square => {
                if self.pos.1 + 1 > self.raw_height() {
                    self.data.resize(self.pos.1 + 1 + 1, 0);
                }
                self.data[self.pos.1] |= 0x03 << self.pos.0;
                self.data[self.pos.1 + 1] |= 0x03 << self.pos.0;
            }
        }
    }

    // True if can move right
    fn check_right(&self) -> bool {
        match self.falling {
            Rock::Horizontal => {
                (self.pos.0 + 3) < 6
                    && (self.pos.1 > self.raw_height()
                        || ((1 << (self.pos.0 + 3 + 1)) & self.data[self.pos.1]) == 0)
            }
            Rock::Plus => {
                (self.pos.0 + 1) < 6
                    && (self.pos.1 - 1 > self.raw_height()
                        || ((1 << (self.pos.0 + 1)) & self.data[self.pos.1 - 1]) == 0)
                    && (self.pos.1 > self.raw_height()
                        || ((1 << (self.pos.0 + 1 + 1)) & self.data[self.pos.1]) == 0)
                    && (self.pos.1 + 1 > self.raw_height()
                        || ((1 << (self.pos.0 + 1)) & self.data[self.pos.1 + 1]) == 0)
            }
            Rock::L => {
                (self.pos.0) < 6
                    && (self.pos.1 > self.raw_height()
                        || ((1 << (self.pos.0 + 1)) & self.data[self.pos.1]) == 0)
                    && (self.pos.1 + 1 > self.raw_height()
                        || ((1 << (self.pos.0 + 1)) & self.data[self.pos.1 + 1]) == 0)
                    && (self.pos.1 + 2 > self.raw_height()
                        || ((1 << (self.pos.0 + 1)) & self.data[self.pos.1 + 2]) == 0)
            }
            Rock::Vertical => {
                (self.pos.0) < 6
                    && (self.pos.1 > self.raw_height()
                        || ((1 << (self.pos.0 + 1)) & self.data[self.pos.1]) == 0)
                    && (self.pos.1 + 1 > self.raw_height()
                        || ((1 << (self.pos.0 + 1)) & self.data[self.pos.1 + 1]) == 0)
                    && (self.pos.1 + 2 > self.raw_height()
                        || ((1 << (self.pos.0 + 1)) & self.data[self.pos.1 + 2]) == 0)
                    && (self.pos.1 + 3 > self.raw_height()
                        || ((1 << (self.pos.0 + 1)) & self.data[self.pos.1 + 3]) == 0)
            }
            Rock::Square => {
                (self.pos.0 + 1) < 6
                    && (self.pos.1 > self.raw_height()
                        || ((1 << (self.pos.0 + 1 + 1)) & self.data[self.pos.1]) == 0)
                    && (self.pos.1 + 1 > self.raw_height()
                        || ((1 << (self.pos.0 + 1 + 1)) & self.data[self.pos.1 + 1]) == 0)
            }
        }
    }

    // True if can move left
    fn check_left(&self) -> bool {
        match self.falling {
            Rock::Horizontal => {
                (self.pos.0) > 0
                    && (self.pos.1 > self.raw_height()
                        || ((1 << (self.pos.0 - 1)) & self.data[self.pos.1]) == 0)
            }
            Rock::Plus => {
                (self.pos.0 - 1) > 0
                    && (self.pos.1 - 1 > self.raw_height()
                        || ((1 << (self.pos.0 - 1)) & self.data[self.pos.1 - 1]) == 0)
                    && (self.pos.1 > self.raw_height()
                        || ((1 << (self.pos.0 - 1 - 1)) & self.data[self.pos.1]) == 0)
                    && (self.pos.1 + 1 > self.raw_height()
                        || ((1 << (self.pos.0 - 1)) & self.data[self.pos.1 + 1]) == 0)
            }
            Rock::L => {
                (self.pos.0 - 2) > 0
                    && (self.pos.1 > self.raw_height()
                        || ((1 << (self.pos.0 - 2 - 1)) & self.data[self.pos.1]) == 0)
                    && (self.pos.1 + 1 > self.raw_height()
                        || ((1 << (self.pos.0 - 1)) & self.data[self.pos.1 + 1]) == 0)
                    && (self.pos.1 + 2 > self.raw_height()
                        || ((1 << (self.pos.0 - 1)) & self.data[self.pos.1 + 2]) == 0)
            }
            Rock::Vertical => {
                (self.pos.0) > 0
                    && (self.pos.1 > self.raw_height()
                        || ((1 << (self.pos.0 - 1)) & self.data[self.pos.1]) == 0)
                    && (self.pos.1 + 1 > self.raw_height()
                        || ((1 << (self.pos.0 - 1)) & self.data[self.pos.1 + 1]) == 0)
                    && (self.pos.1 + 2 > self.raw_height()
                        || ((1 << (self.pos.0 - 1)) & self.data[self.pos.1 + 2]) == 0)
                    && (self.pos.1 + 3 > self.raw_height()
                        || ((1 << (self.pos.0 - 1)) & self.data[self.pos.1 + 3]) == 0)
            }
            Rock::Square => {
                (self.pos.0) > 0
                    && (self.pos.1 > self.raw_height()
                        || ((1 << (self.pos.0 - 1)) & self.data[self.pos.1]) == 0)
                    && (self.pos.1 + 1 > self.raw_height()
                        || ((1 << (self.pos.0 - 1)) & self.data[self.pos.1 + 1]) == 0)
            }
        }
    }

    // True if can move down
    fn check_down(&self) -> bool {
        match self.falling {
            Rock::Horizontal => {
                self.pos.1 > 1
                    && (self.pos.1 - 1 > self.raw_height()
                        || ((0x0F << (self.pos.0)) & self.data[self.pos.1 - 1]) == 0)
            }
            Rock::Plus => {
                self.pos.1 - 1 > 1
                    && (self.pos.1 - 1 - 1 > self.raw_height()
                        || ((1 << (self.pos.0)) & self.data[self.pos.1 - 1 - 1]) == 0)
                    && (self.pos.1 - 1 > self.raw_height()
                        || ((0x05 << (self.pos.0 - 1)) & self.data[self.pos.1 - 1]) == 0)
            }
            Rock::L => {
                self.pos.1 > 1
                    && (self.pos.1 - 1 > self.raw_height()
                        || ((0x07 << (self.pos.0 - 2)) & self.data[self.pos.1 - 1]) == 0)
            }
            Rock::Vertical => {
                self.pos.1 > 1
                    && (self.pos.1 - 1 > self.raw_height()
                        || ((1 << (self.pos.0)) & self.data[self.pos.1 - 1]) == 0)
            }
            Rock::Square => {
                self.pos.1 > 1
                    && (self.pos.1 - 1 > self.raw_height()
                        || ((0x03 << (self.pos.0)) & self.data[self.pos.1 - 1]) == 0)
            }
        }
    }

    fn print_top(&self, n: usize) {
        for line in self.data.iter().rev().take(n) {
            println!("{:#010b}", line);
        }
        println!();
    }
}
