use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn bench_day01(c: &mut Criterion) {
    let input = black_box(aoc2022::read_input(1));
    c.bench_function("Day 01", |b| {
        b.iter(|| (aoc2022::day01::SOLUTION.solve)(&input))
    });
}

criterion_group!(all, bench_day01);

criterion_main!(all);
