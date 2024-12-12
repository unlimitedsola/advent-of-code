use std::collections::HashSet;

use grid::Grid;
use indoc::indoc;
use itertools::Itertools;

use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve(&input!(), false));
}

fn parse(input: &str) -> Grid<char> {
    input
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec()
        .into()
}

fn solve(input: &str, p2: bool) -> u64 {
    let grid = parse(input);
    let mut sum = 0;
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    for (pos, &c) in grid.indexed_iter() {
        let pos = (pos.0 as isize, pos.1 as isize);
        if visited.contains(&pos) {
            continue;
        }
        let (area, perimeter, corner) = visit(&grid, &mut visited, pos, c);
        if p2 {
            println!("{}={}*{}", c, area, corner);
            sum += area * corner;
        } else {
            println!("{}={}*{}", c, area, perimeter);
            sum += area * perimeter;
        }
    }
    sum
}

fn visit(
    grid: &Grid<char>,
    visited: &mut HashSet<(isize, isize)>,
    pos: (isize, isize),
    c: char,
) -> (u64, u64, u64) {
    let mut corner = 0u64;
    let mut perimeter = 0u64;
    let mut area = 0u64;
    let mut to_visit = vec![pos];
    while let Some(pos) = to_visit.pop() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        area += 1;
        perimeter += calc_perimeter(grid, pos, c);
        corner += calc_corners(grid, pos, c);
        for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next = (pos.0 + dir.0, pos.1 + dir.1);
            match grid.get(next.0, next.1) {
                Some(&e) if e == c => {
                    to_visit.push(next);
                }
                _ => continue,
            }
        }
    }
    (area, perimeter, corner)
}

fn calc_perimeter(grid: &Grid<char>, pos: (isize, isize), c: char) -> u64 {
    let mut perimeter = 0u64;
    for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let next = (pos.0 + dir.0, pos.1 + dir.1);
        match grid.get(next.0, next.1) {
            Some(&e) if e == c => {}
            _ => {
                perimeter += 1;
            }
        }
    }
    // println!("{}@{:?}={}", c, pos, perimeter);
    perimeter
}

#[test]
fn part2() {
    dbg!(solve(&input!(), true));
}

fn calc_corners(grid: &Grid<char>, pos: (isize, isize), c: char) -> u64 {
    let mut corners = 0u64;
    for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let y_dir = rotate_right(dir);
        let d_dir = tuple_plus(dir, y_dir);
        let x = tuple_plus(pos, dir);
        let y = tuple_plus(pos, y_dir);
        let d = tuple_plus(pos, d_dir);
        let x = grid.get(x.0, x.1).copied();
        let y = grid.get(y.0, y.1).copied();
        let d = grid.get(d.0, d.1).copied();

        if (x != Some(c) && y != Some(c)) || (x == Some(c) && y == Some(c) && d != x) {
            corners += 1;
        }
    }
    // println!("{}@{:?}={}", c, pos, corners);
    corners
}

fn tuple_plus(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
    (a.0 + b.0, a.1 + b.1)
}

fn rotate_right(dir: (isize, isize)) -> (isize, isize) {
    let (x, y) = dir;
    (y, -x)
}

const EXAMPLE: &str = indoc! {"
    RRRRIICCFF
    RRRRIICCCF
    VVRRRCCFFF
    VVRCCCJFFF
    VVVVCJJCFE
    VVIVCCJJEE
    VVIIICJJEE
    MIIIIIJJEE
    MIIISIJEEE
    MMMISSJEEE
"};

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE, false), 1930);
    assert_eq!(solve(EXAMPLE, true), 1206);
}
