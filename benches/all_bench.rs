use aoc_2024::day1;
use aoc_2024::day2;
use aoc_2024::day3;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn benches(c: &mut Criterion) {
    c.bench_function("all_days_all", |b| {
        b.iter(|| {
            day1::run(black_box(day1::REGULAR));
            day2::run(black_box(day2::REGULAR));
            day3::run(black_box(day3::REGULAR));
        })
    });
}

criterion_group!(day2_bench, benches);
criterion_main!(day2_bench);
