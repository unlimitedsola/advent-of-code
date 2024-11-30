use itertools::Itertools;
use num::Zero;

use crate::aoc::input;

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|l| l.split_ascii_whitespace())
        .map(|nums| nums.map(|n| n.parse::<i64>().unwrap()).collect_vec())
        .collect_vec()
}

#[test]
fn part1() {
    let nums = parse_input(&input!());

    let sum = nums.iter().cloned().map(extrapolate1).sum::<i64>();

    dbg!(sum);
}

fn extrapolate1(mut nums: Vec<i64>) -> i64 {
    let mut lasts = vec![];
    while nums.iter().any(|n| !n.is_zero()) {
        for i in 0..nums.len() - 1 {
            nums[i] = nums[i + 1] - nums[i];
        }
        lasts.push(nums.pop().unwrap());
    }
    lasts.iter().sum()
}

#[test]
fn part2() {
    let nums = parse_input(&input!());

    let sum = nums.iter().cloned().map(extrapolate2).sum::<i64>();
    dbg!(sum);
}

fn extrapolate2(mut nums: Vec<i64>) -> i64 {
    let mut firsts = vec![];
    while nums.iter().any(|n| !n.is_zero()) {
        firsts.push(nums[0]);
        for i in 0..nums.len() - 1 {
            nums[i] = nums[i + 1] - nums[i];
        }
        nums.pop().unwrap();
    }
    firsts.reverse();
    firsts.iter().fold(0, |acc, n| n - acc)
}

#[test]
fn part2_test() {
    assert_eq!(extrapolate2(vec![10, 13, 16, 21, 30, 45]), 5);
}
