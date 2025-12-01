use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve1(&input!()));
}

fn parse(input: &str) -> Vec<(char, i64)> {
    input
        .lines()
        .map(|l| {
            let dir = l.chars().next().unwrap();
            let dist = l[1..].parse().unwrap();
            (dir, dist)
        })
        .collect()
}

fn solve1(input: &str) -> u64 {
    let lines = parse(input);
    let mut zeros = 0;
    let mut pos = 50i64;
    for (dir, dist) in lines {
        if dir == 'L' {
            pos -= dist;
        } else if dir == 'R' {
            pos += dist;
        }
        while pos < 0 {
            pos += 100;
        }
        while pos >= 100 {
            pos -= 100;
        }
        if pos == 0 {
            zeros += 1;
        }
    }
    zeros
}

#[test]
fn part2() {
    dbg!(solve2(&input!()));
}

fn solve2(input: &str) -> u64 {
    let lines = parse(input);
    let mut zeros = 0;
    let mut pos = 50i64;
    for (dir, dist) in lines {
        let start_pos = pos;
        if dir == 'L' {
            pos -= dist;
        } else if dir == 'R' {
            pos += dist;
        }
        if start_pos == 0 && pos < 0 {
            // special case: starting at 0 and going left
            zeros -= 1;
        }
        while pos < 0 {
            pos += 100;
            zeros += 1;
        }
        if pos == 0 && start_pos > 0 {
            // special case: ending at 0 and coming from the right
            zeros += 1;
        }
        while pos >= 100 {
            pos -= 100;
            zeros += 1;
        }
    }
    zeros
}

const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

#[test]
fn test_example() {
    assert_eq!(solve1(TEST_INPUT), 3);
}

#[test]
fn test_example2() {
    assert_eq!(solve2(TEST_INPUT), 6);
}

#[test]
fn test_example3() {
    assert_eq!(solve2("R1000"), 10);
}
