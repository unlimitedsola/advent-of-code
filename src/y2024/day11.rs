use std::collections::HashMap;

use itertools::Itertools;
use num::Integer;

use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve(&input!(), false));
}

fn parse(input: &str) -> Vec<u64> {
    input
        .lines()
        .flat_map(|l| l.split_ascii_whitespace().map(|n| n.parse().unwrap()))
        .collect_vec()
}

fn solve(input: &str, p2: bool) -> u64 {
    let loops = if p2 { 75 } else { 25 };
    let num = parse(input);
    let mut num_map = HashMap::new();
    for n in num {
        *num_map.entry(n).or_insert(0u64) += 1;
    }
    for _ in 0..loops {
        num_map = blink1(num_map);
    }
    num_map.values().sum()
}

fn blink1(map: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new = HashMap::new();
    for (n, cnt) in map {
        match n {
            0 => *new.entry(1).or_insert(0) += cnt,
            _ => {
                let sn = format!("{}", n);
                if sn.len().is_even() {
                    let (a, b) = sn.split_at(sn.len() / 2);
                    let (a, b) = (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap());
                    *new.entry(a).or_insert(0) += cnt;
                    *new.entry(b).or_insert(0) += cnt;
                } else {
                    *new.entry(n * 2024).or_insert(0) += cnt;
                }
            }
        }
    }
    new
}

#[test]
fn part2() {
    dbg!(solve(&input!(), true));
}

const EXAMPLE2: &str = "125 17";

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE2, false), 55312);
    // assert_eq!(solve2(EXAMPLE), 81);
}
