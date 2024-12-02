use aoc_2024::day1::{self};
fn main() {
    day1_main();
}
#[allow(dead_code)]
fn day1_main() {
    // assert_eq!(11, day1::part1(day1::SIMPLE));
    // assert_eq!(31, day1::part2(day1::SIMPLE));
    // println!("Day 1, Part 1: {}", day1::part1(day1::REGULAR));
    // println!("Day 1, Part 2: {}", day1::part2(day1::REGULAR));

    // println!("Day 1, Part 1: {}", day1::part1_optimized(day1::REGULAR));
    // let map = HashMap::with_hasher(day1::IdentityHasherGenerator);
    // println!(
    //     "Day 1, Part 2: {}",
    //     day1::part2_optimized(day1::REGULAR, map)
    // );
    // let (mut v1, mut v2, map) = day1::parse_optimized_5len(day1::REGULAR);
    // let ans1 = day1::part1_optimized_withvec(&mut v1, &mut v2);
    // let ans2 = day1::part2_optimzied_withvec_preparsed(&v1, map);
    // println!("Part1: {}, Part2: {}", ans1, ans2);
    let ans1 = day1::part1_optimized_manual_2(day1::REGULAR);
    let ans1_simd = day1::part1_optimized_manual_2_simd(day1::REGULAR);
    let ans2 = day1::part2_optimized_manual_2_arr(day1::REGULAR);
    let ans2_simd = day1::part2_optimized_manual_2_arr_simd(day1::REGULAR);
    println!(
        "Part1: {}; {}, Part2: {}; {}",
        ans1, ans1_simd, ans2, ans2_simd
    );
}
