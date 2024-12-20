use grid::Grid;
use indoc::indoc;
use itertools::Itertools;
use pathfinding::prelude::{bfs, bfs_reach};

use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve(&input!(), 2));
}

fn parse(input: &str) -> Grid<char> {
    input
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec()
        .into()
}

fn solve(input: &str, max_dist: usize) -> u64 {
    let grid = parse(input);
    let s = find(&grid, 'S');
    let e = find(&grid, 'E');
    let base = time(&grid, s, e);
    let p1 = reach(&grid, s);
    let p2 = reach(&grid, e);
    let mut sum = 0;
    for (px, cx) in p1 {
        for (py, cy) in p2.clone() {
            let dist = dist(px, py);
            if dist <= max_dist && dist + cx + cy + 100 <= base {
                sum += 1;
            }
        }
    }
    sum
}

fn dist(a: (isize, isize), b: (isize, isize)) -> usize {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as usize
}

fn time(grid: &Grid<char>, s: (isize, isize), e: (isize, isize)) -> usize {
    bfs(
        &s,
        |&prev| {
            let mut next = vec![];
            for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let np = v2p(prev, dir);
                if let Some(&c) = grid.get(np.0, np.1) {
                    if c != '#' {
                        next.push(np);
                    } else {
                        continue;
                    }
                }
            }
            next
        },
        |&pos| pos == e,
    )
    .unwrap()
    .len()
        - 1
}

fn reach(grid: &Grid<char>, pos: (isize, isize)) -> Vec<((isize, isize), usize)> {
    let reachable = bfs_reach(pos, |&prev| {
        let prev = (prev.0, prev.1);
        let mut next = vec![];
        for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let np = v2p(prev, dir);
            if let Some(&c) = grid.get(np.0, np.1) {
                if c == '#' {
                    continue;
                } else {
                    next.push(np);
                }
            }
        }
        next
    });
    let mut res = vec![];
    for (cost, i) in reachable.enumerate() {
        res.push((i, cost));
    }
    res
}

fn v2p(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
    (a.0 + b.0, a.1 + b.1)
}

fn find(grid: &Grid<char>, c: char) -> (isize, isize) {
    let p = grid
        .indexed_iter()
        .find_map(|(pos, &cc)| if cc == c { Some(pos) } else { None })
        .unwrap();
    (p.0 as isize, p.1 as isize)
}

#[test]
fn part2() {
    dbg!(solve(&input!(), 20));
}

const EXAMPLE: &str = indoc! {"
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############

"};

#[test]
fn test_example() {
    let grid = parse(EXAMPLE);
    let s = find(&grid, 'S');
    let e = find(&grid, 'E');
    assert_eq!(time(&grid, s, e), 84);
}
