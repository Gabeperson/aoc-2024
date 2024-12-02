use aoc_2024::day1::*;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn benches(c: &mut Criterion) {
    bench_parsing(c);
    bench_part1(c);
    bench_part2(c);
    bench_all(c);
}

fn bench_all(c: &mut Criterion) {
    let mut all = c.benchmark_group("day1_all");
    all.bench_function("day1_all", |b| {
        b.iter(|| {
            let (mut left, mut right, arr) = preparse_simd(REGULAR);
            part1_preparsed(black_box(&mut left), black_box(&mut right));
            part2_preparsed(black_box(left), black_box(arr));
        });
    });
    all.finish();
}

#[allow(unused)]
fn bench_parsing(c: &mut Criterion) {
    let mut parse_group = c.benchmark_group("day1_parse_only");
    // parse_group.bench_function("day1_parse_optimized", |b| {
    //     b.iter(|| {
    //         parse_optimized(black_box(REGULAR));
    //     })
    // });
    parse_group.bench_function("day1_parse_5len", |b| {
        b.iter(|| {
            parse_optimized_5len(black_box(REGULAR));
        })
    });
    // parse_group.bench_function("day1_parse_regex", |b| {
    //     b.iter(|| {
    //         parse(black_box(REGULAR));
    //     })
    // });
    parse_group.bench_function("preparse_simd", |b| {
        b.iter(|| {
            preparse_simd(black_box(REGULAR));
        })
    });
    parse_group.finish();
}

#[allow(unused)]
fn bench_part2(c: &mut Criterion) {
    let mut part2_group = c.benchmark_group("day1_part2");
    // part2_group.bench_function("day1_part2_regex", |b| b.iter(|| part2(black_box(REGULAR))));
    // part2_group.bench_function("day1_part2_naive_linear_search", |b| {
    //     b.iter(|| part2_simd_naive_linear_search(REGULAR));
    // });
    // part2_group.bench_function("day1_part2_manual_identity_hashmap", |b| {
    //     b.iter(|| {
    //         let mut map = HashMap::with_hasher(IdentityHasherGenerator);
    //         map.reserve(1000);
    //         part2_optimized(black_box(REGULAR), black_box(map));
    //     })
    // });

    // part2_group.bench_function("day1_part2_manual_fxhash_hashmap", |b| {
    //     b.iter(|| {
    //         let mut map = HashMap::with_hasher(FxBuildHasher::default());
    //         map.reserve(1000);
    //         part2_optimized(black_box(REGULAR), black_box(map));
    //     })
    // });
    // part2_group.bench_function("day1_part2_manual_regular_hashmap", |b| {
    //     b.iter(|| {
    //         let mut map = HashMap::new();
    //         map.reserve(1000);
    //         part2_optimized(black_box(REGULAR), black_box(map));
    //     })
    // });
    // part2_group.bench_function("day1_part2_manual_btreemap", |b| {
    //     b.iter(|| {
    //         part2_optimized_btree(black_box(REGULAR));
    //     })
    // });
    // part2_group.bench_function("day1_part2_manual_fxhash_2", |b| {
    //     b.iter(|| {
    //         part2_optimized_manual_2(black_box(REGULAR));
    //     })
    // });
    // part2_group.bench_function("day1_part2_manual_vec", |b| {
    //     b.iter(|| {
    //         part2_optimized_manual_2_vec(black_box(REGULAR));
    //     })
    // });
    // part2_group.bench_function("day1_part2_manual_arr", |b| {
    //     b.iter(|| {
    //         part2_optimized_manual_2_arr(black_box(REGULAR));
    //     })
    // });
    part2_group.bench_function("day1_part2_manual_arr_simd", |b| {
        b.iter(|| {
            part2_optimized_manual_2_arr_simd(black_box(REGULAR));
        })
    });
    // part2_group.bench_function("day1_part2_end", |b| {
    //     b.iter_batched(
    //         || {
    //             let (mut v1, mut v2, map) = parse_optimized_5len(black_box(REGULAR));
    //             part1_optimized_withvec(&mut v1, &mut v2);
    //             (v1, map)
    //         },
    //         |(v1, map)| part2_optimzied_withvec_preparsed(&v1[..], map),
    //         criterion::BatchSize::SmallInput,
    //     );
    // });
    part2_group.finish();
}

#[allow(unused)]
fn bench_part1(c: &mut Criterion) {
    let mut part1_group = c.benchmark_group("day1_part1");
    // part1_group.bench_function("day1_part1_regex", |b| b.iter(|| part1(black_box(REGULAR))));
    // part1_group.bench_function("day1_part1_manual", |b| {
    //     b.iter(|| part1_optimized(black_box(REGULAR)))
    // });
    // part1_group.bench_function("day1_part1_end", |b| {
    //     b.iter_batched(
    //         || parse_optimized_5len(black_box(REGULAR)),
    //         |(mut v1, mut v2, _)| part1_optimized_withvec(&mut v1[..], &mut v2[..]),
    //         criterion::BatchSize::SmallInput,
    //     );
    // });
    // part1_group.bench_function("day1_part1_optimized2", |b| {
    //     b.iter(|| {
    //         part1_optimized_manual_2(black_box(REGULAR));
    //     })
    // });
    part1_group.bench_function("day1_part1_optimized2_simd", |b| {
        b.iter(|| {
            part1_optimized_manual_2_simd(black_box(REGULAR));
        })
    });
    part1_group.bench_function("day1_part1_preparsed_array", |b| {
        b.iter_batched(
            || {
                let (left, right, arr) = preparse_simd(REGULAR);
                (black_box(left), black_box(right))
            },
            |(mut left, mut right)| {
                part1_preparsed(black_box(&mut left), black_box(&mut right));
            },
            criterion::BatchSize::SmallInput,
        );
    });
    part1_group.finish();
}

criterion_group!(day1_bench, benches);
criterion_main!(day1_bench);
