use aoc;

fn main() {
    let input = aoc::read_input(1);
    let input: Vec<i32> = input.lines().filter_map(|s| s.parse().ok()).collect();

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(input: &Vec<i32>) -> usize {
    count_depth_increases(input, 1)
}

fn part_2(input: &Vec<i32>) -> usize {
    count_depth_increases(input, 3)
}

fn count_depth_increases(input: &Vec<i32>, gap: usize) -> usize {
    input
        .iter()
        .zip(input.iter().skip(gap))
        .filter(|(i, j)| j > i)
        .count()
}
