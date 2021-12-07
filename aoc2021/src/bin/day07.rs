use std::cmp;

use aoc;

fn main() {
    let input = aoc::read_input(7);
    let input = input.trim();
    let input: Vec<u32> = input.split(',').map(aoc::parse_str).collect();

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

// PART 1
fn part_1(positions: &[u32]) -> u32 {
    let &max_pos = positions.iter().max().unwrap();
    (0..=max_pos)
        .map(|pos| {
            positions
                .iter()
                .map(|&init_pos| cmp::max(init_pos, pos) - cmp::min(init_pos, pos))
                .sum()
        })
        .min()
        .unwrap()
}

// PART 2
fn part_2(positions: &[u32]) -> u32 {
    let &max_pos = positions.iter().max().unwrap();
    (0..=max_pos)
        .map(|pos| {
            positions
                .iter()
                .map(|&init_pos| {
                    let n = cmp::max(init_pos, pos) - cmp::min(init_pos, pos);
                    n * (n + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}
