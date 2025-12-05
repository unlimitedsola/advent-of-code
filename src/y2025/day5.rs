use indoc::indoc;
use itertools::Itertools;

use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve1(&input!()));
}

type Range = (u64, u64);

fn parse(input: &str) -> (Vec<Range>, Vec<u64>) {
    let mut lines = input.lines();
    let ranges = lines
        .by_ref()
        .take_while(|c| !c.is_empty())
        .map(|l| {
            let (a, b) = l.split_once('-').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect_vec();
    let nums = lines.by_ref().map(|l| l.parse().unwrap()).collect_vec();
    (ranges, nums)
}

fn solve1(input: &str) -> u64 {
    let (ranges, nums) = parse(input);
    let mut valid = 0;
    for n in nums {
        for &(a, b) in &ranges {
            if n >= a && n <= b {
                valid += 1;
                break;
            }
        }
    }
    valid
}

#[test]
fn part2() {
    dbg!(solve2(&input!()));
}

fn solve2(input: &str) -> u64 {
    let (mut ranges, _) = parse(input);
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut merged = vec![];

    let mut iter = ranges.into_iter();
    if let Some(first) = iter.next() {
        merged.push(first);
        for (start, end) in iter {
            let (_, l_end) = merged.last_mut().unwrap();
            if start <= *l_end {
                if end > *l_end {
                    *l_end = end;
                }
            } else {
                merged.push((start, end));
            }
        }
    }

    let mut total = 0u64;
    for r in merged {
        total += r.1 - r.0 + 1;
    }
    total
}

const EXAMPLE: &str = indoc! {"
    3-5
    10-14
    16-20
    12-18

    1
    5
    8
    11
    17
    32
"};

#[test]
fn test_example() {
    assert_eq!(solve1(EXAMPLE), 3);
    assert_eq!(solve2(EXAMPLE), 14);
}
