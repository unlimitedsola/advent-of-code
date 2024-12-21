use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use std::hash::Hash;

use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve(&input!(), 2));
}

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_owned()).collect_vec()
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Pad {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
    A = 4,
}

impl Pad {
    fn values() -> &'static [Pad] {
        use Pad::*;
        &[Up, Down, Left, Right, A]
    }

    fn adj(self) -> &'static [(Pad, Pad)] {
        use Pad::*;
        match self {
            Up => &[(Down, Down), (A, Right)],
            Down => &[(Left, Left), (Up, Up), (Right, Right)],
            Left => &[(Down, Right)],
            Right => &[(Down, Left), (A, Up)],
            A => &[(Up, Left), (Right, Down)],
        }
    }
}

type PadMap = [[u64; 5]; 5];

fn pad_dist(map: PadMap) -> PadMap {
    let mut new_map = [[0; 5]; 5];
    for &s in Pad::values() {
        for &e in Pad::values() {
            new_map[s as usize][e as usize] = cost(s, e, Pad::adj, &map);
        }
    }
    new_map
}

fn cost<N, FN>(s: N, e: N, mut adj: FN, pad: &PadMap) -> u64
where
    N: Copy + Eq + Hash + 'static,
    FN: FnMut(N) -> &'static [(N, Pad)],
{
    if s == e {
        return 1;
    }
    let path = dijkstra(
        &(s, Pad::A),
        |&(prev, prev_pad)| {
            if prev == e {
                return vec![((prev, Pad::A), pad[prev_pad as usize][Pad::A as usize])];
            }
            adj(prev)
                .iter()
                .map(|&(n, p)| ((n, p), pad[prev_pad as usize][p as usize]))
                .collect_vec()
        },
        |&p| p == (e, Pad::A),
    )
    .unwrap();
    path.1
}

fn solve(input: &str, pads: u64) -> u64 {
    let codes = parse(input);
    let mut pad = [[1u64; 5]; 5];
    for _ in 0..pads {
        pad = pad_dist(pad);
        for p in pad {
            println!("{:?}", p);
        }
        println!()
    }
    let mut sum = 0u64;
    for code in codes {
        let code_num = code[..3].parse::<u64>().unwrap();
        let a_code = String::from("A") + &code;
        let len = solve_code(&a_code, &pad);
        dbg!(len);
        sum += code_num * len;
    }
    sum
}

fn solve_code(code: &str, pad: &PadMap) -> u64 {
    code.chars()
        .tuple_windows()
        .map(|(a, b)| {
            let cost = cost(a, b, numpad_adj, pad);
            println!("{} -> {} = {}", a, b, cost);
            cost
        })
        .sum()
}

fn numpad_adj(a: char) -> &'static [(char, Pad)] {
    // 789
    // 456
    // 123
    // _0A
    use Pad::*;
    match a {
        '0' => &[('2', Up), ('A', Right)],
        '1' => &[('2', Right), ('4', Up)],
        '2' => &[('1', Left), ('3', Right), ('5', Up), ('0', Down)],
        '3' => &[('2', Left), ('6', Up), ('A', Down)],
        '4' => &[('1', Down), ('5', Right), ('7', Up)],
        '5' => &[('2', Down), ('4', Left), ('6', Right), ('8', Up)],
        '6' => &[('3', Down), ('5', Left), ('9', Up)],
        '7' => &[('4', Down), ('8', Right)],
        '8' => &[('5', Down), ('7', Left), ('9', Right)],
        '9' => &[('6', Down), ('8', Left)],
        'A' => &[('0', Left), ('3', Up)],
        _ => unreachable!(),
    }
}

#[test]
fn part2() {
    dbg!(solve(&input!(), 25));
}

#[test]
fn test_example() {
    assert_eq!(solve("029A", 2), 68 * 29);
    assert_eq!(solve("179A", 2), 68 * 179);
    assert_eq!(solve("456A", 2), 64 * 456);
    assert_eq!(solve("379A", 2), 64 * 379);
    // A->9: <v<A>>^AAAvA^A
    // 9->8: <vA<AA>>^AvAA<^A>A
    assert_eq!(solve("980A", 2), 60 * 980);
}
