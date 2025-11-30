use grid::Grid;
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
    check_slope(&grid, 3, 1)
}

#[test]
fn part2() {
    dbg!(solve2(&input!()));
}

fn solve2(input: &str) -> u64 {
    let grid = parse(input);
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    slopes
        .iter()
        .map(|&(right, down)| check_slope(&grid, right, down))
        .product()
}

fn check_slope(grid: &Grid<char>, right: usize, down: usize) -> u64 {
    let (mut x, mut y) = (0, 0);
    let mut trees = 0;

    while y < grid.rows() {
        if grid[(y, x % grid.cols())] == '#' {
            trees += 1;
        }
        x += right;
        y += down;
    }

    trees
}
