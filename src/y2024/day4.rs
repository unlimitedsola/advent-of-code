use grid::Grid;
use itertools::Itertools;

use crate::aoc::input;

#[test]
fn part1() {
    let input = input!();
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let grid = Grid::from(grid);
    dbg!(p1(&grid));
}

fn p1(grid: &Grid<char>) -> u32 {
    let mut sum = 0;
    for (pos, &c) in grid.indexed_iter() {
        if c != 'X' {
            continue;
        }
        sum += xmas_count(grid, pos);
    }
    sum
}

fn xmas_count(grid: &Grid<char>, (x, y): (usize, usize)) -> u32 {
    let mut cnt = 0;
    // find 'M' in 8 directions
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let mut x = x as isize;
            let mut y = y as isize;
            x += dx;
            y += dy;
            if let Some('M') = grid.get(x, y) {
                if has_xmas(grid, (x, y), (dx, dy)) {
                    cnt += 1;
                }
            }
        }
    }
    cnt
}

// continue in the same direction and try to find 'A' and 'S'
fn has_xmas(grid: &Grid<char>, (mut x, mut y): (isize, isize), (dx, dy): (isize, isize)) -> bool {
    x += dx;
    y += dy;
    match grid.get(x, y) {
        Some('A') => {}
        _ => return false,
    }

    x += dx;
    y += dy;
    match grid.get(x, y) {
        Some('S') => {}
        _ => return false,
    }
    true
}

#[test]
fn part2() {
    let input = input!();
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let grid = Grid::from(grid);
    dbg!(p2(&grid));
}

fn p2(grid: &Grid<char>) -> u32 {
    let mut sum = 0;
    for (pos, &c) in grid.indexed_iter() {
        if c != 'A' {
            continue;
        }
        if has_x_mas(grid, pos) {
            sum += 1;
        }
    }
    sum
}

fn has_x_mas(grid: &Grid<char>, (x, y): (usize, usize)) -> bool {
    let mut chars = vec![];
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let mut x = x as isize;
            let mut y = y as isize;
            x += dx;
            y += dy;
            if let Some(c) = grid.get(x, y) {
                chars.push(c)
            } else {
                // must have all 8 directions
                return false;
            }
        }
    }
    matches!(
        chars.as_slice(),
        ['M', _, 'M', _, _, 'S', _, 'S']
            | ['S', _, 'S', _, _, 'M', _, 'M']
            | ['M', _, 'S', _, _, 'M', _, 'S']
            | ['S', _, 'M', _, _, 'S', _, 'M']
    )
}
