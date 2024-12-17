use grid::Grid;
use indoc::indoc;
use itertools::Itertools;
use num::pow;
use pathfinding::prelude::{astar_bag, dijkstra};
use std::collections::HashSet;
use std::io::Write;

use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve1(&input!()));
}

fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut reg = vec![];
    let mut lines = input.lines();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        reg.push(
            line.split_ascii_whitespace()
                .last()
                .unwrap()
                .parse::<u64>()
                .unwrap(),
        );
    }
    let ins = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();
    (reg, ins)
}

fn solve1(input: &str) -> String {
    let (mut reg, ins) = parse(input);
    let output = exec_prg(&mut reg, &ins);
    output.into_iter().join(",")
}

fn exec_prg(reg: &mut [u64], ins: &[u64]) -> Vec<u64> {
    let mut output = vec![];
    let mut pc = 0usize;
    while pc < ins.len() {
        match exec_ins(reg, ins[pc], ins[pc + 1]) {
            ExecResult::Jump(j) => pc = j,
            ExecResult::Output(o) => {
                output.push(o);
                pc += 2;
            }
            ExecResult::None => {
                pc += 2;
            }
        }
    }
    output
}

enum ExecResult {
    None,
    Jump(usize),
    Output(u64),
}

fn exec_ins(reg: &mut [u64], opcode: u64, operand: u64) -> ExecResult {
    match opcode {
        0 => reg[0] = reg[0] >> combo_operand(reg, operand),
        1 => reg[1] ^= operand,
        2 => reg[1] = combo_operand(reg, operand) % 8,
        3 => {
            if reg[0] != 0 {
                return ExecResult::Jump(operand as usize);
            }
        }
        4 => reg[1] = reg[1] ^ reg[2],
        5 => return ExecResult::Output(combo_operand(reg, operand) % 8),
        6 => reg[1] = reg[0] >> combo_operand(reg, operand),
        7 => reg[2] = reg[0] >> combo_operand(reg, operand),
        _ => unreachable!("Invalid opcode: {opcode}"),
    }
    ExecResult::None
}

fn combo_operand(reg: &[u64], operand: u64) -> u64 {
    match operand {
        ..4 => operand,
        4..7 => reg[(operand - 4) as usize],
        _ => unreachable!(),
    }
}
fn fast_exec(mut a: u64) -> Vec<u64> {
    let mut output = vec![];
    while a != 0 {
        let (a1, o) = step(a);
        output.push(o);
        a = a1;
    }
    output
}

// Program: 2,4,1,1,7,5,4,7,1,4,0,3,5,5,3,0
// b = a & 7;  // 2,4; bst a
// b ^= 1;     // 1,1; bxl 1
// c = a >> b; // 7,5; cdv b
// b ^= c;     // 4,7; bxc _
// b ^= 4;     // 1,4; bxl 4
// a >>= 3;    // 0,3; adv 3
// b & 7       // 5,5; out b
//             // 3,0; jmp 0 (loop)
fn step(mut a: u64) -> (u64, u64) {
    let mut b = a & 7; // 2,4; bst a
    b ^= 1; // 1,1; bxl 1
    let c = a >> b; // 7,5; cdv b
    b ^= c; // 4,7; bxc _
    b ^= 4; // 1,4; bxl 4
    a >>= 3; // 0,3; adv 3
    (a, b & 7) // 5,5; out b
}

#[test]
fn part2() {
    dbg!(solve2(&input!()));
}

fn solve2(input: &str) -> u64 {
    let (_, ins) = parse(input);
    let mut a = 1u64;
    loop {
        let out = fast_exec(a);
        if ins.ends_with(&out) {
            if ins == out {
                return a;
            } else {
                a <<= 3;
            }
        } else {
            while a & 0b111 == 0b111 {
                a >>= 3;
            }
            a += 1;
        }
    }
}

const EXAMPLE: &str = indoc! {"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"};

const EXAMPLE2: &str = indoc! {"
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"};

#[test]
fn test_example() {
    assert_eq!(solve1(EXAMPLE), "4,6,3,5,6,3,5,2,1,0");
    assert_eq!(solve2(EXAMPLE2), 117440);
}
