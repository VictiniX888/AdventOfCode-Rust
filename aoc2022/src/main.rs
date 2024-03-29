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
                9 => aoc2022::day09::SOLUTION,
                10 => aoc2022::day10::SOLUTION,
                11 => aoc2022::day11::SOLUTION,
                12 => aoc2022::day12::SOLUTION,
                13 => aoc2022::day13::SOLUTION,
                14 => aoc2022::day14::SOLUTION,
                15 => aoc2022::day15::SOLUTION,
                16 => aoc2022::day16::SOLUTION,
                17 => aoc2022::day17::SOLUTION,
                18 => aoc2022::day18::SOLUTION,
                19 => aoc2022::day19::SOLUTION,
                20 => aoc2022::day20::SOLUTION,
                21 => aoc2022::day21::SOLUTION,
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
