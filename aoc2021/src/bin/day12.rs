use std::collections::HashMap;

use aoc;

fn main() {
    let input = aoc::read_input(12);
    let input = input.lines().map(|s| s.split_once('-').unwrap());
    let input: HashMap<&str, Vec<&str>> = input.fold(HashMap::new(), |mut acc, (a, b)| {
        acc.entry(a).or_default().push(b);
        acc.entry(b).or_default().push(a);
        acc
    });

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

// PART 1
fn part_1(connections: &HashMap<&str, Vec<&str>>) -> u32 {
    dfs(START, connections, vec![], true)
}

// PART 2
fn part_2(connections: &HashMap<&str, Vec<&str>>) -> u32 {
    dfs(START, connections, vec![], false)
}

// COMMON
const START: &str = "start";
const END: &str = "end";

fn dfs<'a>(
    current: &'a str,
    connections: &HashMap<&str, Vec<&str>>,
    mut visited: Vec<&'a str>,
    repeated: bool,
) -> u32 {
    if current == END {
        return 1;
    }

    if current.to_lowercase() == current {
        visited.push(current);
    }

    connections
        .get(current)
        .unwrap()
        .iter()
        .map(|next| (next, visited.contains(next)))
        .filter(|&(next, has_visited)| *next != START && (!repeated || !has_visited))
        .map(|(next, has_visited)| {
            dfs(next, connections, visited.to_vec(), repeated || has_visited)
        })
        .sum()
}
