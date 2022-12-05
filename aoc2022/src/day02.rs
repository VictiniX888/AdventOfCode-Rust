use crate::*;

pub const SOLUTION: Solution = Solution {
    day: 2,
    solve: solve_optimized,
};

/* ======== OPTIMIZED (MATH) ======== */
// (~30 us)
fn solve_optimized(input: &str) -> AnswerSet {
    let iter = input.lines();

    let (p1, p2) = iter
        .map(|line| {
            let mut line = line.bytes();
            // Outcome is player hand for part 1, match outcome for part 2
            let (opponent, outcome) = (line.next().unwrap(), line.skip(1).next().unwrap());
            let diff = (outcome - opponent + 2) % 3;
            let outcome_norm = outcome - b'X';
            let score1 = outcome_norm + 1 + (diff * 3);
            let player = (opponent - b'A' + outcome_norm + 2) % 3 + 1;
            let score2 = outcome_norm * 3 + player;
            (score1, score2)
        })
        .fold((0, 0), |acc, (score1, score2)| {
            (acc.0 + score1 as u16, acc.1 + score2 as u16)
        });

    AnswerSet {
        p1: Answer::U16(p1),
        p2: Answer::U16(p2),
    }
}

/* ======== OPTIMIZED (MATCHERS) ======== */
// (~40 us)
fn solve_match(input: &str) -> AnswerSet {
    let iter = input.lines();

    let (p1, p2) = iter
        .map(|line| {
            let mut line = line.bytes();
            // Outcome is player hand for part 1, match outcome for part 2
            let (opponent, outcome) = (line.next().unwrap(), line.skip(1).next().unwrap());
            let score1 = part_1_match(opponent, outcome);
            let score2 = part_2_match(opponent, outcome);
            (score1, score2)
        })
        .fold((0, 0), |acc, (score1, score2)| {
            (acc.0 + score1, acc.1 + score2)
        });

    AnswerSet {
        p1: Answer::U16(p1),
        p2: Answer::U16(p2),
    }
}

fn part_1_match(opponent: u8, player: u8) -> u16 {
    match (opponent, player) {
        (b'B', b'X') => 1,
        (b'C', b'Y') => 2,
        (b'A', b'Z') => 3,
        (b'A', b'X') => 4,
        (b'B', b'Y') => 5,
        (b'C', b'Z') => 6,
        (b'C', b'X') => 7,
        (b'A', b'Y') => 8,
        (b'B', b'Z') => 9,
        _ => unreachable!(),
    }
}

fn part_2_match(opponent: u8, outcome: u8) -> u16 {
    match (opponent, outcome) {
        (b'B', b'X') => 1,
        (b'C', b'X') => 2,
        (b'A', b'X') => 3,
        (b'A', b'Y') => 4,
        (b'B', b'Y') => 5,
        (b'C', b'Y') => 6,
        (b'C', b'Z') => 7,
        (b'A', b'Z') => 8,
        (b'B', b'Z') => 9,
        _ => unreachable!(),
    }
}

/* ======== FIRST ATTEMPT ======== */
// (~110 us)
fn solve(input: &str) -> AnswerSet {
    let input = parse_input(input);
    AnswerSet {
        p1: Answer::U32(part_1(&input)),
        p2: Answer::U32(part_2(&input)),
    }
}

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| line.split_once(' ').expect("Input format incorrect"))
        .collect()
}

fn parse_input_part_1(input: &[(&str, &str)]) -> Vec<Round> {
    input
        .iter()
        .map(|round| {
            let opponent = Hand::from_literal(round.0);
            let player = Hand::from_literal(round.1);
            Round {
                player,
                outcome: Outcome::from_hands(player, opponent),
            }
        })
        .collect()
}

fn part_1(input: &[(&str, &str)]) -> u32 {
    sum_scores(&parse_input_part_1(input))
}

fn parse_input_part_2(input: &[(&str, &str)]) -> Vec<Round> {
    input
        .iter()
        .map(|round| {
            let opponent = Hand::from_literal(round.0);
            let outcome = Outcome::from_literal(round.1);
            Round {
                player: Hand::from_outcome(opponent, outcome),
                outcome,
            }
        })
        .collect()
}

fn part_2(input: &[(&str, &str)]) -> u32 {
    sum_scores(&parse_input_part_2(input))
}

fn sum_scores(rounds: &[Round]) -> u32 {
    rounds.iter().map(|round| round.get_score()).sum()
}

struct Round {
    player: Hand,
    outcome: Outcome,
}

impl Round {
    fn get_score(&self) -> u32 {
        self.outcome.score() + self.player.score()
    }
}

#[derive(Clone, Copy)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn from_literal(s: &str) -> Outcome {
        match s {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Invalid outcome"),
        }
    }

    fn from_hands(player: Hand, opponent: Hand) -> Outcome {
        if player == opponent {
            Outcome::Draw
        } else if (player == Hand::Rock && opponent == Hand::Scissors)
            || (player == Hand::Paper && opponent == Hand::Rock)
            || (player == Hand::Scissors && opponent == Hand::Paper)
        {
            Outcome::Win
        } else {
            Outcome::Lose
        }
    }

    fn score(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Lose => 0,
            Self::Draw => 3,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn from_literal(s: &str) -> Self {
        match s {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => panic!("Invalid hand"),
        }
    }

    fn from_outcome(opponent: Hand, outcome: Outcome) -> Self {
        match outcome {
            Outcome::Draw => opponent,
            Outcome::Win => match opponent {
                Hand::Paper => Hand::Scissors,
                Hand::Rock => Hand::Paper,
                Hand::Scissors => Hand::Rock,
            },
            Outcome::Lose => match opponent {
                Hand::Paper => Hand::Rock,
                Hand::Rock => Hand::Scissors,
                Hand::Scissors => Hand::Paper,
            },
        }
    }

    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}
