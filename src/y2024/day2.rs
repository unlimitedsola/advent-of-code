use std::cmp::Reverse;
use std::str::FromStr;

use indoc::indoc;
use itertools::Itertools;

use crate::aoc::input;

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(u32::from_str)
                .map(Result::unwrap)
                .collect_vec()
        })
        .collect_vec()
}

fn safe(num: &[u32]) -> bool {
    (num.is_sorted() || num.is_sorted_by_key(Reverse))
        && num
            .iter()
            .tuple_windows()
            .all(|(&a, &b)| (1u32..=3).contains(&a.abs_diff(b)))
}

fn safe_p2(num: &[u32]) -> bool {
    for i in 0..num.len() {
        let mut num = num.to_vec();
        num.remove(i);
        if safe(&num) {
            return true;
        }
    }
    false
}

#[test]
fn part1() {
    let nums = parse(&input!());
    let count = nums.into_iter().filter(|num| safe(num)).count();
    dbg!(count);
}

#[test]
fn part2() {
    let nums = parse(&input!());
    let count = nums.into_iter().filter(|num| safe_p2(num)).count();
    dbg!(count);
}

const EXAMPLE1: &str = indoc! {"
    7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9
"};

#[test]
fn test_example() {
    let nums = parse(EXAMPLE1);
    let count = nums.into_iter().filter(|num| safe(num)).count();
    assert_eq!(count, 2);
}

#[test]
fn test_p1() {
    assert!(safe(&[7, 6, 4, 2, 1]));
}
