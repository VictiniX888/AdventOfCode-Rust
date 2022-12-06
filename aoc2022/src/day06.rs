use crate::*;

pub const SOLUTION: Solution = Solution { day: 6, solve };

// ~25 us
fn solve(input: &str) -> AnswerSet {
    let iter = input.bytes();

    let mut p1 = 0;
    let mut p2 = 0;

    let mut unique_count = 0;

    let mut history = [0; 13];

    for (i, char) in iter.enumerate() {
        if let Some((last_seen, _)) = history
            .iter()
            .rev()
            .take(unique_count)
            .enumerate()
            .find(|(_, &other)| char == other)
        {
            if last_seen < unique_count {
                unique_count = last_seen;
            }
        }

        if unique_count >= 13 {
            p2 = i + 1;
            break;
        } else if unique_count >= 3 && p1 == 0 {
            p1 = i + 1;
        }

        shift_mut_left(&mut history);
        history[12] = char;

        unique_count += 1;
    }

    AnswerSet {
        p1: Answer::Usize(p1),
        p2: Answer::Usize(p2),
    }
}

fn shift_mut_left(slice: &mut [u8]) {
    for i in 0..slice.len() - 1 {
        slice[i] = slice[i + 1];
    }
}
