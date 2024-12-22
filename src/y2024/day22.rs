use std::collections::{HashMap, HashSet};

use indoc::indoc;
use itertools::Itertools;

use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve1(&input!()));
}

fn parse(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse().unwrap()).collect_vec()
}

fn solve1(input: &str) -> u64 {
    let nums = parse(input);

    nums.iter()
        .map(|&n| {
            let mut n = n;
            for _ in 0..2000 {
                n = rng_next(n);
            }
            n
        })
        .sum()
}

fn rng_next(mut n: u64) -> u64 {
    n ^= n << 6;
    n &= 0xff_ffff;
    n ^= n >> 5;
    n &= 0xff_ffff;
    n ^= n << 11;
    n &= 0xff_ffff;
    n
}

#[test]
fn part2() {
    dbg!(solve2(&input!()));
}

fn solve2(input: &str) -> u64 {
    let nums = parse(input);

    let mut map = HashMap::new();
    let mut visit = HashSet::new();

    for n in nums {
        let mut seq = 0u32;
        let mut prev = n;
        let mut prev_r = (prev % 10) as i8;
        let mut next = rng_next(prev);
        let mut next_r = (next % 10) as i8;

        // init first 3
        for _ in 0..3 {
            let d = prev_r - next_r;
            seq = (seq << 8) | (d as u8 as u32);
            prev = next;
            prev_r = next_r;
            next = rng_next(prev);
            next_r = (next % 10) as i8;
        }

        for _ in 3..2000 {
            let d = prev_r - next_r;
            seq = (seq << 8) | (d as u8 as u32);
            if visit.insert(seq) {
                *map.entry(seq).or_insert(0) += next_r as u64;
            }
            prev = next;
            prev_r = next_r;
            next = rng_next(prev);
            next_r = (next % 10) as i8;
        }
        visit.clear()
    }
    map.into_values().max().unwrap()
}

const EXAMPLE: &str = indoc! {"
1
2
3
2024
"};

#[test]
fn test_example() {
    assert_eq!(rng_next(123), 15887950);
    assert_eq!(solve2(EXAMPLE), 23);
}
