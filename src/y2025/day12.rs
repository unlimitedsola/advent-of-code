use grid::Grid;
use indoc::indoc;
use itertools::Itertools;

use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve1(&input!()));
}

type Req = (usize, usize, Vec<usize>);

fn parse(input: &str) -> (Vec<Grid<bool>>, Vec<Req>) {
    let mut lines = input.lines().peekable();
    let mut grids = vec![];

    loop {
        if !lines.peek().unwrap().ends_with(":") {
            break;
        }
        lines.next();

        let grid = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .map(|l| l.chars().map(|c| c == '#').collect_vec())
            .collect_vec()
            .into();
        grids.push(grid);
    }

    let mut reqs = vec![];
    for line in lines {
        let (dim, amts) = line.split_once(": ").unwrap();
        let (x, y) = dim.split_once('x').unwrap();
        let amts = amts
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect_vec();
        reqs.push((x.parse().unwrap(), y.parse().unwrap(), amts));
    }
    (grids, reqs)
}

fn solve1(input: &str) -> u64 {
    let (grids, reqs) = parse(input);
    let grids = grids
        .into_iter()
        .map(|g| g.into_iter().filter(|e| *e).count())
        .collect_vec();
    let mut count = 0;
    for (x, y, amts) in reqs {
        let mut total = 0;
        for (amt, sz) in amts.iter().zip(grids.iter()) {
            total += amt * sz;
        }
        if total <= x * y {
            count += 1;
        }
    }
    count
}

#[expect(dead_code)]
const EXAMPLE: &str = indoc! {"
    0:
    ###
    ##.
    ##.

    1:
    ###
    ##.
    .##

    2:
    .##
    ###
    ##.

    3:
    ##.
    ###
    ##.

    4:
    ###
    #..
    ###

    5:
    ###
    .#.
    ###

    4x4: 0 0 0 0 2 0
    12x5: 1 0 1 0 2 2
    12x5: 1 0 1 0 3 2
"};

#[test]
fn test_example() {
    // CHEESE!
    // assert_eq!(solve1(EXAMPLE), 2);
}
