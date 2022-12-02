use aoc2022::read_input;

fn main() {
    let input = read_input(2);
    let input = parse_input(&input);

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
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
