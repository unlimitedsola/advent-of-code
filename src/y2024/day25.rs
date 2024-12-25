use grid::Grid;
use itertools::Itertools;

use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve1(&input!()));
}

fn parse(input: &str) -> (Vec<Grid<char>>, Vec<Grid<char>>) {
    let mut grids = vec![];
    let mut lines = input.lines().peekable();
    while lines.peek().is_some() {
        let mut ll = vec![];
        for l in lines.by_ref() {
            if l.is_empty() {
                break;
            }
            ll.push(l.chars().collect_vec());
        }
        grids.push(Grid::from(ll));
    }
    grids
        .into_iter()
        .partition(|g| g.iter_row(0).all(|&c| c == '#'))
}

fn solve1(input: &str) -> u64 {
    let (locks, keys) = parse(input);
    let mut fit = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            if key
                .indexed_iter()
                .filter(|(_, &c)| c == '#')
                .all(|(i, _)| lock.get(i.0, i.1) == Some(&'.'))
            {
                fit += 1;
            }
        }
    }
    fit
}
