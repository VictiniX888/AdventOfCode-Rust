use std::collections::HashMap;

use aoc;

fn main() {
    let input = aoc::read_input(6);
    let input = input.trim();
    let input: Vec<usize> = input.split(',').map(|s| s.parse().unwrap()).collect();

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

// PART 1
fn part_1(timers: &[usize]) -> u64 {
    const DAYS: usize = 80;

    timers.iter().map(|&age| count_fish(age, DAYS)).sum()
}

// PART 2
fn part_2(timers: &[usize]) -> u64 {
    const DAYS: usize = 256;

    let mut cache: HashMap<usize, u64> = HashMap::new();

    let mut sum: u64 = 0;
    for &age in timers {
        sum += *cache.entry(age).or_insert(count_fish(age, DAYS))
    }

    sum
}

// COMMON
const CYCLE: usize = 7;
const VEC_SIZE: usize = 9;

fn count_fish(age: usize, days: usize) -> u64 {
    let mut age_map = [0; VEC_SIZE];
    age_map[age] += 1;

    for day in 1..=days {
        let index = (day - 1) % CYCLE;

        let children_to_conform = age_map[VEC_SIZE - 2];
        age_map[VEC_SIZE - 2] = age_map[VEC_SIZE - 1];

        age_map[VEC_SIZE - 1] = age_map[index];

        age_map[index % CYCLE] += children_to_conform;
    }

    age_map.iter().sum()
}

/* How it works:
    [0, 0, 0, 1, 0, 0, 0, 0, 0]
     ^
    [0, 0, 0, 1, 0, 0, 0, 0, 0]
        ^
    [0, 0, 0, 1, 0, 0, 0, 0, 0]
           ^
    [0, 0, 0, 1, 0, 0, 0, 0, 1]
              ^              +
    [0, 0, 0, 1, 0, 0, 0, 1, 0]
                 ^         <<
    [0, 0, 0, 1, 0, 1, 0, 0, 0]
                    ^+
    ...
*/
