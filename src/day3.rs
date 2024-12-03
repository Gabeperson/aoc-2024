use core::str;

use memchr::memmem::Finder;
use regex::Regex;

pub static SIMPLE: &str = include_str!("../inputs/day3_simple.txt");
pub static REGULAR: &str = include_str!("../inputs/day3.txt");

pub fn run(s: &str) -> (i32, i32) {
    (part1_opt(s), part2(s))
}

pub fn part1(s: &str) -> i32 {
    let re = Regex::new(r#"mul\((\d{1,3}),(\d{1,3})\)"#).unwrap();
    re.captures_iter(s)
        .map(|c| (c[1].parse::<i32>().unwrap() * c[2].parse::<i32>().unwrap()))
        .sum()
}

fn num_parse(s: &[u8]) -> Option<(i32, usize)> {
    let digit1 = if let Some(d) = s.first().copied() {
        if d.is_ascii_digit() {
            d
        } else {
            return None;
        }
    } else {
        return None;
    };
    let digit2 = if let Some(d) = s.get(1).copied() {
        if d.is_ascii_digit() {
            d
        } else {
            return Some((digit1 as i32 - 48, 1));
        }
    } else {
        return Some((digit1 as i32 - 48, 1));
    };
    let digit3 = if let Some(d) = s.get(2).copied() {
        if d.is_ascii_digit() {
            d
        } else {
            return Some(((digit1 as i32 - 48) * 10 + digit2 as i32 - 48, 2));
        }
    } else {
        return Some(((digit1 as i32 - 48) * 10 + digit2 as i32 - 48, 2));
    };
    Some((
        (digit1 as i32 - 48) * 100 + (digit2 as i32 - 48) * 10 + digit3 as i32 - 48,
        3,
    ))
}
fn next_mul<'a>(mut s: &'a [u8], mul_finder: &Finder<'_>) -> Option<(i32, &'a [u8])> {
    loop {
        let found = mul_finder.find(s)?;
        s = &s[found + 4..];
        let num1 = if let Some((num, parsed)) = num_parse(s) {
            s = &s[parsed..];
            num
        } else {
            continue;
        };
        if let Some(b) = s.first().copied() {
            if b != b',' {
                continue;
            }
            s = &s[1..];
        } else {
            continue;
        }
        let num2 = if let Some((num, parsed)) = num_parse(s) {
            s = &s[parsed..];
            num
        } else {
            continue;
        };
        if let Some(b) = s.first().copied() {
            if b != b')' {
                continue;
            }
            s = &s[1..];
        } else {
            continue;
        }
        return Some((num1 * num2, s));
    }
}
fn next_mul_nomemchr(mut s: &str) -> Option<(i32, &str)> {
    loop {
        let found = s.find("mul(")?;
        s = &s[found + 4..];
        let num1 = if let Some((num, parsed)) = num_parse(s.as_bytes()) {
            s = &s[parsed..];
            num
        } else {
            continue;
        };
        if let Some(b) = s.as_bytes().first().copied() {
            if b != b',' {
                continue;
            }
            s = &s[1..];
        } else {
            continue;
        }
        let num2 = if let Some((num, parsed)) = num_parse(s.as_bytes()) {
            s = &s[parsed..];
            num
        } else {
            continue;
        };
        if let Some(b) = s.as_bytes().first().copied() {
            if b != b')' {
                continue;
            }
            s = &s[1..];
        } else {
            continue;
        }
        return Some((num1 * num2, s));
    }
}

pub fn part1_opt(s: &str) -> i32 {
    let mul_finder = Finder::new("mul(");
    let mut input = s.as_bytes();
    let mut n = 0;
    while let Some((num, new_slice)) = next_mul(input, &mul_finder) {
        input = new_slice;
        n += num;
    }
    n
}

pub fn part1_opt_nomemchr(s: &str) -> i32 {
    let mut input = s;
    let mut n = 0;
    while let Some((num, new_slice)) = next_mul_nomemchr(input) {
        input = new_slice;
        n += num;
    }
    n
}

pub fn part2(s: &str) -> i32 {
    let mut input = s.as_bytes();
    let mul_finder = Finder::new("mul(");
    let do_finder = Finder::new("do()");
    let dont_finder = Finder::new("don't()");

    let mut enabled = true;
    let mut num = 0;

    'outer: loop {
        if enabled {
            if let Some(pos) = dont_finder.find(input) {
                let ptr = input[pos..].as_ptr();
                let after_dont = &input[pos..];
                while let Some((n, new_slice)) = next_mul(input, &mul_finder) {
                    input = new_slice;
                    if new_slice.as_ptr() >= ptr {
                        input = after_dont;
                        enabled = false;
                        continue 'outer;
                    }
                    num += n;
                }
            } else {
                while let Some((n, new_slice)) = next_mul(input, &mul_finder) {
                    input = new_slice;
                    num += n;
                }
                return num;
            }
        } else if let Some(pos) = do_finder.find(input) {
            input = &input[pos..];
            enabled = true;
        } else {
            return num;
        }
    }
}

pub fn part2_nomemchr(s: &str) -> i32 {
    let mut input = s;
    let do_finder = Finder::new("do()");
    let dont_finder = Finder::new("don't()");

    let mut enabled = true;
    let mut num = 0;

    'outer: loop {
        if enabled {
            if let Some(pos) = dont_finder.find(input.as_bytes()) {
                let ptr = input[pos..].as_ptr();
                let after_dont = &input[pos..];
                while let Some((n, new_slice)) = next_mul_nomemchr(input) {
                    input = new_slice;
                    if new_slice.as_ptr() >= ptr {
                        input = after_dont;
                        enabled = false;
                        continue 'outer;
                    }
                    num += n;
                }
            } else {
                while let Some((n, new_slice)) = next_mul_nomemchr(input) {
                    input = new_slice;
                    num += n;
                }
                return num;
            }
        } else if let Some(pos) = do_finder.find(input.as_bytes()) {
            input = &input[pos..];
            enabled = true;
        } else {
            return num;
        }
    }
}
