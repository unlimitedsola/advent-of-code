use indoc::indoc;
use itertools::Itertools;
use std::iter::zip;

use crate::aoc::input;

type Grid = grid::Grid<char>;

#[test]
fn part1() {
    let grids = parse(&input!());
    let sum: usize = grids.into_iter().map(|g| solve(g, 0)).sum();
    dbg!(sum);
}

#[test]
fn part2() {
    let grids = parse(&input!());
    let sum: usize = grids.into_iter().map(|g| solve(g, 1)).sum();
    dbg!(sum);
}

fn solve(mut grid: Grid, tolerance: usize) -> usize {
    (1..grid.rows())
        .find(|&i| mirror_diff(&grid, i) == tolerance)
        .map(|i| i * 100)
        .or_else(|| {
            grid.transpose();
            (1..grid.rows()).find(|&i| mirror_diff(&grid, i) == tolerance)
        })
        .unwrap()
}

fn mirror_diff(grid: &Grid, i: usize) -> usize {
    if i == 0 {
        unreachable!()
    }
    zip((0..i).rev(), i..grid.rows())
        .map(|(l, r)| {
            grid.iter_row(l)
                .zip(grid.iter_row(r))
                .filter(|(l, r)| l != r)
                .count()
        })
        .sum()
}

fn parse(input: &str) -> Vec<Grid> {
    let mut grids = vec![];
    let mut lines = input.lines();
    loop {
        let grid = (&mut lines)
            .take_while(|l| !l.is_empty())
            .map(|l| l.chars().collect_vec())
            .collect_vec();
        if grid.is_empty() {
            break;
        }
        grids.push(grid.into());
    }
    grids
}

const EXAMPLE: &str = indoc! {"
    #.##..##.
    ..#.##.#.
    ##......#
    ##......#
    ..#.##.#.
    ..##..##.
    #.#.##.#.

    #...##..#
    #....#..#
    ..##..###
    #####.##.
    #####.##.
    ..##..###
    #....#..#
"};

#[test]
fn test_p1() {
    let grids = parse(EXAMPLE);
    assert_eq!(grids.len(), 2);
    assert_eq!(solve(grids[0].clone(), 0), 5);
    assert_eq!(solve(grids[1].clone(), 0), 400);
}

#[test]
fn test_p2() {
    let grids = parse(EXAMPLE);
    assert_eq!(solve(grids[0].clone(), 1), 300);
    assert_eq!(solve(grids[1].clone(), 1), 100);
}
