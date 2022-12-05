use crate::*;

pub const SOLUTION: Solution = Solution { day: 4, solve };

// 33 us
fn solve(input: &str) -> AnswerSet {
    let mut p1_sum = 0;
    let mut p2_sum = 0;

    let iter = input.lines().map(|line| {
        let mut ascii_iter = line.bytes();
        (
            (
                parse_u8_from_ascii_iter(&mut ascii_iter),
                parse_u8_from_ascii_iter(&mut ascii_iter),
            ),
            (
                parse_u8_from_ascii_iter(&mut ascii_iter),
                parse_u8_from_ascii_iter(&mut ascii_iter),
            ),
        )
    });

    for ((range1_start, range1_end), (range2_start, range2_end)) in iter {
        p1_sum += ((range1_start >= range2_start && range1_end <= range2_end)
            || (range2_start >= range1_start && range2_end <= range1_end)) as u16;

        p2_sum += (range1_end >= range2_start && range2_end >= range1_start) as u16;
    }

    AnswerSet {
        p1: Answer::U16(p1_sum),
        p2: Answer::U16(p2_sum),
    }
}

fn parse_u8_from_ascii_iter(ascii_iter: &mut impl Iterator<Item = u8>) -> u8 {
    let mut num = 0;

    while let Some(byte) = ascii_iter.next() {
        if !byte.is_ascii_digit() {
            break;
        }

        num *= 10;
        num += byte - b'0';
    }

    num
}
