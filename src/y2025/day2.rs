use indoc::indoc;
use itertools::Itertools;

use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve1(&input!()));
}

fn parse(input: &str) -> Vec<(i64, i64)> {
    let line = input.lines().next().unwrap();
    line.split(',')
        .map(|s| {
            let (start, end) = s.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect_vec()
}

fn solve1(input: &str) -> u64 {
    let ranges = parse(input);
    let mut sum = 0i64;
    for (start, end) in ranges {
        for i in start..=end {
            let i_str = i.to_string();
            let (l, r) = i_str.split_at(i_str.len() / 2);
            if l == r {
                sum += i;
            }
        }
    }
    sum as u64
}

fn is_valid(i: i64) -> bool {
    let s = i.to_string();
    let len = s.len();
    'outer: for la in 1..=s.len() / 2 {
        if !len.is_multiple_of(la) {
            continue;
        }
        let mut j = la;
        let first = &s[0..la];
        while j + la <= len {
            if &s[j..j + la] != first {
                continue 'outer;
            }
            j += la;
        }
        return true;
    }
    false
}

#[test]
fn part2() {
    dbg!(solve2(&input!()));
}

fn solve2(input: &str) -> u64 {
    let ranges = parse(input);
    let mut sum = 0i64;
    for (start, end) in ranges {
        for i in start..=end {
            if is_valid(i) {
                sum += i;
            }
        }
    }
    sum as u64
}

const EXAMPLE: &str = indoc! {"
   11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
"};

#[test]
fn test_example() {
    assert_eq!(solve1(EXAMPLE), 1227775554);
    assert_eq!(solve2(EXAMPLE), 4174379265);
}
