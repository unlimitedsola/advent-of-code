use std::collections::HashSet;

use grid::Grid;
use indoc::indoc;
use itertools::Itertools;
use pathfinding::prelude::{astar_bag, dijkstra};

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
    let mut grid = parse(input);
    let start = find(&grid, 'S');
    let end = find(&grid, 'E');

    let path = dijkstra(
        &(start, (0isize, 1isize)),
        |&(p, d)| {
            let mut next = vec![];
            for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                if dir == (-d.0, -d.1) {
                    continue;
                }
                let np = v2p(p, dir);
                let cost: i64 = if d == dir { 1 } else { 1001 };
                match grid.get(np.0, np.1) {
                    Some(&'.') | Some(&'E') => next.push(((np, dir), cost)),
                    _ => {}
                }
            }
            next
        },
        |&(p, _)| p == end,
    );

    let (path, cost) = path.unwrap();
    for (i, d) in path {
        let d_char = match d {
            (0, 1) => '>',
            (0, -1) => '<',
            (1, 0) => 'v',
            (-1, 0) => '^',
            _ => unreachable!(),
        };
        *grid.get_mut(i.0, i.1).unwrap() = d_char;
    }
    print_grid(&grid);
    cost as u64
}

fn find(grid: &Grid<char>, c: char) -> (isize, isize) {
    let p = grid
        .indexed_iter()
        .find_map(|(pos, &cc)| if cc == c { Some(pos) } else { None })
        .unwrap();
    (p.0 as isize, p.1 as isize)
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
    dbg!(solve2(&input!()));
}

fn solve2(input: &str) -> u64 {
    let grid = parse(input);
    let start = find(&grid, 'S');
    let end = find(&grid, 'E');

    let path = astar_bag(
        &(start, (0isize, 1isize)),
        |&(p, d)| {
            let mut next = vec![];
            for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                if dir == (-d.0, -d.1) {
                    continue;
                }
                let np = v2p(p, dir);
                let cost: i64 = if d == dir { 1 } else { 1001 };
                match grid.get(np.0, np.1) {
                    Some(&'.') | Some(&'E') => next.push(((np, dir), cost)),
                    _ => {}
                }
            }
            next
        },
        |_| 0,
        |&(p, _)| p == end,
    );

    let (solves, _cost) = path.unwrap();
    let nodes = solves.flatten().map(|(p, _)| p).collect::<HashSet<_>>();
    nodes.len() as u64
}

const EXAMPLE: &str = indoc! {"
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"};

#[test]
fn test_example() {
    assert_eq!(solve1(EXAMPLE), 7036);
    assert_eq!(solve2(EXAMPLE), 45);
}
