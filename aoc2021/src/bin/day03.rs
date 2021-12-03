use std::collections::HashMap;

use aoc;

fn main() {
    let input = aoc::read_input(3);
    let input: Vec<Vec<bool>> = input.lines().map(parse_binary_str).collect();

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

// PART 1
fn part_1(input: &Vec<Vec<bool>>) -> usize {
    let gamma_binary = get_most_common_bits(input);
    let gamma_rate = get_int_from_binary(&gamma_binary);

    let mut epsilon_binary = gamma_binary;
    epsilon_binary.iter_mut().for_each(|bit| *bit = !*bit);

    let epsilon_rate = get_int_from_binary(&epsilon_binary);

    gamma_rate * epsilon_rate
}

fn get_most_common_bits(binary_ints: &Vec<Vec<bool>>) -> Vec<bool> {
    let mut bit_count: HashMap<usize, i32> = HashMap::new();

    for binary in binary_ints {
        for (i, &bit) in binary.iter().enumerate() {
            let count = bit_count.entry(i).or_insert(0);
            if bit {
                *count += 1;
            } else {
                *count -= 1;
            }
        }
    }

    // Collect all bits into bit array
    let mut most_common_bits = vec![false; bit_count.len()];
    for (i, count) in bit_count {
        most_common_bits[i] = if count >= 0 { true } else { false }
    }

    most_common_bits
}

// PART 2
fn part_2(input: &Vec<Vec<bool>>) -> usize {
    let mut i = 0;
    let mut filtered_bins: Vec<Vec<bool>> = input.clone();
    let oxygen_binary = loop {
        if filtered_bins.len() == 1 {
            break &filtered_bins[0];
        }

        let most_common_bit = get_most_common_bit(&filtered_bins, i);
        filtered_bins = filtered_bins
            .into_iter()
            .filter(|binary| binary[i] == most_common_bit)
            .collect();

        i += 1;
    };
    let oxygen_rating = get_int_from_binary(oxygen_binary);

    i = 0;
    filtered_bins = input.clone();
    let co2_binary = loop {
        if filtered_bins.len() == 1 {
            break &filtered_bins[0];
        }

        let least_common_bit = !get_most_common_bit(&filtered_bins, i);
        filtered_bins = filtered_bins
            .into_iter()
            .filter(|binary| binary[i] == least_common_bit)
            .collect();

        i += 1;
    };
    let co2_rating = get_int_from_binary(&co2_binary);

    oxygen_rating * co2_rating
}

fn get_most_common_bit(binary_ints: &Vec<Vec<bool>>, index: usize) -> bool {
    let sum = binary_ints
        .iter()
        .map(|binary| binary[index])
        .fold(0, |acc, bit| acc + if bit { 1 } else { -1 });

    sum >= 0
}

// COMMON
fn parse_binary_str(str: &str) -> Vec<bool> {
    str.chars().map(parse_bit).collect()
}

fn parse_bit(c: char) -> bool {
    match c {
        '0' => false,
        '1' => true,
        _ => panic!("Failed to pase unknown char {} as bool", c),
    }
}

fn get_int_from_binary(binary: &Vec<bool>) -> usize {
    binary.iter().rev().enumerate().fold(0, |acc, (i, bit)| {
        acc + 2_usize.pow(i.try_into().unwrap()) * usize::from(*bit)
    })
}
