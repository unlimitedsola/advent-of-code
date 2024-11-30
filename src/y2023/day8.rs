use std::collections::HashMap;
use std::str::Lines;

use itertools::Itertools;
use num::integer::lcm;

use crate::aoc::input;

fn parse(input: &str) -> (Vec<char>, HashMap<String, Loc>) {
    let mut lines = input.lines();
    let moves = lines.next().unwrap().chars().collect_vec();
    lines.next();
    let locs = Loc::parse(lines);
    (moves, locs)
}

#[derive(Debug)]
struct Loc(String, String);

impl Loc {
    fn parse(lines: Lines) -> HashMap<String, Loc> {
        lines
            .map(|line| {
                (
                    line[0..3].to_string(),
                    Loc(line[7..10].to_owned(), line[12..15].to_owned()),
                )
            })
            .collect()
    }

    fn nav(&self, d: char) -> &str {
        match d {
            'L' => &self.0,
            'R' => &self.1,
            _ => unreachable!("invalid direction: {}", d),
        }
    }
}

#[test]
fn part1() {
    let (moves, locs) = parse(&input!());

    let mut mv_cnt = 0usize;
    let mut cur = locs.get("AAA").unwrap();
    'main: loop {
        for d in moves.iter() {
            mv_cnt += 1;
            let dst = cur.nav(*d);
            if dst == "ZZZ" {
                break 'main;
            }
            cur = locs.get(dst).unwrap();
        }
    }
    dbg!(mv_cnt);
}

#[test]
fn part2() {
    let (moves, locs) = parse(&input!());

    let src = locs.keys().filter(|k| k.ends_with('A')).collect_vec();
    let cnt = src
        .iter()
        .map(|&s| part2_len(locs.get(s).unwrap(), &moves, &locs))
        .collect_vec();
    let ans = cnt.into_iter().reduce(lcm).unwrap();
    dbg!(ans);
}

fn part2_len<'a>(mut cur: &'a Loc, moves: &[char], locs: &'a HashMap<String, Loc>) -> usize {
    let mut mv_cnt = 0usize;
    'main: loop {
        for d in moves {
            mv_cnt += 1;
            let dst = cur.nav(*d);
            if dst.ends_with('Z') {
                break 'main;
            }
            cur = locs.get(dst).unwrap();
        }
    }
    mv_cnt
}
