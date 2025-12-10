use std::collections::{HashSet, VecDeque};

use indoc::indoc;
use itertools::Itertools;
use z3::ast::Int;

use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve1(&input!()));
}

fn parse(input: &str) -> Vec<(u32, Vec<u32>, Vec<u32>)> {
    input.lines().map(parse_line).collect_vec()
}

fn parse_line(line: &str) -> (u32, Vec<u32>, Vec<u32>) {
    let mut parts = line.split_ascii_whitespace();
    let lights = parts.next().unwrap();
    let lights = lights.strip_prefix('[').unwrap().strip_suffix(']').unwrap();
    let mut lights_num = 0u32;
    for c in lights.chars().rev() {
        lights_num <<= 1;
        if c == '#' {
            lights_num |= 1;
        }
    }
    let mut rem = parts.collect_vec();
    let joltages = rem.pop().unwrap();
    let joltages = joltages
        .strip_prefix('{')
        .unwrap()
        .strip_suffix('}')
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect_vec();
    let mut buttons = vec![];
    for b in rem {
        let b = b
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect_vec();
        let mut b_num = 0u32;
        for n in b {
            b_num |= 1 << n;
        }
        buttons.push(b_num);
    }
    (lights_num, buttons, joltages)
}

fn solve1(input: &str) -> u64 {
    let cases = parse(input);
    let mut sum = 0;
    for case in cases {
        sum += solve_line1(case) as u64;
    }
    sum
}

fn solve_line1((lights, buttons, _): (u32, Vec<u32>, Vec<u32>)) -> u32 {
    if lights == 0 {
        return 0;
    }
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    visited.insert(0u32);
    for &btn in &buttons {
        if btn == lights {
            return 1;
        }
        visited.insert(btn);
        queue.push_back((btn, 1));
    }
    while let Some((state, cost)) = queue.pop_front() {
        for &btn in &buttons {
            let new_state = state ^ btn;
            if new_state == lights {
                return cost + 1;
            }
            if !visited.contains(&new_state) {
                visited.insert(new_state);
                queue.push_back((new_state, cost + 1));
            }
        }
    }
    unreachable!()
}

#[test]
fn part2() {
    dbg!(solve2(&input!()));
}

fn solve2(input: &str) -> u64 {
    let cases = parse(input);
    let mut sum = 0;
    for case in cases {
        sum += solve_line2(case) as u64;
    }
    sum
}

fn solve_line2((_, buttons, joltages): (u32, Vec<u32>, Vec<u32>)) -> u32 {
    let ctx = z3::Optimize::new();
    let presses = (0..buttons.len())
        .map(|_| Int::fresh_const("p"))
        .inspect(|x| ctx.assert(&x.ge(0)))
        .collect::<Vec<_>>();
    for (i, &j) in joltages.iter().enumerate() {
        let sat = &buttons
            .iter()
            .enumerate()
            .filter(|&(_, &b)| b & (1 << i) != 0)
            .map(|(x, _)| presses[x].clone())
            .reduce(|a, b| a + b)
            .unwrap()
            .eq(j);
        ctx.assert(sat);
    }
    let sum = presses.iter().cloned().reduce(|a, b| a + b).unwrap();
    ctx.minimize(&sum);
    assert_eq!(ctx.check(&[]), z3::SatResult::Sat);
    let model = ctx.get_model().unwrap();
    model.eval(&sum, true).unwrap().as_u64().unwrap() as u32
}

const EXAMPLE: &str = indoc! {"
    [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"};

#[test]
fn test_example() {
    assert_eq!(solve1(EXAMPLE), 7);
    assert_eq!(solve2(EXAMPLE), 33);
}
