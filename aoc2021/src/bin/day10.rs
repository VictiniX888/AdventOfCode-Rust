use aoc;

fn main() {
    let input = aoc::read_input(10);
    let input: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

// PART 1
fn part_1(lines: &Vec<Vec<char>>) -> u32 {
    lines
        .iter()
        .filter_map(find_first_illegal_char)
        .map(get_points_1)
        .sum()
}

fn find_first_illegal_char(line: &Vec<char>) -> Option<char> {
    let mut stack: Vec<char> = vec![];
    for &c in line {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                if stack.pop() != Some(get_opposite_bracket(c)) {
                    return Some(c);
                }
            }
            _ => panic!("Unknown char {} found in line", c),
        };
    }

    None
}

fn get_points_1(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Could not get points of unexpected char {}", c),
    }
}

// PART 2
fn part_2(lines: &Vec<Vec<char>>) -> u64 {
    let mut scores: Vec<u64> = lines.iter().filter_map(get_total_score).collect();

    scores.sort();

    scores[scores.len() / 2]
}

fn get_total_score(line: &Vec<char>) -> Option<u64> {
    let mut stack: Vec<char> = vec![];
    for &c in line {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                if stack.pop() != Some(get_opposite_bracket(c)) {
                    return None;
                }
            }
            _ => panic!("Unknown char {} found in line", c),
        };
    }

    let score = stack
        .iter()
        .rev()
        .fold(0, |acc, &c| acc * 5 + get_points_2(c));

    Some(score)
}

fn get_points_2(c: char) -> u64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("Could not get points of unexpected char {}", c),
    }
}

// COMMON
fn get_opposite_bracket(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("Could not get opposite of unexpected char {}", c),
    }
}
