use grid::Grid;
use indoc::indoc;
use itertools::Itertools;
use std::collections::HashSet;

use crate::aoc::input;

fn parse(input: &str) -> Grid<char> {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    Grid::from(grid)
}

#[test]
fn part1() {
    let grid = parse(&input!());
    dbg!(solve1(grid));
}

fn solve1(grid: Grid<char>) -> u64 {
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let pos = grid.indexed_iter().find(|(_, &c)| c == '^').unwrap().0;
    let mut pos = (pos.0 as isize, pos.1 as isize);
    visited.insert(pos);
    let mut dir = (-1, 0);
    loop {
        let next = (pos.0 + dir.0, pos.1 + dir.1);
        match grid.get(next.0, next.1) {
            Some('#') => dir = rotate_right(dir),
            None => break,
            _ => {
                visited.insert(next);
                pos = next;
            }
        }
    }
    visited.len() as u64
}

fn rotate_right(dir: (isize, isize)) -> (isize, isize) {
    let (x, y) = dir;
    (y, -x)
}

#[test]
fn part2() {
    let grid = parse(&input!());
    dbg!(solve2(grid));
}

fn solve2(mut grid: Grid<char>) -> u64 {
    let s_pos = grid.indexed_iter().find(|(_, &c)| c == '^').unwrap().0;
    grid[s_pos] = '.';
    let s_pos = (s_pos.0 as isize, s_pos.1 as isize);
    let mut pos = s_pos;
    let mut dir = (-1, 0);
    let mut obstacles: HashSet<(isize, isize)> = HashSet::new();
    loop {
        let forward = (pos.0 + dir.0, pos.1 + dir.1);
        match grid.get(forward.0, forward.1).copied() {
            Some('#') => dir = rotate_right(dir),
            None => break,
            Some('.') => {
                *grid.get_mut(forward.0, forward.1).unwrap() = '#';
                if does_loop(&grid, s_pos) {
                    obstacles.insert(forward);
                }
                *grid.get_mut(forward.0, forward.1).unwrap() = '.';
                pos = forward;
            }
            _ => unreachable!(),
        }
    }
    obstacles.remove(&s_pos);
    obstacles.len() as u64
}

fn does_loop(grid: &Grid<char>, start: (isize, isize)) -> bool {
    let mut visited: HashSet<((isize, isize), (isize, isize))> = HashSet::new();
    let mut pos = (start.0, start.1);
    let mut dir = (-1, 0);
    visited.insert((pos, dir));
    loop {
        let next = (pos.0 + dir.0, pos.1 + dir.1);
        match grid.get(next.0, next.1) {
            Some('#') => dir = rotate_right(dir),
            None => break,
            Some('.') => {
                if visited.contains(&(next, dir)) {
                    return true;
                }
                visited.insert((next, dir));
                pos = next;
            }
            _ => unreachable!(),
        }
    }
    false
}

const EXAMPLE: &str = indoc! {"
    ....#.....
    .........#
    ..........
    ..#.......
    .......#..
    ..........
    .#..^.....
    ........#.
    #.........
    ......#...
"};

#[test]
fn test_example() {
    let grid = parse(EXAMPLE);
    assert_eq!(solve1(grid.clone()), 41);
    assert_eq!(solve2(grid.clone()), 6);
}
