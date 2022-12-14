use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn bench_day01(c: &mut Criterion) {
    let input = black_box(aoc2022::read_input(1));
    c.bench_function("Day 01", |b| {
        b.iter(|| (aoc2022::day01::SOLUTION.solve)(&input))
    });
}

pub fn bench_day02(c: &mut Criterion) {
    let input = black_box(aoc2022::read_input(2));
    c.bench_function("Day 02", |b| {
        b.iter(|| (aoc2022::day02::SOLUTION.solve)(&input))
    });
}

pub fn bench_day03(c: &mut Criterion) {
    let input = black_box(aoc2022::read_input(3));
    c.bench_function("Day 03", |b| {
        b.iter(|| (aoc2022::day03::SOLUTION.solve)(&input))
    });
}

pub fn bench_day04(c: &mut Criterion) {
    let input = black_box(aoc2022::read_input(4));
    c.bench_function("Day 04", |b| {
        b.iter(|| (aoc2022::day04::SOLUTION.solve)(&input))
    });
}

pub fn bench_day05(c: &mut Criterion) {
    let input = black_box(aoc2022::read_input(5));
    c.bench_function("Day 05", |b| {
        b.iter(|| (aoc2022::day05::SOLUTION.solve)(&input))
    });
}

pub fn bench_day06(c: &mut Criterion) {
    let input = black_box(aoc2022::read_input(6));
    c.bench_function("Day 06", |b| {
        b.iter(|| (aoc2022::day06::SOLUTION.solve)(&input))
    });
}

pub fn bench_day07(c: &mut Criterion) {
    let input = black_box(aoc2022::read_input(7));
    c.bench_function("Day 07", |b| {
        b.iter(|| (aoc2022::day07::SOLUTION.solve)(&input))
    });
}

pub fn bench_day08(c: &mut Criterion) {
    let input = black_box(aoc2022::read_input(8));
    c.bench_function("Day 08", |b| {
        b.iter(|| (aoc2022::day08::SOLUTION.solve)(&input))
    });
}

pub fn bench_day09(c: &mut Criterion) {
    let input = black_box(aoc2022::read_input(9));
    c.bench_function("Day 09", |b| {
        b.iter(|| (aoc2022::day09::SOLUTION.solve)(&input))
    });
}

pub fn bench_day10(c: &mut Criterion) {
    let input = black_box(aoc2022::read_input(10));
    c.bench_function("Day 10", |b| {
        b.iter(|| (aoc2022::day10::SOLUTION.solve)(&input))
    });
}

pub fn bench_day11(c: &mut Criterion) {
    let input = black_box(aoc2022::read_input(11));
    c.bench_function("Day 11", |b| {
        b.iter(|| (aoc2022::day11::SOLUTION.solve)(&input))
    });
}

pub fn bench_day12(c: &mut Criterion) {
    let input = black_box(aoc2022::read_input(12));
    c.bench_function("Day 12", |b| {
        b.iter(|| (aoc2022::day12::SOLUTION.solve)(&input))
    });
}

pub fn bench_day13(c: &mut Criterion) {
    let input = black_box(aoc2022::read_input(13));
    c.bench_function("Day 13", |b| {
        b.iter(|| (aoc2022::day13::SOLUTION.solve)(&input))
    });
}

criterion_group!(
    all,
    bench_day01,
    bench_day02,
    bench_day03,
    bench_day04,
    bench_day05,
    bench_day06,
    bench_day07,
    bench_day08,
    bench_day09,
    bench_day10,
    bench_day11,
    bench_day12,
    bench_day13,
);

criterion_main!(all);
