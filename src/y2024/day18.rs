use grid::Grid;
use indoc::indoc;
use itertools::Itertools;
use pathfinding::prelude::bfs;

use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve1(&input!(), 1024, 70));
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec()
}

fn solve1(input: &str, take: usize, max: usize) -> u64 {
    let bytes = &parse(input)[..take];
    let mut grid = Grid::init(max + 1, max + 1, '.');
    for &pos in bytes {
        grid[pos] = '#';
    }
    print_grid(&grid);

    let path = reach(&grid, (0isize, 0isize), (max as isize, max as isize)).unwrap();

    (path.len() - 1) as u64
}

fn reach(
    grid: &Grid<char>,
    pos: (isize, isize),
    to: (isize, isize),
) -> Option<Vec<(isize, isize)>> {
    bfs(
        &pos,
        |&prev| {
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
        },
        |&pos| pos == to,
    )
}

fn v2p(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
    (a.0 + b.0, a.1 + b.1)
}

fn print_grid(grid: &Grid<char>) {
    for r in grid.iter_rows() {
        for c in r {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

#[test]
fn part2() {
    dbg!(solve2(&input!(), 70));
}

fn solve2(input: &str, max: usize) -> (usize, usize) {
    let bytes = parse(input).into_iter();
    let mut grid = Grid::init(max + 1, max + 1, '.');
    for pos in bytes {
        grid[pos] = '#';
        if reach(&grid, (0isize, 0isize), (max as isize, max as isize)).is_none() {
            return pos;
        }
    }
    unreachable!()
}

const EXAMPLE: &str = indoc! {"
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"};

#[test]
fn test_example() {
    assert_eq!(solve1(EXAMPLE, 12, 6), 22);
    assert_eq!(solve2(EXAMPLE, 6), (6, 1));
}
