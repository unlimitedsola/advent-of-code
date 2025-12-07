use std::collections::{HashMap, HashSet};

use grid::Grid;
use indoc::indoc;
use itertools::Itertools;

use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve1(&input!()));
}

fn parse(input: &str) -> Grid<char> {
    input
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec()
        .into()
}

fn solve1(input: &str) -> u64 {
    let grid = parse(input);
    let mut beams = HashSet::new();
    let mut iter = grid.iter_rows();
    let mut first = iter.next().unwrap();
    let start = first.find_position(|&&c| c == 'S').unwrap();
    beams.insert(start.0);
    let mut split = 0;
    for row in iter {
        let cur = beams.clone();
        for (i, &c) in row.enumerate() {
            if c != '^' {
                continue;
            }
            if cur.contains(&i) {
                split += 1;
                beams.remove(&i);
                beams.insert(i + 1);
                beams.insert(i - 1);
            }
        }
    }
    split
}

#[test]
fn part2() {
    dbg!(solve2(&input!()));
}

fn solve2(input: &str) -> u64 {
    let grid = parse(input);
    let mut beams = HashMap::new();
    let mut iter = grid.iter_rows();
    let mut first = iter.next().unwrap();
    let start = first.find_position(|&&c| c == 'S').unwrap();
    beams.insert(start.0, 1u64);
    for row in iter {
        let cur = beams.keys().copied().collect::<HashSet<_>>();
        for (i, &c) in row.enumerate() {
            if c != '^' {
                continue;
            }
            if cur.contains(&i) {
                let v = beams.remove(&i).unwrap();
                beams.entry(i + 1).and_modify(|e| *e += v).or_insert(v);
                beams.entry(i - 1).and_modify(|e| *e += v).or_insert(v);
            }
        }
    }
    beams.values().sum()
}

const EXAMPLE: &str = indoc! {"
    .......S.......
    ...............
    .......^.......
    ...............
    ......^.^......
    ...............
    .....^.^.^.....
    ...............
    ....^.^...^....
    ...............
    ...^.^...^.^...
    ...............
    ..^...^.....^..
    ...............
    .^.^.^.^.^...^.
    ...............
"};

#[test]
fn test_example() {
    assert_eq!(solve1(EXAMPLE), 21);
    assert_eq!(solve2(EXAMPLE), 40);
}
