use std::env;

fn main() {
    let day = env::args().nth(1);
    if let Some(day) = day {
        if let Ok(day) = day.parse::<u8>() {
            let aoc2022::AnswerSet { p1, p2 } = aoc2022::run(match day {
                1 => aoc2022::day01::SOLUTION,
                2 => aoc2022::day02::SOLUTION,
                3 => aoc2022::day03::SOLUTION,
                4 => aoc2022::day04::SOLUTION,
                5 => aoc2022::day05::SOLUTION,
                6 => aoc2022::day06::SOLUTION,
                7 => aoc2022::day07::SOLUTION,
                8 => aoc2022::day08::SOLUTION,
                _ => panic!("Invalid day"),
            });
            println!("Day {}", day);
            println!("Part 1: {}", p1);
            println!("Part 2: {}", p2);
        } else {
            panic!("Day should be a number");
        }
    } else {
        panic!("No day provided");
    }
}
