use std::collections::VecDeque;

use crate::*;

pub const SOLUTION: Solution = Solution { day: 20, solve };

// ~ 100 ms
fn solve(input: &str) -> AnswerSet {
    let mut nums = parse_input(input);
    let mut indices = (0..nums.len()).collect::<VecDeque<_>>();

    // Part 1
    for i in 0..indices.len() {
        mix(&nums, &mut indices, i);
    }

    let pos_0_original = nums.iter().position(|&num| num == 0).unwrap();
    let pos_0 = indices.iter().position(|&i| i == pos_0_original).unwrap();
    let p1 = nums[indices[(pos_0 + 1000) % nums.len()]]
        + nums[indices[(pos_0 + 2000) % nums.len()]]
        + nums[indices[(pos_0 + 3000) % nums.len()]];

    // Part 2
    for num in nums.iter_mut() {
        *num *= 811589153;
    }
    let mut indices = (0..nums.len()).collect::<VecDeque<_>>();

    for _ in 0..10 {
        for i in 0..indices.len() {
            mix(&nums, &mut indices, i);
        }
    }

    let pos_0 = indices.iter().position(|&i| i == pos_0_original).unwrap();
    let p2 = nums[indices[(pos_0 + 1000) % nums.len()]]
        + nums[indices[(pos_0 + 2000) % nums.len()]]
        + nums[indices[(pos_0 + 3000) % nums.len()]];

    AnswerSet {
        p1: Answer::I64(p1),
        p2: Answer::I64(p2),
    }
}

fn mix(nums: &[i64], indices: &mut VecDeque<usize>, i: usize) {
    let diff = nums[i] % (indices.len() as i64 - 1);
    let i = indices.iter().position(|&index| index == i).unwrap();
    if diff >= 0 {
        mix_positive(indices, i, diff as usize);
    } else {
        mix_negative(indices, i, -diff as usize);
    }
}

fn mix_negative(indices: &mut VecDeque<usize>, i: usize, diff: usize) {
    indices.rotate_right(indices.len() - i - 1);
    let index = indices.pop_back().unwrap();
    indices.rotate_right(diff);
    indices.push_back(index);
}

fn mix_positive(indices: &mut VecDeque<usize>, i: usize, diff: usize) {
    indices.rotate_left(i);
    let index = indices.pop_front().unwrap();
    indices.rotate_left(diff);
    indices.push_front(index);
}

fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}
