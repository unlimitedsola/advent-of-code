use std::collections::HashMap;

use indoc::indoc;
use itertools::Itertools;
use rustworkx_core::petgraph::prelude::{DiGraphMap, GraphMap};

use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve1(&input!()));
}

fn parse(input: &str) -> DiGraphMap<&str, ()> {
    let mut graph = GraphMap::new();
    for line in input.lines() {
        let (node, neighbors) = line.split_once(": ").unwrap();
        let neighbors = neighbors.split_ascii_whitespace().collect_vec();
        graph.add_node(node);
        for neighbor in neighbors {
            graph.add_node(neighbor);
            graph.add_edge(node, neighbor, ());
        }
    }
    graph
}

fn solve1(input: &str) -> u64 {
    let graph = parse(input);
    let start = "you";
    let target = "out";
    fn dfs<'a>(
        graph: &DiGraphMap<&'a str, ()>,
        node: &'a str,
        target: &'a str,
        memo: &mut HashMap<&'a str, u64>,
    ) -> u64 {
        if node == target {
            return 1;
        }
        if let Some(&cached) = memo.get(node) {
            return cached;
        }
        let mut total = 0u64;
        for neighbor in graph.neighbors(node) {
            total += dfs(graph, neighbor, target, memo);
        }
        memo.insert(node, total);
        total
    }

    let mut memo: HashMap<&str, u64> = HashMap::new();
    dfs(&graph, start, target, &mut memo)
}

#[test]
fn part2() {
    dbg!(solve2(&input!()));
}

fn solve2(input: &str) -> u64 {
    let graph = parse(input);
    let start = "svr";
    let target = "out";
    fn dfs<'a>(
        graph: &DiGraphMap<&'a str, ()>,
        node: &'a str,
        target: &'a str,
        mut flags: u8,
        memo: &mut HashMap<(&'a str, u8), u64>,
    ) -> u64 {
        if node == "dac" {
            flags |= 1;
        } else if node == "fft" {
            flags |= 2;
        }
        if node == target {
            return if flags == 3 { 1 } else { 0 };
        }
        if let Some(&cached) = memo.get(&(node, flags)) {
            return cached;
        }
        let mut total = 0u64;
        for neighbor in graph.neighbors(node) {
            total += dfs(graph, neighbor, target, flags, memo);
        }
        memo.insert((node, flags), total);
        total
    }

    let mut memo: HashMap<(&str, u8), u64> = HashMap::new();
    dfs(&graph, start, target, 0, &mut memo)
}

const EXAMPLE: &str = indoc! {"
    aaa: you hhh
    you: bbb ccc
    bbb: ddd eee
    ccc: ddd eee fff
    ddd: ggg
    eee: out
    fff: out
    ggg: out
    hhh: ccc fff iii
    iii: out
"};

const EXAMPLE2: &str = indoc! {"
    svr: aaa bbb
    aaa: fft
    fft: ccc
    bbb: tty
    tty: ccc
    ccc: ddd eee
    ddd: hub
    hub: fff
    eee: dac
    dac: fff
    fff: ggg hhh
    ggg: out
    hhh: out
"};

#[test]
fn test_example() {
    assert_eq!(solve1(EXAMPLE), 5);
    assert_eq!(solve2(EXAMPLE2), 2);
}
