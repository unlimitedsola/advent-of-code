use std::collections::HashSet;

use grid::Grid;
use indoc::indoc;
use itertools::Itertools;

use crate::aoc::input;

fn parse(input: &str) -> Grid<char> {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
        .into()
}

#[test]
fn part1() {
    dbg!(solve1(&input!()));
}

fn solve1(input: &str) -> u64 {
    let mut grid = parse(input);
    let freqs: HashSet<char> = grid.iter().filter(|c| **c != '.').copied().collect();
    let mut loc_map = HashSet::new();
    for freq in freqs {
        let pos = grid
            .indexed_iter()
            .filter(|(_, c)| **c == freq)
            .map(|(i, _)| i)
            .collect_vec();
        pos.into_iter().tuple_combinations().for_each(|(a, b)| {
            let (a, b) = ((a.0 as isize, a.1 as isize), (b.0 as isize, b.1 as isize));
            let d = tuple_minus(b, a);
            let b2 = tuple_plus(b, d);
            let a2 = tuple_minus(a, d);
            for p in [a2, b2] {
                if grid.get_mut(p.0, p.1).is_some() {
                    loc_map.insert(p);
                }
            }
        });
    }
    loc_map.len() as u64
}

fn tuple_plus(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
    (a.0 + b.0, a.1 + b.1)
}

fn tuple_minus(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
    (a.0 - b.0, a.1 - b.1)
}

#[test]
fn part2() {
    dbg!(solve2(&input!()));
}

fn solve2(input: &str) -> u64 {
    let mut grid = parse(input);
    let freqs: HashSet<char> = grid.iter().filter(|c| **c != '.').copied().collect();
    let mut loc_map = HashSet::new();
    for freq in freqs {
        let pos = grid
            .indexed_iter()
            .filter(|(_, c)| **c == freq)
            .map(|(i, _)| i)
            .collect_vec();
        pos.into_iter().tuple_combinations().for_each(|(a, b)| {
            let (a, b) = ((a.0 as isize, a.1 as isize), (b.0 as isize, b.1 as isize));
            loc_map.insert(a);
            loc_map.insert(b);
            let d = tuple_minus(b, a);
            let mut b2 = tuple_plus(b, d);
            while let Some(v) = grid.get_mut(b2.0, b2.1) {
                if *v == '.' {
                    *v = '#';
                }
                loc_map.insert(b2);
                b2 = tuple_plus(b2, d);
            }
            let mut a2 = tuple_minus(a, d);
            while let Some(v) = grid.get_mut(a2.0, a2.1) {
                if *v == '.' {
                    *v = '#';
                }
                loc_map.insert(a2);
                a2 = tuple_minus(a2, d);
            }
        });
    }
    grid.iter_rows().for_each(|row| {
        row.for_each(|c| print!("{}", c));
        println!();
    });
    println!();
    loc_map.len() as u64
}

const EXAMPLE: &str = indoc! {"
    ............
    ........0...
    .....0......
    .......0....
    ....0.......
    ......A.....
    ............
    ............
    ........A...
    .........A..
    ............
    ............
"};

#[test]
fn test_example() {
    assert_eq!(solve1(EXAMPLE), 14);
    assert_eq!(solve2(EXAMPLE), 34);
}
