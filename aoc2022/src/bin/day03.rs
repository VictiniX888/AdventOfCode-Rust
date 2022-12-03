use std::collections::HashSet;

use aoc2022::read_input;

fn main() {
    let input = read_input(3);
    let input = parse_input(&input);

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
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
