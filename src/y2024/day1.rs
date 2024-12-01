use std::iter::zip;

use itertools::Itertools;

use crate::aoc::input;

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            let t: (u32, u32) = line
                .split_ascii_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap();
            t
        })
        .unzip()
}

#[test]
fn part1() {
    let (mut va, mut vb) = parse(&input!());
    va.sort();
    vb.sort();
    let sum: u32 = zip(va, vb).map(|(a, b)| a.abs_diff(b)).sum();
    dbg!(sum);
}

#[test]
fn part2() {
    let (va, vb) = parse(&input!());
    let sum: u32 = va
        .iter()
        .map(|&x| x * vb.iter().filter(|&&y| x == y).count() as u32)
        .sum();
    dbg!(sum);
}
