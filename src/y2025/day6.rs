use grid::Grid;
use indoc::indoc;
use itertools::Itertools;

use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve1(&input!()));
}

fn parse(input: &str) -> (Grid<u64>, Vec<char>) {
    let mut lines = input.lines().collect_vec();
    let operations = lines
        .pop()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.chars().exactly_one().unwrap())
        .collect_vec();
    let numbers: Grid<u64> = lines
        .into_iter()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect_vec()
        })
        .collect_vec()
        .into();
    (numbers, operations)
}

fn solve1(input: &str) -> u64 {
    let (mut numbers, operations) = parse(input);
    numbers.transpose();
    let mut total = 0u64;
    for (i, nums) in numbers.iter_rows().enumerate() {
        let op = operations[i];
        match op {
            '+' => {
                let sum: u64 = nums.sum();
                total += sum;
            }
            '*' => {
                let product: u64 = nums.product();
                total += product;
            }
            _ => panic!("Unknown operation"),
        }
    }
    total
}

fn parse2(input: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    let mut lines = input.lines().collect_vec();
    let operations = lines
        .pop()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.chars().exactly_one().unwrap())
        .collect_vec();
    let mut grid: Grid<char> = lines
        .into_iter()
        .map(|l| l.chars().collect_vec())
        .collect_vec()
        .into();
    grid.transpose();
    let mut nums = vec![];
    let mut temp = vec![];
    for row in grid.iter_rows() {
        let str = row.into_iter().collect::<String>();
        let trimmed = str.trim();
        if trimmed.is_empty() {
            nums.push(temp);
            temp = vec![];
            continue;
        }
        let num: u64 = trimmed.parse().unwrap();
        temp.push(num);
    }
    nums.push(temp);
    (nums, operations)
}

#[test]
fn part2() {
    dbg!(solve2(&input!()));
}

fn solve2(input: &str) -> u64 {
    let (numbers, operations) = parse2(input);
    let mut total = 0u64;
    for (i, nums) in numbers.into_iter().enumerate() {
        let op = operations[i];
        match op {
            '+' => {
                let sum: u64 = nums.into_iter().sum();
                total += sum;
            }
            '*' => {
                let product: u64 = nums.into_iter().product();
                total += product;
            }
            _ => panic!("Unknown operation"),
        }
    }
    total
}

const EXAMPLE: &str = indoc! {"
    123 328  51 64 
     45 64  387 23 
      6 98  215 314
    *   +   *   +  
"};

#[test]
fn test_example() {
    assert_eq!(solve1(EXAMPLE), 4277556);
    assert_eq!(solve2(EXAMPLE), 3263827);
}
