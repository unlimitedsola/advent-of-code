use indoc::indoc;
use itertools::Itertools;
use polyfit_rs::polyfit_rs::polyfit;
use std::collections::{HashSet, VecDeque};
use std::ops::{Deref, DerefMut, Index};

use crate::aoc::input;

#[test]
fn part1() {
    let grid = parse_grid(&input!());
    dbg!(solve1(&grid, 64));
}

#[test]
fn part2() {
    let grid = parse_grid(&input!());
    dbg!(solve2(&grid));
}

struct Grid(grid::Grid<char>);

fn parse_grid(input: &str) -> Grid {
    Grid(
        input
            .lines()
            .map(|line| line.chars().collect())
            .collect_vec()
            .into(),
    )
}

type Pos = (isize, isize);

fn solve1(grid: &Grid, steps: usize) -> usize {
    let oddness = steps % 2;
    let mut cnt = 0;
    let mut vis = HashSet::new();
    let mut q = VecDeque::new();
    let start = grid.indexed_iter().find(|(_, &c)| c == 'S').unwrap().0;
    let start = (start.0 as isize, start.1 as isize);
    q.push_back((start, 0));
    while let Some((pos, step)) = q.pop_front() {
        if step > steps {
            break;
        }
        if vis.contains(&pos) {
            continue;
        }
        vis.insert(pos);
        if step % 2 == oddness {
            cnt += 1;
        }
        for next in grid.neighbors(pos) {
            if grid[next] != '#' {
                q.push_back((next, step + 1));
            }
        }
    }
    cnt
}

fn solve2(grid: &Grid) -> usize {
    let steps = 65;
    let x = [0, 1, 2, 3].map(|x| steps + x * grid.size().0);
    let y = x.map(|x| solve1(grid, x) as f64);
    let coefficients = polyfit(&x.map(|x| x as f64), &y, 2).unwrap();
    let x = 26501365f64;
    (coefficients[2] * x * x + coefficients[1] * x + coefficients[0]) as usize
}

impl Grid {
    fn neighbors(&self, pos: Pos) -> [Pos; 4] {
        let (row, col) = pos;
        [
            (row - 1, col),
            (row + 1, col),
            (row, col - 1),
            (row, col + 1),
        ]
    }
}

impl Index<Pos> for Grid {
    type Output = char;

    fn index(&self, index: Pos) -> &Self::Output {
        let (row, col) = index;
        let row = row.rem_euclid(self.rows() as isize) as usize;
        let col = col.rem_euclid(self.cols() as isize) as usize;
        &self.0[(row, col)]
    }
}

impl Deref for Grid {
    type Target = grid::Grid<char>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

const EXAMPLE: &str = indoc! {"
    ...........
    .....###.#.
    .###.##..#.
    ..#.#...#..
    ....#.#....
    .##..S####.
    .##..#...#.
    .......##..
    .##.#.####.
    .##..##.##.
    ...........
"};

#[test]
fn test_example() {
    let grid = parse_grid(EXAMPLE);
    assert_eq!(solve1(&grid, 6), 16);
}
