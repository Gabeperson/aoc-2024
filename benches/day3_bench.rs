use aoc_2024::day3::*;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn benches(c: &mut Criterion) {
    let mut part1_group = c.benchmark_group("day3_part1");
    part1_group.bench_function("day3_part1_naive", |b| b.iter(|| part1(black_box(REGULAR))));
    part1_group.bench_function("day3_part1_optimized", |b| {
        b.iter(|| part1_opt(black_box(REGULAR)))
    });
    part1_group.finish();
    let mut part2_group = c.benchmark_group("day3_part2");
    part2_group.bench_function("day3_part2_optimized", |b| {
        b.iter(|| part2(black_box(REGULAR)))
    });
    part2_group.finish();
    let mut all = c.benchmark_group("day3_all");
    all.bench_function("day3_all", |b| b.iter(|| run(REGULAR)));
    all.finish();
}

criterion_group!(day2_bench, benches);
criterion_main!(day2_bench);
