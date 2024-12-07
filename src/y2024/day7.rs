use indoc::indoc;
use itertools::Itertools;

use crate::aoc::input;

fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let (v, n) = line.split_once(": ").unwrap();
            let v: u64 = v.parse().unwrap();
            let n: Vec<u64> = n
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect_vec();
            (v, n)
        })
        .collect_vec()
}

#[test]
fn part1() {
    let eqs = parse(&input!());
    dbg!(solve(&eqs, false));
}

fn solve(eqs: &[(u64, Vec<u64>)], p2: bool) -> u64 {
    let sum: u64 = eqs
        .iter()
        .filter(|(v, n)| {
            let [n, rest @ ..] = n.as_slice() else {
                unreachable!()
            };
            solvable(*v, *n, rest, p2)
        })
        .map(|(v, _)| *v)
        .sum();
    sum
}

fn solvable(v: u64, acc: u64, n: &[u64], p2: bool) -> bool {
    let [n, rest @ ..] = n else {
        return v == acc;
    };
    solvable(v, acc * n, rest, p2)
        || solvable(v, acc + n, rest, p2)
        || (p2 && solvable(v, acc * 10u64.pow(n.ilog10() + 1) + n, rest, p2))
}

#[test]
fn part2() {
    let eqs = parse(&input!());
    dbg!(solve(&eqs, true));
}

const EXAMPLE: &str = indoc! {"
    190: 10 19
    3267: 81 40 27
    83: 17 5
    156: 15 6
    7290: 6 8 6 15
    161011: 16 10 13
    192: 17 8 14
    21037: 9 7 18 13
    292: 11 6 16 20
"};

#[test]
fn test_example() {
    let eqs = parse(EXAMPLE);
    assert_eq!(solve(&eqs, false), 3749);
    assert_eq!(solve(&eqs, true), 11387);
}
