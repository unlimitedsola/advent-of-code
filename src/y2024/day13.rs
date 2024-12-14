use indoc::indoc;
use num::rational::Ratio;
use regex::Regex;

use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve(&input!(), false));
}

#[derive(Debug)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    p: (i64, i64),
}

fn parse(input: &str) -> Vec<Machine> {
    let btn_re = Regex::new(r#"Button [AB]: X\+(\d+), Y\+(\d+)"#).unwrap();
    let prize_re = Regex::new(r#"Prize: X=(\d+), Y=(\d+)"#).unwrap();
    let mut lines = input.lines();
    let mut machines = Vec::new();
    while let Some(line) = lines.next() {
        let a = btn_re.captures(line).unwrap();
        let a = (a[1].parse::<i64>().unwrap(), a[2].parse::<i64>().unwrap());
        let b = lines.next().unwrap();
        let b = btn_re.captures(b).unwrap();
        let b = (b[1].parse::<i64>().unwrap(), b[2].parse::<i64>().unwrap());
        let p = lines.next().unwrap();
        let p = prize_re.captures(p).unwrap();
        let p = (p[1].parse::<i64>().unwrap(), p[2].parse::<i64>().unwrap());
        machines.push(Machine { a, b, p });
        lines.next();
    }
    machines
}

fn solve(input: &str, p2: bool) -> u64 {
    let machines = parse(input);
    let mut sum = 0;
    for machine in machines {
        let tokens = if p2 {
            solve_machine(machine, 10000000000000) as u64
        } else {
            solve_machine(machine, 0) as u64
        };
        sum += dbg!(tokens);
    }
    sum
}

fn solve_machine(machine: Machine, offset: i64) -> i64 {
    let Machine {
        a: (ax, ay),
        b: (bx, by),
        p: (px, py),
    } = machine;
    let (ax, ay, bx, by, px, py) = (
        Ratio::new(ax, 1),
        Ratio::new(ay, 1),
        Ratio::new(bx, 1),
        Ratio::new(by, 1),
        Ratio::new(px, 1),
        Ratio::new(py, 1),
    );
    let px = px + offset;
    let py = py + offset;
    // ax * a + bx * b = px
    // ay * a + by * b = py
    let a = (bx * py - px * by) / (bx * ay - ax * by);
    let b = (px * ay - ax * py) / (bx * ay - ax * by);
    if a.is_integer() && b.is_integer() {
        a.to_integer() * 3 + b.to_integer()
    } else {
        0
    }
}

#[test]
fn part2() {
    dbg!(solve(&input!(), true));
}

const EXAMPLE: &str = indoc! {"
    Button A: X+94, Y+34
    Button B: X+22, Y+67
    Prize: X=8400, Y=5400

    Button A: X+26, Y+66
    Button B: X+67, Y+21
    Prize: X=12748, Y=12176

    Button A: X+17, Y+86
    Button B: X+84, Y+37
    Prize: X=7870, Y=6450

    Button A: X+69, Y+23
    Button B: X+27, Y+71
    Prize: X=18641, Y=10279
"};

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE, false), 480);
    assert_eq!(solve(EXAMPLE, true), 875318608908);
}
