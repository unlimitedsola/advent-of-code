use crate::aoc::input;
use itertools::Itertools;

#[test]
fn part1() {
    dbg!(solve1(&input!()));
}

fn parse(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse::<u64>().unwrap()).collect()
}

fn solve1(input: &str) -> u64 {
    let nums = parse(input);
    let (a, b) = nums
        .into_iter()
        .tuple_combinations()
        .find(|&(a, b)| a + b == 2020)
        .unwrap();
    a * b
}

#[test]
fn part2() {
    dbg!(solve2(&input!()));
}

fn solve2(input: &str) -> u64 {
    let nums = parse(input);
    let (a, b, c) = nums
        .into_iter()
        .tuple_combinations()
        .find(|&(a, b, c)| a + b + c == 2020)
        .unwrap();
    a * b * c
}
