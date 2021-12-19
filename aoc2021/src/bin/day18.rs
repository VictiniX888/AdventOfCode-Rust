use aoc;

fn main() {
    let input = aoc::read_input(18);
    let input: Vec<Vec<SnailNum>> = input
        .lines()
        .map(|s| s.chars().filter_map(SnailNum::from_char).collect())
        .collect();

    println!("{}", part_1(input.to_vec()));
    println!("{}", part_2(&input));
}

// PART 1
fn part_1(nums: Vec<Vec<SnailNum>>) -> u32 {
    let sum = nums.into_iter().reduce(SnailNum::add).unwrap();

    SnailNum::get_magnitude(sum)
}

// PART 2
fn part_2(nums: &Vec<Vec<SnailNum>>) -> u32 {
    let mut max = 0;
    for a in nums {
        for b in nums {
            let sum = SnailNum::get_magnitude(SnailNum::add(a.to_vec(), b.to_vec()));
            if sum > max {
                max = sum;
            }

            let sum = SnailNum::get_magnitude(SnailNum::add(b.to_vec(), a.to_vec()));
            if sum > max {
                max = sum;
            }
        }
    }

    max
}

// COMMON
#[derive(Debug, Clone)]
enum SnailNum {
    Num(u32),
    Down,
    Up,
}

impl SnailNum {
    fn from_char(c: char) -> Option<SnailNum> {
        if let Some(num) = c.to_digit(10) {
            Some(SnailNum::Num(num))
        } else {
            match c {
                '[' => Some(SnailNum::Down),
                ']' => Some(SnailNum::Up),
                _ => None,
            }
        }
    }

    fn add(mut lhs: Vec<SnailNum>, mut rhs: Vec<SnailNum>) -> Vec<SnailNum> {
        let mut result = Vec::with_capacity(lhs.len() + rhs.len() + 2);
        result.push(SnailNum::Down);
        result.append(&mut lhs);
        result.append(&mut rhs);
        result.push(SnailNum::Up);

        SnailNum::reduce(&mut result);

        result
    }

    fn reduce(nums: &mut Vec<SnailNum>) {
        let mut reduced = false;
        'outer: while !reduced {
            let mut depth = 0;
            let mut split_i = None;
            for (i, num) in nums.iter().enumerate() {
                if depth > 4 {
                    SnailNum::explode(nums, i);
                    continue 'outer;
                }

                match num {
                    SnailNum::Down => depth += 1,
                    SnailNum::Up => depth -= 1,
                    SnailNum::Num(num) => {
                        if *num >= 10 && split_i == None {
                            split_i = Some(i);
                        }
                    }
                }
            }

            if let Some(split_i) = split_i {
                SnailNum::split(nums, split_i);
                continue 'outer;
            }

            reduced = true;
        }
    }

    fn explode(nums: &mut Vec<SnailNum>, i: usize) {
        // Add left
        if let SnailNum::Num(left_old) = nums[i] {
            if let Some(i) = SnailNum::seek_left(nums, i) {
                if let SnailNum::Num(left_new) = nums[i] {
                    nums[i] = SnailNum::Num(left_old + left_new);
                } else {
                    panic!("Seek left was not a number");
                }
            }
        } else {
            panic!("Explode position was not a number");
        }

        // Add right
        if let SnailNum::Num(right_old) = nums[i + 1] {
            if let Some(i) = SnailNum::seek_right(nums, i + 1) {
                if let SnailNum::Num(right_new) = nums[i] {
                    nums[i] = SnailNum::Num(right_old + right_new);
                } else {
                    panic!("Seek right was not a number");
                }
            }
        } else {
            panic!("Explode position was not a number");
        }

        // Replace with 0
        nums.splice(i - 1..=i + 2, [SnailNum::Num(0)]);
    }

    fn split(nums: &mut Vec<SnailNum>, i: usize) {
        if let SnailNum::Num(old) = nums[i] {
            let left = SnailNum::Num(old / 2);
            let right = SnailNum::Num((old + 1) / 2);
            nums.splice(i..i + 1, [SnailNum::Down, left, right, SnailNum::Up]);
        } else {
            panic!("Split position was not a number");
        }
    }

    fn seek_left(nums: &Vec<SnailNum>, i: usize) -> Option<usize> {
        for i in (0..i).rev() {
            if let SnailNum::Num(_) = nums[i] {
                return Some(i);
            }
        }

        None
    }

    fn seek_right(nums: &Vec<SnailNum>, i: usize) -> Option<usize> {
        for i in i + 1..nums.len() {
            if let SnailNum::Num(_) = nums[i] {
                return Some(i);
            }
        }

        None
    }

    fn get_magnitude(mut nums: Vec<SnailNum>) -> u32 {
        let mut i = 0;
        while nums.len() > 1 {
            if matches!(nums[i], SnailNum::Down) && matches!(nums[i + 3], SnailNum::Up) {
                if let SnailNum::Num(left) = nums[i + 1] {
                    if let SnailNum::Num(right) = nums[i + 2] {
                        let magnitude = left * 3 + right * 2;
                        nums.splice(i..=i + 3, [SnailNum::Num(magnitude)]);
                        if i > 0 && matches!(nums[i - 1], SnailNum::Down) {
                            i -= 1;
                        } else if i > 1 {
                            i -= 2;
                        }
                    } else {
                        panic!("Right number not found in pair")
                    }
                } else {
                    panic!("Left number not found in pair")
                }
            } else {
                i += 1;
            }
        }

        if let SnailNum::Num(magnitude) = nums[0] {
            magnitude
        } else {
            panic!("Magnitude is not a number")
        }
    }

    // Debugging
    #[allow(dead_code)]
    fn to_string(nums: &Vec<SnailNum>) -> String {
        nums.iter()
            .map(|num| match num {
                SnailNum::Down => String::from("["),
                SnailNum::Up => String::from("]"),
                SnailNum::Num(num) => num.to_string(),
            })
            .collect()
    }
}
