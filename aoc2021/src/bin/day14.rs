use std::collections::HashMap;

use aoc;

fn main() {
    let input = aoc::read_input(14);
    let (template, rules) = input.split_once("\n\n").unwrap();
    let template: Vec<char> = template.chars().collect();
    let rules: HashMap<(char, char), char> = rules
        .lines()
        .map(|s| s.split_once(" -> ").unwrap())
        .map(|(front, back)| {
            (
                (front.chars().nth(0).unwrap(), front.chars().nth(1).unwrap()),
                back.chars().nth(0).unwrap(),
            )
        })
        .collect();

    println!("{}", part_1(&template, &rules));
    println!("{}", part_2(&template, &rules));
}

// PART 1
fn part_1(template: &[char], rules: &HashMap<(char, char), char>) -> u64 {
    solve(template, 10, rules)
}

// PART 2
fn part_2(template: &[char], rules: &HashMap<(char, char), char>) -> u64 {
    solve(template, 40, rules)
}

// COMMON
fn solve(template: &[char], steps: u32, rules: &HashMap<(char, char), char>) -> u64 {
    let polymer = expand_polymer(template, steps, rules);
    polymer.values().max().unwrap() - polymer.values().min().unwrap()
}

fn expand_polymer(
    template: &[char],
    steps: u32,
    rules: &HashMap<(char, char), char>,
) -> HashMap<char, u64> {
    let mut polymer_pairs: HashMap<(char, char), u64> = HashMap::new();
    for pair in template.windows(2) {
        *polymer_pairs.entry((pair[0], pair[1])).or_default() += 1;
    }

    for _ in 0..steps {
        let mut polymer_new: HashMap<(char, char), u64> = HashMap::new();
        for (&pair, &count) in polymer_pairs.iter() {
            let &inner = rules.get(&pair).unwrap();
            *polymer_new.entry((pair.0, inner)).or_default() += count;
            *polymer_new.entry((inner, pair.1)).or_default() += count;
        }
        polymer_pairs = polymer_new
    }

    // collapse all pairs
    let mut polymer: HashMap<char, u64> = HashMap::new();
    for (&(c, _), &count) in polymer_pairs.iter() {
        *polymer.entry(c).or_default() += count;
    }
    *polymer.entry(*template.last().unwrap()).or_default() += 1;

    polymer
}
