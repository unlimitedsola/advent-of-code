use regex::Regex;
use std::sync::LazyLock;

use crate::aoc::input;

static RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"don't\(\)|do\(\)|mul\((\d+),(\d+)\)").unwrap());

#[test]
fn part1() {
    let input = input!();
    let sum = RE
        .captures_iter(&input)
        .map(|cap| cap[1].parse::<u64>().unwrap() * cap[2].parse::<u64>().unwrap())
        .sum::<u64>();
    dbg!(sum);
}

#[test]
fn part2() {
    let input = input!();
    let mut enable = true;
    let sum = RE
        .captures_iter(&input)
        .map(|cap| match &cap[0] {
            "do()" => {
                enable = true;
                0
            }
            "don't()" => {
                enable = false;
                0
            }
            _ => {
                if enable {
                    cap[1].parse::<u64>().unwrap() * cap[2].parse::<u64>().unwrap()
                } else {
                    0
                }
            }
        })
        .sum::<u64>();
    dbg!(sum);
}
