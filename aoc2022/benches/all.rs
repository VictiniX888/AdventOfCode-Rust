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

criterion_group!(
    all,
    bench_day01,
    bench_day02,
    bench_day03,
    bench_day04,
    bench_day05
);

criterion_main!(all);
