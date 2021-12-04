use aoc;

fn main() {
    let input = aoc::read_input(4);
    let mut input = input.split("\n\n");

    let drawn_numbers: Vec<u32> = input
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let bingo_cards: Vec<Card> = input
        .map(|s| {
            s.lines()
                .map(|s| {
                    s.split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .map(Cell::new)
                        .collect()
                })
                .collect()
        })
        .map(Card)
        .collect();

    println!("{}", part_1(&drawn_numbers, bingo_cards.to_vec()));
    println!("{}", part_2(&drawn_numbers, bingo_cards));
}

// PART 1
fn part_1(drawn_numbers: &Vec<u32>, mut bingo_cards: Vec<Card>) -> u32 {
    for &number in drawn_numbers {
        for card in bingo_cards.iter_mut() {
            card.mark(number);

            if card.has_won() {
                return card.calculate_score(number);
            }
        }
    }

    panic!("Could not find winning card!")
}

// PART 2
fn part_2(drawn_numbers: &Vec<u32>, mut bingo_cards: Vec<Card>) -> u32 {
    for &number in drawn_numbers {
        let mut i = 0;
        while i < bingo_cards.len() {
            let card = &mut bingo_cards[i];
            card.mark(number);
            if card.has_won() {
                if bingo_cards.len() == 1 {
                    // Last card
                    return bingo_cards[0].calculate_score(number);
                } else {
                    bingo_cards.remove(i);
                }
            } else {
                i += 1;
            }
        }
    }

    panic!("Could not find last card!")
}

// COMMON
struct Cell {
    number: u32,
    marked: bool,
}

impl Cell {
    fn new(number: u32) -> Cell {
        Cell {
            number,
            marked: false,
        }
    }

    fn mark(&mut self) {
        self.marked = true;
    }
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        Cell { ..*self }
    }
}

struct Card(Vec<Vec<Cell>>);

impl Card {
    fn find_cell(&mut self, number: u32) -> Option<&mut Cell> {
        let Card(card) = self;

        for row in card.iter_mut() {
            for cell in row.iter_mut() {
                if cell.number == number {
                    return Some(cell);
                }
            }
        }

        None
    }

    fn mark(&mut self, number: u32) {
        if let Some(cell) = self.find_cell(number) {
            cell.mark();
        }
    }

    fn has_won(&self) -> bool {
        let Card(card) = self;

        // check rows
        let has_won_row = card.iter().any(|row| row.iter().all(|cell| cell.marked));
        if has_won_row {
            return true;
        }

        // check columns
        for i_col in 0..card[0].len() {
            let mut marked = 0;
            for i_row in 0..card.len() {
                if card[i_row][i_col].marked {
                    marked += 1;
                } else {
                    break;
                }
            }
            if marked == card.len() {
                return true;
            }
        }

        false
    }

    fn calculate_score(&self, winning_num: u32) -> u32 {
        let Card(card) = self;

        let sum_unmarked: u32 = card
            .iter()
            .flatten()
            .filter(|cell| !cell.marked)
            .map(|cell| cell.number)
            .sum();

        sum_unmarked * winning_num
    }
}

impl Clone for Card {
    fn clone(&self) -> Self {
        let Card(vec) = self;
        Card(vec.to_vec())
    }
}
