use std::collections::HashMap;

use indoc::indoc;
use itertools::Itertools;

use crate::aoc::input;

type Grid = grid::Grid<char>;

fn parse_grid(input: &str) -> Grid {
    input
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec()
        .into()
}

#[test]
fn part1() {
    let grid = parse_grid(&input!());
    dbg!(solve1(grid));
}

fn solve1(mut grid: Grid) -> usize {
    tick(&mut grid);
    weight(&grid)
}

#[test]
fn part2() {
    let grid = parse_grid(&input!());
    dbg!(solve2(grid));
}

fn solve2(mut grid: Grid) -> usize {
    let mut seen = HashMap::new();
    let count = 1_000_000_000;
    for i in 0..count {
        if let Some(j) = seen.get(&grid) {
            let cycle = i - j;
            let remaining = count - i;
            let remaining = remaining % cycle;
            for _ in 0..remaining {
                tick4(&mut grid);
            }
            break;
        }
        seen.insert(grid.clone(), i);
        tick4(&mut grid);
    }
    weight(&grid)
}

fn tick4(grid: &mut Grid) {
    for _ in 0..4 {
        tick(grid);
        grid.rotate_right();
    }
}

fn tick(grid: &mut Grid) {
    let mut heights = vec![0; grid.cols()];
    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            let c = grid[(y, x)];
            match c {
                'O' => {
                    grid[(y, x)] = '.';
                    grid[(heights[x], x)] = 'O';
                    heights[x] += 1;
                }
                '#' => {
                    heights[x] = y + 1;
                }
                _ => {}
            }
        }
    }
}

fn weight(grid: &Grid) -> usize {
    grid.indexed_iter()
        .filter(|&(_, &c)| c == 'O')
        .map(|((y, _), _)| grid.rows() - y)
        .sum()
}

const EXAMPLE: &str = indoc! {"
    O....#....
    O.OO#....#
    .....##...
    OO.#O....O
    .O.....O#.
    O.#..O.#.#
    ..O..#O..O
    .......O..
    #....###..
    #OO..#....
"};

#[test]
fn test_p1() {
    assert_eq!(solve1(parse_grid(EXAMPLE)), 136)
}

#[test]
fn test_p2() {
    assert_eq!(solve2(parse_grid(EXAMPLE)), 64)
}
