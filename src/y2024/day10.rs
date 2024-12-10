use std::collections::HashSet;

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
    let starting_pos = grid
        .indexed_iter()
        .filter(|(_, &c)| c == '0')
        .map(|x| x.0)
        .collect_vec();
    let mut sum = 0;
    for sp in starting_pos {
        sum += trail_score(&grid, (sp.0 as isize, sp.1 as isize));
    }
    sum
}

fn trail_score(grid: &Grid<char>, pos: (isize, isize)) -> u64 {
    let mut branches = HashSet::new();
    branches.insert(pos);
    let mut branches_next = HashSet::new();
    for cc in '1'..='9' {
        for sp in branches {
            for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let pos = tuple_plus(sp, dir);
                if let Some(&c) = grid.get(pos.0, pos.1) {
                    if c == cc {
                        branches_next.insert(pos);
                    }
                }
            }
        }
        branches = branches_next;
        branches_next = HashSet::new();
    }
    branches.len() as u64
}

fn tuple_plus(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
    (a.0 + b.0, a.1 + b.1)
}

#[test]
fn part2() {
    dbg!(solve2(&input!()));
}

fn solve2(input: &str) -> u64 {
    let grid = parse(input);
    let starting_pos = grid
        .indexed_iter()
        .filter(|(_, &c)| c == '0')
        .map(|x| x.0)
        .collect_vec();
    let mut sum = 0;
    for sp in starting_pos {
        sum += trail_score_2(&grid, (sp.0 as isize, sp.1 as isize));
    }
    sum
}

fn trail_score_2(grid: &Grid<char>, pos: (isize, isize)) -> u64 {
    let mut branches = vec![pos];
    let mut branches_next = vec![];
    for cc in '1'..='9' {
        for sp in branches {
            for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let pos = tuple_plus(sp, dir);
                if let Some(&c) = grid.get(pos.0, pos.1) {
                    if c == cc {
                        branches_next.push(pos);
                    }
                }
            }
        }
        branches = branches_next;
        branches_next = vec![];
    }
    branches.len() as u64
}

const EXAMPLE: &str = indoc! {"
    89010123
    78121874
    87430965
    96549874
    45678903
    32019012
    01329801
    10456732
"};

#[test]
fn test_example() {
    assert_eq!(solve1(EXAMPLE), 36);
    assert_eq!(solve2(EXAMPLE), 81);
}
