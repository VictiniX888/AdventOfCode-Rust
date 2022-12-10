use crate::*;

pub const SOLUTION: Solution = Solution { day: 10, solve };

// ~3 us
fn solve(input: &str) -> AnswerSet {
    let mut instructions = input.lines();

    let mut cpu = CPU::new();
    let (p1, p2) = cpu.run(240, &mut instructions);

    AnswerSet {
        p1: Answer::I32(p1),
        p2: Answer::String(p2),
    }
}

struct CPU {
    x: i32,
    cycle: usize,
    memory: i8,
    waiting: bool,
}

impl CPU {
    fn new() -> Self {
        CPU {
            x: 1,
            cycle: 1,
            memory: 0,
            waiting: false,
        }
    }

    fn run<'a>(
        &mut self,
        cycles: usize,
        instructions: &mut impl Iterator<Item = &'a str>,
    ) -> (i32, String) {
        let mut signal_sum = 0;
        let mut pixels = String::with_capacity(246);
        pixels.push('\n');

        loop {
            if self.cycle % 40 == 20 {
                signal_sum += self.cycle as i32 * self.x;
            }

            // Draw pixel
            pixels.push(if (self.x - ((self.cycle - 1) % 40) as i32).abs() <= 1 {
                '#'
            } else {
                '.'
            });

            if self.cycle >= cycles {
                break;
            }

            if self.cycle % 40 == 0 {
                pixels.push('\n');
            }

            if self.waiting {
                self.x += self.memory as i32;
                self.waiting = false;
            } else if let Some(instruction) = instructions.next() {
                let mut iter = instruction.bytes();
                match iter.next().unwrap() {
                    b'a' => {
                        iter.nth(3);
                        self.memory = parse_i8_from_ascii_iter(&mut iter);
                        self.waiting = true;
                    }
                    _ => {}
                }
            } else {
                break;
            }

            self.cycle += 1;
        }

        (signal_sum, pixels)
    }
}

fn parse_i8_from_ascii_iter(ascii_iter: &mut impl Iterator<Item = u8>) -> i8 {
    let mut num = 0;

    let front = ascii_iter.next().unwrap();
    if front.is_ascii_digit() {
        num += front - b'0';
    }

    while let Some(byte) = ascii_iter.next() {
        if !byte.is_ascii_digit() {
            break;
        }

        num *= 10;
        num += byte - b'0';
    }

    let num = if front == b'-' {
        num as i8 * -1
    } else {
        num as i8
    };

    num
}
