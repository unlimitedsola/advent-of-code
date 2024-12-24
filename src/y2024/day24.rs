use std::collections::{HashMap, HashSet};

use indoc::indoc;
use itertools::Itertools;

use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve1(&input!()));
}

struct Wire {
    a: String,
    b: String,
    op: String,
    out: String,
}

fn parse(input: &str) -> (HashMap<String, bool>, Vec<Wire>) {
    let mut lines = input.lines();
    let mut regs = HashMap::new();
    for l in lines.by_ref() {
        if l.is_empty() {
            break;
        }
        let (reg, val) = l.split_once(": ").unwrap();
        regs.insert(reg.to_string(), val.parse::<u8>().unwrap() == 1);
    }

    let mut wires = vec![];

    for l in lines {
        let (rest, out) = l.split_once(" -> ").unwrap();
        let [a, op, b] = rest.split(" ").collect_vec().try_into().unwrap();
        wires.push(Wire {
            a: a.to_string(),
            b: b.to_string(),
            op: op.to_string(),
            out: out.to_string(),
        });
    }
    (regs, wires)
}

fn solve1(input: &str) -> u64 {
    let (mut regs, mut wires) = parse(input);
    while !wires.is_empty() {
        let mut i = 0;
        while i < wires.len() {
            let wire = &wires[i];
            if regs.contains_key(&wire.a) && regs.contains_key(&wire.b) {
                let wire = wires.swap_remove(i);
                let a = regs[&wire.a];
                let b = regs[&wire.b];
                let out = match wire.op.as_str() {
                    "AND" => a & b,
                    "OR" => a | b,
                    "XOR" => a ^ b,
                    _ => unreachable!(),
                };
                regs.insert(wire.out.clone(), out);
            } else {
                i += 1;
            }
        }
    }
    let k = regs
        .iter()
        .filter(|(k, _)| k.starts_with("z"))
        .map(|(k, &v)| (k.clone(), v))
        .collect_vec();
    let mut n = 0u64;
    for (k, v) in k {
        let i = k.strip_prefix("z").unwrap().parse::<u64>().unwrap();
        n |= (v as u64) << i;
    }
    n
}

#[test]
fn part2() {
    let (regs, wires) = parse(&input!());

    // manual inspection in generated graphviz

    println!("digraph {{");

    for wire in wires {
        let color = match wire.op.as_str() {
            "AND" => "blue",
            "OR" => "green",
            "XOR" => "red",
            _ => unreachable!(),
        };
        println!(r#"    {} [color="{color}"];"#, wire.out);
        println!(r#"    {} -> {};"#, wire.a, wire.out);
        println!(r#"    {} -> {};"#, wire.b, wire.out);
    }

    println!("}}")
}

const EXAMPLE: &str = indoc! {"
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
"};

#[test]
fn test_example() {
    assert_eq!(solve1(EXAMPLE), 2024);
    // assert_eq!(solve2(EXAMPLE), "co,de,ka,ta");
}
