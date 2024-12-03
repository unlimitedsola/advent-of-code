use regex::Regex;

use crate::aoc::input;

#[test]
fn part1() {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let input = input!();
    let sum = re
        .captures_iter(&input)
        .map(|cap| cap[1].parse::<u64>().unwrap() * cap[2].parse::<u64>().unwrap())
        .sum::<u64>();
    dbg!(sum);
}

#[test]
fn part2() {
    let re = Regex::new(r"don't\(\)|do\(\)|mul\((\d+),(\d+)\)").unwrap();
    let input = input!();
    let mut enable = true;
    let sum = re
        .captures_iter(&input)
        .map(|cap| {
            match &cap[0] {
                "do()" => enable = true,
                "don't()" => enable = false,
                _ if enable => {
                    return cap[1].parse::<u64>().unwrap() * cap[2].parse::<u64>().unwrap()
                }
                _ => (),
            }
            0
        })
        .sum::<u64>();
    dbg!(sum);
}
