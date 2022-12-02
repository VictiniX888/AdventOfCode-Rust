use aoc2022::read_input;

fn main() {
    let input = read_input(1);
    let input = parse_input(&input);

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .split_terminator("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|n| n.parse::<u32>().expect("Failed to parse input as number"))
                .collect()
        })
        .collect()
}

fn part_1(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|group| group.iter().sum())
        .max()
        .expect("Empty input iterator")
}

fn part_2(input: &[Vec<u32>]) -> u32 {
    let mut calories: Vec<u32> = input.iter().map(|group| group.iter().sum()).collect();
    calories.sort();

    calories.iter().rev().take(3).sum()
}
