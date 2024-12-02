use fxhash::FxBuildHasher;
use regex::Regex;
use std::{
    collections::{BTreeMap, HashMap},
    hash::{BuildHasher, Hasher},
};

pub static SIMPLE: &str = include_str!("../inputs/day1_simple.txt");
pub static REGULAR: &str = include_str!("../inputs/day1.txt");

pub fn part1(s: &str) -> i32 {
    let (left, right) = parse(s);
    left.iter().zip(right).map(|(a, b)| (a - b).abs()).sum()
}

pub fn part1_optimized(s: &str) -> i32 {
    let (mut left, mut right) = parse_optimized(s);
    left.sort_unstable();
    right.sort_unstable();
    left.iter().zip(right).map(|(a, b)| (a - b).abs()).sum()
}

pub fn part1_optimized_withvec(v1: &mut [i32], v2: &mut [i32]) -> u32 {
    v1.sort_unstable();
    v2.sort_unstable();
    v1.iter().zip(v2).map(|(a, b)| a.abs_diff(*b)).sum()
}

pub fn part1_optimized_manual_2(s: &str) -> i32 {
    let mut left = [0; 1000];
    let mut right = [0; 1000];
    let mut count = 0;
    s.as_bytes().chunks(14).for_each(|line| {
        let (l, r) = parse_line_5l(line);
        left[count] = l;
        right[count] = r;
        count += 1;
    });
    left.sort_unstable();
    right.sort_unstable();
    left.iter()
        .zip(right)
        .map(|(a, b)| a.abs_diff(b))
        .sum::<u32>() as i32
}

pub fn part1_optimized_manual_2_simd(s: &str) -> i32 {
    let mut left = [0; 1000];
    let mut right = [0; 1000];
    let mut count = 0;
    s.as_bytes().chunks(14).for_each(|line| {
        let (l, r) = parse_line_5l_simd(line);
        left[count] = l;
        right[count] = r;
        count += 1;
    });
    left.sort_unstable();
    right.sort_unstable();
    left.iter()
        .zip(right)
        .map(|(a, b)| a.abs_diff(b))
        .sum::<u32>() as i32
}

pub fn preparse_simd(s: &str) -> ([i32; 1000], [i32; 1000], [i32; 90000]) {
    fn parse_line_5l_simd(c: &[u8]) -> (i32, i32) {
        use std::simd::{
            num::{SimdInt, SimdUint},
            Simd,
        };
        const DIGIT_WEIGHTS_A: Simd<i32, 8> = Simd::from_array([10000, 1000, 100, 10, 1, 0, 0, 0]);
        const DIGIT_WEIGHTS_B: Simd<i32, 8> = Simd::from_array([0, 0, 0, 10000, 1000, 100, 10, 1]);
        let a_arr = Simd::<u8, 8>::from_slice(c).cast();
        let b_arr = Simd::<u8, 8>::from_slice(&c[5..13]).cast();
        let a_digits = a_arr * DIGIT_WEIGHTS_A;
        let b_digits = b_arr * DIGIT_WEIGHTS_B;
        (
            a_digits.reduce_sum() - 533328,
            b_digits.reduce_sum() - 533328,
        )
    }
    let mut left = [0; 1000];
    let mut right = [0; 1000];
    let mut arr = [0; 90000];
    let mut count = 0;
    s.as_bytes().chunks(14).for_each(|line| {
        let (l, r) = parse_line_5l_simd(line);
        left[count] = l;
        right[count] = r;
        arr[r as usize - 10000] += 1;
        count += 1;
    });
    (left, right, arr)
}

pub fn part1_preparsed(v1: &mut [i32], v2: &mut [i32]) -> i32 {
    v1.sort_unstable();
    v2.sort_unstable();
    v1.iter().zip(v2).map(|(a, b)| a.abs_diff(*b)).sum::<u32>() as i32
}

pub fn part2_preparsed(v: [i32; 1000], arr: [i32; 90000]) -> i32 {
    v.iter().map(|a| a * arr[*a as usize - 10000]).sum()
}

pub fn part2_optimzied_withvec_preparsed(v1: &[i32], map: HashMap<i32, i32, FxBuildHasher>) -> i32 {
    v1.iter().map(|a| a * map.get(a).unwrap_or(&0)).sum()
}

pub fn part2_optimized<T: BuildHasher>(s: &str, map: HashMap<i32, i32, T>) -> i32 {
    let (mut left, right) = parse_optimized(s);
    left.sort_unstable();
    let right = right.into_iter().fold(map, |mut acc, item| {
        *acc.entry(item).or_insert(0) += 1;
        acc
    });
    left.into_iter()
        .map(|a| a * right.get(&a).unwrap_or(&0))
        .sum()
}

pub fn part2_optimized_btree(s: &str) -> i32 {
    let map = BTreeMap::new();
    let (mut left, right) = parse_optimized(s);
    left.sort_unstable();
    let right = right.into_iter().fold(map, |mut acc, item| {
        *acc.entry(item).or_insert(0) += 1;
        acc
    });
    left.into_iter()
        .map(|a| a * right.get(&a).unwrap_or(&0))
        .sum()
}

pub fn part2(s: &str) -> i32 {
    let (left, right) = parse(s);
    let right = right.into_iter().fold(HashMap::new(), |mut acc, item| {
        *acc.entry(item).or_insert(0) += 1;
        acc
    });
    left.into_iter()
        .map(|a| a * right.get(&a).unwrap_or(&0))
        .sum()
}

pub fn part2_optimized_manual_2(s: &str) -> i32 {
    let mut left = Vec::with_capacity(1000);
    let mut map = HashMap::with_hasher(FxBuildHasher::default());
    s.as_bytes().chunks(14).for_each(|line| {
        let (l, r) = parse_line_5l(line);
        left.push(l);
        *map.entry(r).or_insert(0) += 1;
    });
    left.iter().map(|a| a * map.get(a).unwrap_or(&0)).sum()
}

pub fn part2_optimized_manual_2_vec(s: &str) -> i32 {
    let mut left = Vec::with_capacity(1000);
    let mut right = [0; 90000];
    s.as_bytes().chunks(14).for_each(|line| {
        let (l, r) = parse_line_5l(line);
        left.push(l);
        right[r as usize - 10000] += 1;
    });
    left.iter().map(|a| a * right[*a as usize - 10000]).sum()
}

pub fn part2_optimized_manual_2_arr(s: &str) -> i32 {
    let mut left = [0; 1000];
    let mut counter = 0;
    let mut right = [0; 90000];
    s.as_bytes().chunks(14).for_each(|line| {
        let (l, r) = parse_line_5l(line);
        left[counter] = l;
        right[r as usize - 10000] += 1;
        counter += 1;
    });
    left.iter().map(|a| a * right[*a as usize - 10000]).sum()
}

pub fn part2_optimized_manual_2_arr_simd(s: &str) -> i32 {
    let mut left = [0; 1000];
    let mut counter = 0;
    let mut right = [0; 90000];
    s.as_bytes().chunks(14).for_each(|line| {
        let (l, r) = parse_line_5l_simd(line);
        left[counter] = l;
        right[r as usize - 10000] += 1;
        counter += 1;
    });
    left.iter().map(|a| a * right[*a as usize - 10000]).sum()
}

pub fn part2_simd_naive_linear_search(s: &str) -> i32 {
    let mut left = [0; 1000];
    let mut right = [0; 1000];
    let mut count = 0;
    s.as_bytes().chunks(14).for_each(|line| {
        let (l, r) = parse_line_5l_simd(line);
        left[count] = l;
        right[count] = r;
        count += 1;
    });
    left.iter()
        .map(|a| a * right.iter().filter(|&n| n == a).count() as i32)
        .sum()
}

fn parse_line_5l_simd(c: &[u8]) -> (i32, i32) {
    use std::simd::{
        num::{SimdInt, SimdUint},
        Simd,
    };
    const DIGIT_WEIGHTS_A: Simd<i32, 8> = Simd::from_array([10000, 1000, 100, 10, 1, 0, 0, 0]);
    const DIGIT_WEIGHTS_B: Simd<i32, 8> = Simd::from_array([0, 0, 0, 10000, 1000, 100, 10, 1]);
    let a_arr = Simd::<u8, 8>::from_slice(c).cast();
    let b_arr = Simd::<u8, 8>::from_slice(&c[5..13]).cast();
    let a_digits = a_arr * DIGIT_WEIGHTS_A;
    let b_digits = b_arr * DIGIT_WEIGHTS_B;
    (
        a_digits.reduce_sum() - 533328,
        b_digits.reduce_sum() - 533328,
    )
}

pub fn parse_optimized(s: &str) -> (Vec<i32>, Vec<i32>) {
    let mut v1 = Vec::with_capacity(1000);
    let mut v2 = Vec::with_capacity(1000);
    let mut s = s.trim();
    loop {
        let (a, pos1) = parse_int(s);
        s = &s[(pos1 + 3)..];
        let (b, pos2) = parse_int(s);
        s = &s[pos2..];
        v1.push(a);
        v2.push(b);
        if s.is_empty() {
            break;
        }
        s = &s[1..];
    }

    (v1, v2)
}

fn parse_line_5l(c: &[u8]) -> (i32, i32) {
    let a = c[0] as i32 * 10000
        + c[1] as i32 * 1000
        + c[2] as i32 * 100
        + c[3] as i32 * 10
        + c[4] as i32
        - 533328;
    let b = c[8] as i32 * 10000
        + c[9] as i32 * 1000
        + c[10] as i32 * 100
        + c[11] as i32 * 10
        + c[12] as i32
        - 533328;
    (a, b)
}

pub fn parse_optimized_5len(
    s: &str,
) -> (Vec<i32>, Vec<i32>, HashMap<i32, i32, fxhash::FxBuildHasher>) {
    let mut map = HashMap::with_hasher(FxBuildHasher::default());
    map.reserve(1000);
    let (v1, v2): (Vec<i32>, Vec<i32>) = s
        .as_bytes()
        .chunks(14)
        .map(|c| {
            let a = c[0] as i32 * 10000
                + c[1] as i32 * 1000
                + c[2] as i32 * 100
                + c[3] as i32 * 10
                + c[4] as i32
                - 533328;
            let b = c[8] as i32 * 10000
                + c[9] as i32 * 1000
                + c[10] as i32 * 100
                + c[11] as i32 * 10
                + c[12] as i32
                - 533328;
            *map.entry(b).or_insert(0) += 1;
            (a, b)
        })
        .unzip();
    (v1, v2, map)
}

fn parse_int(s: &str) -> (i32, usize) {
    let mut value = 0;
    for (i, c) in s.bytes().enumerate() {
        if !(48..=57).contains(&c) {
            return (value, i);
        }
        value *= 10;
        value += c as i32 - 48;
    }
    (value, s.len())
}

pub fn parse(s: &str) -> (Vec<i32>, Vec<i32>) {
    let re = Regex::new(r#"(\d+)\s+(\d+)"#).unwrap();
    let (mut a, mut b): (Vec<i32>, Vec<i32>) = s
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let cap = re.captures(line).unwrap();
            (
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
            )
        })
        .unzip();
    a.sort_unstable();
    b.sort_unstable();
    (a, b)
}

#[derive(Clone, Debug, Copy)]
pub struct IdentityHasherGenerator;

impl BuildHasher for IdentityHasherGenerator {
    type Hasher = IdentityHasher;

    fn build_hasher(&self) -> Self::Hasher {
        IdentityHasher(0)
    }
}

pub struct IdentityHasher(i32);

impl Hasher for IdentityHasher {
    fn finish(&self) -> u64 {
        self.0 as u64
    }

    fn write(&mut self, _bytes: &[u8]) {
        unimplemented!("Only implemented for i32")
    }

    fn write_i32(&mut self, i: i32) {
        self.0 = i
    }
}
