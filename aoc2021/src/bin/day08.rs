use std::collections::HashMap;
use std::collections::HashSet;

use aoc;

fn main() {
    let input = aoc::read_input(8);
    let input: Vec<(Vec<&str>, Vec<&str>)> = input
        .lines()
        .map(|s| s.split_once(" | ").unwrap())
        .map(|(front, back)| {
            (
                front.split_whitespace().collect(),
                back.split_whitespace().collect(),
            )
        })
        .collect();

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

// PART 1
const UNIQUE_SEGMENTS: [usize; 4] = [2, 3, 4, 7];

fn part_1(input: &[(Vec<&str>, Vec<&str>)]) -> usize {
    input
        .iter()
        .map(|(_, back)| {
            back.iter()
                .filter(|s| UNIQUE_SEGMENTS.contains(&s.len()))
                .count()
        })
        .sum()
}

// PART 2
fn part_2(input: &[(Vec<&str>, Vec<&str>)]) -> u32 {
    input
        .iter()
        .map(|(patterns, output)| get_output_value(output, &map_segments(patterns)))
        .sum()
}

// Only maps c, d, f segments, as only they are required to deduce the digits
fn map_segments(patterns: &[&str]) -> HashMap<char, char> {
    let one = &find_strs_as_char_set(patterns, |s| s.len() == 2)[0];
    let four = &find_strs_as_char_set(patterns, |s| s.len() == 4)[0];

    // 0, 6, 9
    let len_six = find_strs_as_char_set(patterns, |s| s.len() == 6);

    // Determine segment c by comparing 1 with 0, 6, 9
    let &c = len_six
        .iter()
        .find_map(|chars| one.difference(chars).next())
        .unwrap();

    // Determine segment f by 1 - c
    let &f = one.iter().find(|&&char| char != c).unwrap();

    // Determine segment d by comparing 4 with 0, 6, 9
    let &d = len_six
        .iter()
        .filter_map(|chars| four.difference(chars).next())
        .find(|&&char| char != c)
        .unwrap();

    HashMap::from([('c', c), ('d', d), ('f', f)])
}

fn find_strs_as_char_set<P>(strings: &[&str], predicate: P) -> Vec<HashSet<char>>
where
    P: for<'r> FnMut(&'r &&str) -> bool,
{
    strings
        .iter()
        .filter(predicate)
        .map(|s| s.chars().collect())
        .collect()
}

fn get_output_value(signal_patterns: &[&str], segments: &HashMap<char, char>) -> u32 {
    signal_patterns
        .iter()
        .map(|signal_pattern| get_digit(signal_pattern, segments))
        .reduce(|acc, digit| acc * 10 + digit)
        .unwrap()
}

fn get_digit(signal_pattern: &str, segments: &HashMap<char, char>) -> u32 {
    match signal_pattern.len() {
        2 => 1,
        3 => 7,
        4 => 4,
        5 => {
            if !signal_pattern
                .chars()
                .any(|c| c == *segments.get(&'f').unwrap())
            {
                2
            } else if !signal_pattern
                .chars()
                .any(|c| c == *segments.get(&'c').unwrap())
            {
                5
            } else {
                3
            }
        }
        6 => {
            if !signal_pattern
                .chars()
                .any(|c| c == *segments.get(&'d').unwrap())
            {
                0
            } else if !signal_pattern
                .chars()
                .any(|c| c == *segments.get(&'c').unwrap())
            {
                6
            } else {
                9
            }
        }
        7 => 8,
        _ => panic!("Unknown signal pattern {} encountered!", signal_pattern),
    }
}
