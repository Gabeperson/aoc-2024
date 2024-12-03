#![feature(portable_simd)]
use std::hint::black_box;

use aoc_2024::day1::{self};
use aoc_2024::day2;
use aoc_2024::day3;
fn main() {
    // day1_main();
    // day2_main();
    // simd_test();
    day3_main();
}

fn day3_main() {
    // let now = std::time::Instant::now();
    // let res1 = day3::part1(day3::REGULAR);
    // dbg!(now.elapsed());
    // let now = std::time::Instant::now();
    // let res2 = day3::part1_opt(day3::REGULAR);
    // dbg!(now.elapsed());
    // assert_eq!(res1, res2);

    println!("{:?}", day3::run(day3::REGULAR));
}

#[allow(dead_code)]
fn day1_main() {
    let ans1 = day1::part1_optimized_manual_2(day1::REGULAR);
    let ans1_simd = day1::part1_optimized_manual_2_simd(day1::REGULAR);
    let ans2 = day1::part2_optimized_manual_2_arr(day1::REGULAR);
    let ans2_simd = day1::part2_optimized_manual_2_arr_simd(day1::REGULAR);
    println!(
        "Part1: {}; {}, Part2: {}; {}",
        ans1, ans1_simd, ans2, ans2_simd
    );

    let (mut left, mut right, arr) = day1::preparse_simd(day1::REGULAR);
    dbg!(day1::part1_preparsed(
        black_box(&mut left),
        black_box(&mut right)
    ));
    dbg!(day1::part2_preparsed(black_box(left), black_box(arr)));
}
#[allow(dead_code)]
fn day2_main() {
    let ans1 = day2::part1_naive(day2::REGULAR);
    let ans1_opt = day2::part1_optimized(day2::REGULAR);
    let ans2 = day2::part2_naive(day2::REGULAR);
    let ans2_opt = day2::part2_optimized(day2::REGULAR);
    println!(
        "Part1: {}; {}, Part2: {}; {}",
        ans1, ans1_opt, ans2, ans2_opt
    );
}
