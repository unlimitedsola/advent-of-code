use indoc::indoc;

use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve1(&input!()));
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn solve1(input: &str) -> u64 {
    let lines = parse(input);
    let mut sum = 0;
    for line in lines {
        let jotalge = max1(line);
        sum += jotalge;
    }
    sum
}

fn max1(input: &str) -> u64 {
    let mut chars = input.chars().enumerate().collect::<Vec<_>>();
    chars.sort_by(|a, b| b.1.cmp(&a.1));
    let mut max_char = chars[0];
    if max_char.0 == chars.len() - 1 {
        max_char = chars[1];
    }
    let sec_char = chars
        .iter()
        .filter(|(i, _)| *i > max_char.0)
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    format!("{}{}", max_char.1, sec_char.1).parse().unwrap()
}

#[test]
fn part2() {
    dbg!(solve2(&input!()));
}

fn solve2(input: &str) -> u64 {
    let lines = parse(input);
    let mut sum = 0;
    for line in lines {
        let jotalge = max2(line);
        sum += jotalge;
    }
    sum
}

fn max2(input: &str) -> u64 {
    let chars = input.chars().collect::<Vec<_>>();
    let mut chosen = vec![];
    let mut start = 0;
    while chosen.len() < 12 {
        let end = chars.len() - (12 - chosen.len());
        let mut idx = 0;
        let mut ch = '0';
        for (i, c) in chars[start..=end].iter().enumerate() {
            if *c > ch {
                idx = i;
                ch = *c;
            }
        }
        chosen.push(ch);
        start += idx + 1;
    }
    chosen.iter().collect::<String>().parse().unwrap()
}

const EXAMPLE: &str = indoc! {"
    987654321111111
    811111111111119
    234234234234278
    818181911112111
"};

#[test]
fn test_example() {
    assert_eq!(solve1(EXAMPLE), 357);
    assert_eq!(solve2(EXAMPLE), 3121910778619);
}
