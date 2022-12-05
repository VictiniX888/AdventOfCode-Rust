use crate::*;
use std::collections::HashSet;

pub const SOLUTION: Solution = Solution {
    day: 3,
    solve: solve_bitmask,
};

/* ======== OPTIMIZED (BITMASK) ======== */
// ~37 us
fn solve_bitmask(input: &str) -> AnswerSet {
    let iter = input
        .lines()
        .map(|line| line.bytes().map(|byte| byte_to_priority(byte)));

    let mut p1_sum = 0;
    let mut p2_sum = 0;
    let mut unique: u64 = 0;

    for (i, mut bag) in iter.enumerate() {
        // Part 1
        let mut front: u64 = 0;
        let mut back: u64 = 0;
        for _ in 0..bag.len() / 2 {
            front |= 1 << bag.next().unwrap();
        }
        for _ in 0..bag.len() {
            back |= 1 << bag.next().unwrap();
        }
        p1_sum += find_common_bit(&front, &back);

        // Part 2
        let bag = front | back;
        if i % 3 == 2 {
            if unique != 0 {
                p2_sum += find_set_bit_pos(&unique);
            }
            unique = bag;
        } else {
            unique &= bag;
        }
    }
    p2_sum += find_set_bit_pos(&unique);

    AnswerSet {
        p1: Answer::U16(p1_sum),
        p2: Answer::U16(p2_sum),
    }
}

fn find_set_bit_pos(bits: &u64) -> u16 {
    for i in 1..53 {
        if bits & (1 << i) != 0 {
            return i;
        }
    }

    panic!()
}

fn find_common_bit(bits1: &u64, bits2: &u64) -> u16 {
    let and = bits1 & bits2;
    find_set_bit_pos(&and)
}

/* ======== OPTIMIZED (INTERSECTION) ======== */
// ~230 us
fn solve_optimized(input: &str) -> AnswerSet {
    let iter = input.lines();

    let mut p1_sum = 0;
    let mut p2_sum = 0;
    let mut unique: Vec<u8> = Vec::new();
    let mut half: Vec<u8> = Vec::new();

    for (i, bag) in iter.enumerate() {
        // Part 2
        if i % 3 == 0 {
            p2_sum += byte_to_priority(*unique.iter().next().unwrap_or(&(&b'a' - 1))) as u16;
            unique.clear();
            unique.extend(bag.bytes());
        } else {
            let items: HashSet<u8> = HashSet::from_iter(bag.bytes());
            unique.retain(|item| items.contains(item));
        }

        // Part 1
        let (front, back) = bag.split_at(bag.len() / 2);
        half.extend(front.bytes());
        let mut intersection = back.bytes().filter(|item| half.contains(item));
        p1_sum += byte_to_priority(intersection.next().unwrap()) as u16;
        half.clear();
    }
    p2_sum += byte_to_priority(*unique.iter().next().unwrap_or(&0)) as u16;

    AnswerSet {
        p1: Answer::U16(p1_sum),
        p2: Answer::U16(p2_sum),
    }
}

fn byte_to_priority(byte: u8) -> u8 {
    if byte >= b'A' && byte <= b'Z' {
        byte + 27 - b'A'
    } else {
        byte + 1 - b'a'
    }
}

/* ======== FIRST ATTEMPT ======== */
// ~650 us
fn solve(input: &str) -> AnswerSet {
    let input = parse_input(input);
    AnswerSet {
        p1: Answer::U32(part_1(&input)),
        p2: Answer::U32(part_2(&input)),
    }
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|&b| {
                    if b >= b'a' && b <= b'z' {
                        b - b'a' + 1
                    } else {
                        b - b'A' + 27
                    }
                })
                .collect()
        })
        .collect()
}

fn part_1(input: &[Vec<u8>]) -> u32 {
    input
        .iter()
        .map(|bag| {
            *(HashSet::<u8>::from_iter(bag.iter().take(bag.len() / 2).cloned())
                .intersection(&HashSet::from_iter(
                    bag.into_iter().skip(bag.len() / 2).cloned(),
                ))
                .next()
                .expect("No common element")) as u32
        })
        .sum()
}

fn part_2(input: &[Vec<u8>]) -> u32 {
    input
        .chunks_exact(3)
        .map(|group| {
            *(group
                .iter()
                .map(|bag| HashSet::<u8>::from_iter(bag.iter().cloned()))
                .reduce(|acc, bag| acc.intersection(&bag).cloned().collect())
                .expect("Group is empty")
                .iter()
                .next()
                .expect("No badge found")) as u32
        })
        .sum()
}
