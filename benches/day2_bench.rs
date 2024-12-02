use aoc_2024::day2::*;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn benches(c: &mut Criterion) {
    let mut parsing = c.benchmark_group("day2_parsing");
    parsing.bench_function("parse", |b| {
        b.iter(|| parse(black_box(REGULAR)));
    });
    parsing.bench_function("parse_optimized", |b| {
        b.iter(|| parse_optimized(black_box(REGULAR)));
    });

    parsing.finish();
    let mut part1 = c.benchmark_group("day2_part1");
    part1.bench_function("day2_part1_naive", |b| {
        b.iter(|| part1_naive(black_box(REGULAR)))
    });
    part1.bench_function("day2_part1_simd", |b| {
        b.iter(|| part1_simd(black_box(REGULAR)))
    });
    part1.bench_function("day2_part1_optimized", |b| {
        b.iter(|| part1_optimized(black_box(REGULAR)))
    });
    part1.finish();
    let mut part2 = c.benchmark_group("day2_part2");
    part2.bench_function("day2_part2_naive", |b| {
        b.iter(|| part2_naive(black_box(REGULAR)))
    });
    part2.bench_function("day2_part2_simd", |b| {
        b.iter(|| part2_simd(black_box(REGULAR)))
    });
    part2.bench_function("day2_part2_optimized", |b| {
        b.iter(|| part2_optimized(black_box(REGULAR)))
    });
    part2.finish();
}

criterion_group!(day2_bench, benches);
criterion_main!(day2_bench);
