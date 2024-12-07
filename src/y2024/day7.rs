use indoc::indoc;
use itertools::Itertools;

use crate::aoc::input;

fn parse(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| {
            let (v, n) = line.split_once(": ").unwrap();
            let v: i64 = v.parse().unwrap();
            let n: Vec<i64> = n
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

fn solve(eqs: &[(i64, Vec<i64>)], p2: bool) -> i64 {
    let sum: i64 = eqs
        .iter()
        .filter(|(v, n)| solvable(*v, n[0], &n[1..], p2))
        .map(|(v, _)| *v)
        .sum();
    sum
}

fn solvable(v: i64, acc: i64, n: &[i64], p2: bool) -> bool {
    if n.is_empty() {
        return if v == acc { true } else { false };
    }
    solvable(v, acc * n[0], &n[1..], p2)
        || solvable(v, acc + n[0], &n[1..], p2)
        || (p2 && solvable(v, format!("{}{}", acc, n[0]).parse().unwrap(), &n[1..], p2))
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
