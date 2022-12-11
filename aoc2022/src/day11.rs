use std::{collections::HashMap, error::Error, num::ParseIntError, str::FromStr};

use crate::*;

pub const SOLUTION: Solution = Solution {
    day: 11,
    solve: solve_optimized,
};

/* ======== OPTIMIZED (CYCLES) ======== */
// ~2.5 ms
fn solve_optimized(input: &str) -> AnswerSet {
    let monkeys = input
        .split("\n\n")
        .map(|s| s.parse())
        .collect::<Result<Vec<Monkey>, _>>()
        .unwrap();
    let modulus = monkeys
        .iter()
        .map(|monkey| monkey.throw_dest.divisor)
        .product::<u64>();

    let p1 = simulate(monkeys.clone(), 20, modulus, true);

    let mut inspections = vec![0; monkeys.len()];
    let mut count = vec![0; monkeys.len()];

    // Iterate through each item instead of each monkey, since items behave independently of each other
    for (loc, &item) in monkeys
        .iter()
        .enumerate()
        .flat_map(|(loc, monkey)| monkey.items.iter().map(move |item| (loc, item)))
    {
        let mut item = item;
        let mut loc = loc;
        let mut rounds = 0;
        let mut cache = HashMap::new();
        let mut thrown;

        // Loop while item state (value and monkey) does not repeat
        while cache.get(&(item, loc)).is_none() {
            cache.insert((item, loc), rounds);
            thrown = monkeys[loc].process_item(item, modulus, false);
            count[loc] += 1;
            while thrown.dest >= loc {
                item = thrown.item;
                loc = thrown.dest;
                thrown = monkeys[loc].process_item(item, modulus, false);
                count[loc] += 1;
            }
            item = thrown.item;
            loc = thrown.dest;

            rounds += 1;
        }

        for (total, count) in inspections.iter_mut().zip(count.iter()) {
            *total += count;
        }

        // Cycle found
        let cycle_len = rounds - cache.get(&(item, loc)).unwrap();
        let cycles = (10_000 - rounds) / cycle_len;
        let rem_rounds = (10_000 - rounds) % cycle_len;
        let mut rem = Vec::new();

        count.fill(0);
        // Replay cycle (this was faster than keeping track of count in the map at every stage)
        for i in 0..cycle_len {
            if i == rem_rounds {
                rem = count.clone();
            }

            thrown = monkeys[loc].process_item(item, modulus, false);
            count[loc] += 1;
            while thrown.dest >= loc {
                item = thrown.item;
                loc = thrown.dest;
                thrown = monkeys[loc].process_item(item, modulus, false);
                count[loc] += 1;
            }
            item = thrown.item;
            loc = thrown.dest;
        }

        for ((total, count), rem) in inspections.iter_mut().zip(count.iter()).zip(rem) {
            *total += count * cycles + rem;
        }

        count.fill(0);
    }

    inspections.sort();
    let p2 = inspections[inspections.len() - 1] * inspections[inspections.len() - 2];

    AnswerSet {
        p1: Answer::Usize(p1),
        p2: Answer::Usize(p2),
    }
}

/* ======== INITIAL SOLUTION ======== */
// ~25 ms
fn solve(input: &str) -> AnswerSet {
    let monkeys = input
        .split("\n\n")
        .map(|s| s.parse())
        .collect::<Result<Vec<Monkey>, _>>()
        .unwrap();
    let modulus = monkeys
        .iter()
        .map(|monkey| monkey.throw_dest.divisor)
        .product::<u64>();

    let p1 = simulate(monkeys.clone(), 20, modulus, true);
    let p2 = simulate(monkeys, 10_000, modulus, false);

    AnswerSet {
        p1: Answer::Usize(p1),
        p2: Answer::Usize(p2),
    }
}

fn simulate(mut monkeys: Vec<Monkey>, rounds: usize, modulus: u64, is_p1: bool) -> usize {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let thrown = monkeys[i].process_all(modulus, is_p1);
            for thrown in thrown {
                monkeys[thrown.dest].receive(thrown.item);
            }
        }
    }

    monkeys.sort_by_key(|monkey| monkey.inspected);
    monkeys[monkeys.len() - 1].inspected * monkeys[monkeys.len() - 2].inspected
}

/* ======== STRUCTS ======== */
#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    throw_dest: DivTest,
    inspected: usize,
}

impl Monkey {
    fn receive(&mut self, item: u64) {
        self.items.push(item);
    }

    fn process_all(&mut self, modulus: u64, is_p1: bool) -> Vec<ThrownItem> {
        let thrown = self
            .items
            .iter()
            .map(|&item| self.process_item(item, modulus, is_p1))
            .collect();
        self.inspected += self.items.len();
        self.items.clear();

        thrown
    }

    fn process_item(&self, item: u64, modulus: u64, is_p1: bool) -> ThrownItem {
        let item = self.operation.apply(item) % modulus / (is_p1 as u64 * 2 + 1);
        let dest = self.throw_dest.resolve(item);
        ThrownItem { item, dest }
    }
}

struct ThrownItem {
    item: u64,
    dest: usize,
}

#[derive(Clone)]
struct DivTest {
    divisor: u64,
    res_true: usize,
    res_false: usize,
}

impl DivTest {
    fn resolve(&self, num: u64) -> usize {
        if num % self.divisor == 0 {
            self.res_true
        } else {
            self.res_false
        }
    }
}

#[derive(Clone)]
enum Operation {
    Add(Operand),
    Mul(Operand),
}

impl Operation {
    fn apply(&self, other: u64) -> u64 {
        match self {
            Operation::Add(operand) => other + operand.resolve(&other),
            Operation::Mul(operand) => other * operand.resolve(&other),
        }
    }
}

#[derive(Clone)]
enum Operand {
    Num(u64),
    Old,
}

impl<'a> Operand {
    fn resolve(&'a self, old: &'a u64) -> &'a u64 {
        match self {
            Operand::Num(operand) => operand,
            Operand::Old => old,
        }
    }
}

/* ======== PARSING ======== */
impl FromStr for Monkey {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.splitn(4, '\n').skip(1);
        Ok(Monkey {
            items: (&lines.next().unwrap()[18..])
                .split(", ")
                .map(|s| s.parse())
                .collect::<Result<Vec<u64>, _>>()?,
            operation: lines.next().unwrap().parse()?,
            throw_dest: lines.next().unwrap().parse()?,
            inspected: 0,
        })
    }
}

impl FromStr for DivTest {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        Ok(DivTest {
            divisor: (&lines.next().unwrap()[21..]).parse()?,
            res_true: (&lines.next().unwrap()[29..]).parse()?,
            res_false: (&lines.next().unwrap()[30..]).parse()?,
        })
    }
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let operand = (&s[25..]).parse::<Operand>()?;
        let operation = match s.bytes().nth(23).unwrap() {
            b'+' => Operation::Add(operand),
            b'*' => Operation::Mul(operand),
            _ => return Err("Invalid operation".to_string()),
        };

        Ok(operation)
    }
}

impl FromStr for Operand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = s.parse::<u64>() {
            Ok(Operand::Num(num))
        } else {
            // We can be sure that the input will not have any other operands
            Ok(Operand::Old)
        }
    }
}
