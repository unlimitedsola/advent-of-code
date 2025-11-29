use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve1(&input!()));
}

fn parse(input: &str) -> Vec<PasswordLine<'_>> {
    input.lines().map(parse_line).collect()
}

type PasswordLine<'a> = (u64, u64, char, &'a str);

fn parse_line(line: &str) -> PasswordLine<'_> {
    let (policy, passwd) = line.split_once(": ").unwrap();
    let (range, letter) = policy.split_once(' ').unwrap();
    let (min, max) = range.split_once('-').unwrap();
    (
        min.parse().unwrap(),
        max.parse().unwrap(),
        letter.chars().next().unwrap(),
        passwd,
    )
}

fn solve1(input: &str) -> u64 {
    let lines = parse(input);
    let mut valid = 0;
    for (min, max, letter, passwd) in lines {
        let count = passwd.chars().filter(|c| *c == letter).count() as u64;
        if count >= min && count <= max {
            valid += 1;
        }
    }
    valid
}

#[test]
fn part2() {
    dbg!(solve2(&input!()));
}

fn solve2(input: &str) -> u64 {
    let lines = parse(input);
    let mut valid = 0;
    for (pos1, pos2, letter, passwd) in lines {
        let chars: Vec<char> = passwd.chars().collect();
        let first = chars.get((pos1 - 1) as usize) == Some(&letter);
        let second = chars.get((pos2 - 1) as usize) == Some(&letter);
        if first ^ second {
            valid += 1;
        }
    }
    valid
}
