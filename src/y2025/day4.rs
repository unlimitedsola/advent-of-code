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
    let mut valid = 0;
    for r in 0..grid.rows() {
        for c in 0..grid.cols() {
            // check adjacent
            let val = grid[(r, c)];
            if val != '@' {
                continue;
            }
            let neighbors = neighbors(&grid, (r, c));
            if neighbors < 4 {
                valid += 1;
            }
        }
    }
    valid
}

fn neighbors(grid: &Grid<char>, (r, c): (usize, usize)) -> u32 {
    let mut neighbors = 0;
    for (dr, dc) in [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ] {
        let nr = r as isize + dr;
        let nc = c as isize + dc;
        let Some(&neighbor) = grid.get(nr, nc) else {
            continue;
        };
        if neighbor == '@' {
            neighbors += 1;
        }
    }
    neighbors
}

#[test]
fn part2() {
    dbg!(solve2(&input!()));
}

fn solve2(input: &str) -> u64 {
    let mut grid = parse(input);
    let mut total_removed = 0;
    let mut has_removed = true;
    while has_removed {
        has_removed = false;
        for r in 0..grid.rows() {
            for c in 0..grid.cols() {
                // check adjacent
                let val = grid[(r, c)];
                if val != '@' {
                    continue;
                }
                let neighbors = neighbors(&grid, (r, c));
                if neighbors < 4 {
                    has_removed = true;
                    total_removed += 1;
                    grid[(r, c)] = '.';
                }
            }
        }
    }
    total_removed
}

const EXAMPLE: &str = indoc! {"
    ..@@.@@@@.
    @@@.@.@.@@
    @@@@@.@.@@
    @.@@@@..@.
    @@.@@@@.@@
    .@@@@@@@.@
    .@.@.@.@@@
    @.@@@.@@@@
    .@@@@@@@@.
    @.@.@@@.@.
"};

#[test]
fn test_example() {
    assert_eq!(solve1(EXAMPLE), 13);
    assert_eq!(solve2(EXAMPLE), 43);
}
