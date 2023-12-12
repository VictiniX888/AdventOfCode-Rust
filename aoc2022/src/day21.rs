use std::{collections::HashMap, str::FromStr};

use crate::*;

pub const SOLUTION: Solution = Solution { day: 21, solve };

fn solve(input: &str) -> AnswerSet {
    let mut cache: HashMap<&str, u64> = HashMap::new();

    let exprs = parse_input(input);
    let rev_exprs = rev_exprs(&exprs);

    // Part 1
    let p1 = eval("root", &exprs, &mut cache);

    // Part 2
    // Work upwards from humn
    let mut curr = "humn";
    let mut partial_ops = vec![];
    loop {
        let parent = rev_exprs.get(curr);
        if parent.is_none() {
            break;
        }

        let &parent = parent.unwrap();
        let op = eval_partial(parent, curr, &exprs, &mut cache);
        partial_ops.push(op);

        curr = parent;
    }

    // Work downwards from root, inverting operations
    let mut p2 = 0;
    if let Some((last, partial_ops)) = partial_ops.split_last() {
        p2 = last.other;

        for op in partial_ops.iter().rev() {
            p2 = match (op.operator, op.side) {
                (Operator::Add, _) => p2 - op.other,
                (Operator::Sub, OperandSide::First) => op.other - p2,
                (Operator::Sub, OperandSide::Second) => p2 + op.other,
                (Operator::Mul, _) => p2 / op.other,
                (Operator::Div, OperandSide::First) => op.other / p2,
                (Operator::Div, OperandSide::Second) => p2 * op.other,
            };
        }
    }

    AnswerSet {
        p1: Answer::U64(p1),
        p2: Answer::U64(p2),
    }
}

fn rev_exprs<'a>(exprs: &'a HashMap<&str, Expression>) -> HashMap<&'a str, &'a str> {
    exprs
        .into_iter()
        .flat_map(|(&root, expr)| {
            if let Expression::Operation(op) = expr {
                vec![(op.first.as_str(), root), (op.second.as_str(), root)]
            } else {
                vec![]
            }
        })
        .collect()
}

fn eval_partial<'a>(
    root: &'a str,
    exclude: &'a str,
    exprs: &'a HashMap<&str, Expression>,
    cache: &mut HashMap<&'a str, u64>,
) -> PartialOperation {
    // Evaluate the other branch (not exclude)
    if let Expression::Operation(root) = &exprs[root] {
        let (other, side) = if root.first == exclude {
            (root.second.as_str(), OperandSide::Second)
        } else {
            (root.first.as_str(), OperandSide::First)
        };
        let other = eval(other, &exprs, cache);

        PartialOperation {
            operator: root.operator,
            other,
            side,
        }
    } else {
        // Unreachable; this must be an Operation expression
        panic!()
    }
}

fn eval<'a>(
    root: &'a str,
    exprs: &'a HashMap<&str, Expression>,
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    if let Some(&val) = cache.get(root) {
        val
    } else {
        let val = match &exprs[root] {
            Expression::Number(val) => *val,
            Expression::Operation(op) => op.operator.apply(
                eval(&op.first, exprs, cache),
                eval(&op.second, exprs, cache),
            ),
        };

        cache.insert(root, val);
        val
    }
}

fn parse_input(input: &str) -> HashMap<&str, Expression> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> (&str, Expression) {
    let (name, expr_str) = line.split_once(": ").unwrap();

    let mut expr_iter = expr_str.split_whitespace();
    let first = expr_iter.next().unwrap().to_string();

    if let Some(op) = expr_iter.next() {
        let second = expr_iter.next().unwrap().to_string();
        let expr = Operation {
            operator: op.parse().unwrap(),
            first,
            second,
        };

        (name, Expression::Operation(expr))
    } else {
        (name, Expression::Number(first.parse().unwrap()))
    }
}

#[derive(Debug)]
struct PartialOperation {
    operator: Operator,
    other: u64,
    side: OperandSide,
}

#[derive(Clone, Copy, Debug)]
enum OperandSide {
    First,
    Second,
}

enum Expression {
    Operation(Operation),
    Number(u64),
}

struct Operation {
    operator: Operator,
    first: String,
    second: String,
}

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    fn apply(self, first: u64, second: u64) -> u64 {
        match self {
            Operator::Add => first + second,
            Operator::Sub => first - second,
            Operator::Mul => first * second,
            Operator::Div => first / second,
        }
    }

    fn apply_invert(self, first: u64, second: u64) -> u64 {
        self.invert().apply(second, first)
    }

    fn invert(self) -> Self {
        match self {
            Operator::Add => Operator::Sub,
            Operator::Sub => Operator::Add,
            Operator::Mul => Operator::Div,
            Operator::Div => Operator::Mul,
        }
    }
}

#[derive(Debug)]
struct ParseOperatorError;

impl FromStr for Operator {
    type Err = ParseOperatorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Sub),
            "*" => Ok(Operator::Mul),
            "/" => Ok(Operator::Div),
            _ => Err(ParseOperatorError),
        }
    }
}
