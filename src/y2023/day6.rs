use itertools::Itertools;

use crate::aoc::input;

#[test]
fn part1() {
    let input = input!();
    let (t, d) = input
        .lines()
        .map(|l| l.split_once(':').unwrap().1)
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
        })
        .collect_tuple()
        .unwrap();

    let ans = t
        .zip(d)
        .map(|(t, d)| solve(t as f64, d as f64))
        .reduce(|a, b| a * b)
        .unwrap();
    dbg!(ans);
}

#[test]
fn part2() {
    let input = input!();
    let (t, d) = input
        .lines()
        .map(|l| l.split_once(':').unwrap().1)
        .map(|l| l.replace(' ', "").parse::<u64>().unwrap())
        .collect_tuple()
        .unwrap();

    dbg!(solve(t as f64, d as f64));
}

fn solve(t: f64, d: f64) -> u64 {
    let x1 = ((t - (t * t - 4f64 * d).sqrt()) / 2f64).ceil() as u64;
    let x2 = ((t + (t * t - 4f64 * d).sqrt()) / 2f64).floor() as u64;
    x2 - x1 + 1
}

#[test]
fn test_solve1() {
    assert_eq!(solve(7f64, 9f64), 4);
    assert_eq!(solve(15f64, 40f64), 8);
}
