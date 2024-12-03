use itertools::Itertools;

pub static SIMPLE: &str = include_str!("../inputs/day2_simple.txt");
pub static REGULAR: &str = include_str!("../inputs/day2.txt");
pub fn parse(s: &str) -> Vec<Vec<u8>> {
    let res = s
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<u8>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    res
}

pub fn run(s: &str) -> (i32, i32) {
    let v = parse_optimized(s);
    (part1_preparsed(&v), part2_preparsed(&v))
}

pub fn parse_byte_integers_v2(input: &[u8], output: &mut [u8; 8]) -> (usize, usize) {
    let mut i = 0;
    let mut index = 0;

    loop {
        match input[i] {
            b'\n' => break,
            b' ' => {}
            n @ b'0'..=b'9' => {
                if i + 1 < input.len() && input[i + 1].is_ascii_digit() {
                    let tens = (n - 48) * 10;
                    let ones = input[i + 1];
                    output[index] = tens + ones;

                    i += 1;
                } else {
                    output[index] = n;
                }
                index += 1;
            }
            _ => unreachable!(),
        }

        i += 1;
    }
    (index, i + 1)
}

pub fn parse_optimized(s: &str) -> [([u8; 8], usize); 1000] {
    let mut res = [([0u8; 8], 0usize); 1000];
    let mut bytes = s.as_bytes();
    let mut buf = [0u8; 8];
    (0..1000).for_each(|i| {
        let (len, processed_count) = parse_byte_integers_v2(bytes, &mut buf);
        bytes = &bytes[processed_count..];
        res[i] = (buf, len);
    });
    res
}

pub fn part1_naive(s: &str) -> i32 {
    let v = parse(s);

    v.into_iter()
        .fold(0, |acc, v| if safe(&v).is_safe() { acc + 1 } else { acc })
}

pub fn part1_optimized(s: &str) -> i32 {
    let v = parse_optimized(s);

    part1_preparsed(&v)
}

pub fn part1_preparsed(v: &[([u8; 8], usize); 1000]) -> i32 {
    v.iter()
        .filter(|(arr, len)| safe(&arr[..*len]).is_safe())
        .count() as i32
}

pub fn part1_simd(s: &str) -> i32 {
    let v = parse(s);
    v.into_iter().fold(0, |acc, v| {
        if get_safety(&v).is_safe() {
            acc + 1
        } else {
            acc
        }
    })
}

fn safe(l: &[u8]) -> Safety {
    let ascending = l[1] > l[0];
    for (i, (a, b)) in l.iter().tuple_windows().enumerate() {
        if ascending && b <= a || !ascending && a <= b || !(1..=3).contains(&a.abs_diff(*b)) {
            return Safety::NotSafe(i);
        }
    }
    Safety::Safe
}

pub fn part2_naive(s: &str) -> i32 {
    let v = parse(s);

    v.into_iter().fold(0, |acc, v| {
        if safe(&v).is_safe() {
            return acc + 1;
        }
        if (0..v.len())
            .map(|i| {
                let mut v = v.clone();
                v.remove(i);
                v
            })
            .any(|v| safe(&v).is_safe())
        {
            acc + 1
        } else {
            acc
        }
    })
}

pub fn part2_optimized(s: &str) -> i32 {
    let v = parse_optimized(s);
    part2_preparsed(&v)
}

pub fn part2_preparsed(v: &[([u8; 8], usize); 1000]) -> i32 {
    v.iter()
        .filter(|(arr, len)| {
            let index = match safe(&arr[..*len]) {
                Safety::Safe => {
                    return true;
                }
                Safety::NotSafe(i) => i,
            };
            let mut arr1 = *arr;
            let mut arr2 = *arr;
            let mut arr3 = *arr;
            for i in index..7 {
                arr1[i] = arr1[i + 1];
            }
            for i in (index + 1)..7 {
                arr2[i] = arr2[i + 1];
            }
            if index != 0 {
                for i in (index - 1)..7 {
                    arr3[i] = arr3[i + 1];
                }
            } else {
                arr3 = arr1;
            }
            safe(&arr1[..(len - 1)]).is_safe()
                || safe(&arr2[..(len - 1)]).is_safe()
                || safe(&arr3[..(len - 1)]).is_safe()
        })
        .count() as i32
}

pub fn part2_simd(s: &str) -> i32 {
    let v = parse(s);

    v.into_iter().fold(0, |acc, v| {
        if get_safety(&v).is_safe() {
            return acc + 1;
        }
        if (0..v.len())
            .map(|i| {
                let mut v = v.clone();
                v.remove(i);
                v
            })
            .any(|v| get_safety(&v).is_safe())
        {
            acc + 1
        } else {
            acc
        }
    })
}

fn get_safety(slice: &[u8]) -> Safety {
    // use std::simd::{cmp::SimdPartialEq, simd_swizzle, u8x8, Mask, Simd, Swizzle};
    use std::simd::prelude::*;
    let len = slice.len();
    let lane = u8x8::load_or_default(slice);
    let moved = simd_swizzle!(lane, [1, 2, 3, 4, 5, 6, 7, 7]);
    let diffs = moved - lane;
    let zero = i8x8::splat(0);
    let two = u8x8::splat(2);
    let mask = Mask::<i8, 8>::from_array([
        len > 1,
        len > 2,
        len > 3,
        len > 4,
        len > 5,
        len > 6,
        len > 7,
        len > 8,
    ]);
    let diffs_masked: i8x8 = u8x8::load_select(diffs.as_array(), mask, two).cast();
    let eq_mask = diffs_masked.simd_eq(zero);
    let diffs_masked: i8x8 = u8x8::load_select_or_default(diffs.as_array(), mask).cast();
    // two elements are equal
    if let Some(s) = eq_mask.first_set() {
        return Safety::NotSafe(s);
    };

    let reduced_sum = diffs_masked.signum().reduce_sum();
    if reduced_sum.unsigned_abs() as usize != len - 1 {
        return Safety::NotSafe(
            if reduced_sum.is_positive() {
                diffs_masked.is_negative().first_set()
            } else {
                diffs_masked.is_positive().first_set()
            }
            .unwrap(),
        );
    }

    let three = i8x8::splat(3);
    let abs = diffs_masked.abs();
    let cmp = abs.simd_gt(three);
    if let Some(s) = cmp.first_set() {
        return Safety::NotSafe(s);
    }

    Safety::Safe
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Safety {
    Safe,
    NotSafe(usize),
}

impl Safety {
    fn is_safe(&self) -> bool {
        matches!(self, Safety::Safe)
    }
}

pub fn parse_byte_integers(input: &[u8], output: &mut [u8; 8]) -> (usize, usize) {
    let mut parsed_count = 0;
    let mut current_number = 0u8;
    let mut is_second_digit = false;
    let mut bytes_processed = 0;

    for &byte in input.iter() {
        bytes_processed += 1;

        if byte == b'\n' {
            break;
        }

        if byte == b' ' {
            output[parsed_count] = current_number;
            parsed_count += 1;
            current_number = 0;
            is_second_digit = false;
        } else {
            let digit = byte - 48;
            current_number = current_number * (is_second_digit as u8 * 9 + 1) + digit;
            is_second_digit ^= true;
        }
    }

    output[parsed_count] = current_number;
    parsed_count += (current_number > 0) as usize;

    (parsed_count, bytes_processed)
}
