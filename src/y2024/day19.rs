use indoc::indoc;
use itertools::Itertools;
use pathfinding::prelude::count_paths;

use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve1(&input!()));
}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = input.lines();
    let patterns = lines.next().unwrap();
    let patterns = patterns.split(", ").collect_vec();
    lines.next().unwrap();
    let designs = lines.collect_vec();
    (patterns, designs)
}

fn solve1(input: &str) -> u64 {
    let (patterns, designs) = parse(input);
    let mut sum = 0;
    for design in designs {
        if count(design, &patterns) != 0 {
            sum += 1;
        }
    }
    sum
}

fn count(design: &str, patterns: &[&str]) -> usize {
    count_paths(
        design,
        |prev| {
            patterns
                .iter()
                .filter(|&pattern| prev.starts_with(pattern))
                .map(|&pattern| &prev[pattern.len()..])
        },
        |s| s.is_empty(),
    )
}

#[test]
fn part2() {
    dbg!(solve2(&input!()));
}

fn solve2(input: &str) -> u64 {
    let (patterns, designs) = parse(input);
    let mut sum = 0;
    for design in designs {
        let paths = count(design, &patterns);
        sum += paths as u64;
    }
    sum
}

const EXAMPLE: &str = indoc! {"
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"};

#[test]
fn test_example() {
    assert_eq!(solve1(EXAMPLE), 6);
    assert_eq!(solve2(EXAMPLE), 16);
}
