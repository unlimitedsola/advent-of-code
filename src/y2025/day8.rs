use indoc::indoc;
use itertools::Itertools;
use rustworkx_core::{
    connectivity::{connected_components, number_connected_components},
    petgraph::prelude::UnGraphMap,
};

use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve1(&input!(), 1000));
}

type V3 = (i64, i64, i64);

fn parse(input: &str) -> UnGraphMap<V3, ()> {
    let mut graph = UnGraphMap::<V3, ()>::new();
    input.lines().for_each(|l| {
        let node = l
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect_tuple()
            .unwrap();
        graph.add_node(node);
    });
    graph
}

fn solve1(input: &str, edge_count: usize) -> u64 {
    let mut graph = parse(input);

    let edges = edges(&graph);
    for edge in edges.iter().take(edge_count) {
        graph.add_edge((edge.1).0, (edge.1).1, ());
    }

    let components = connected_components(&graph);
    let mut lens = components.iter().map(|c| c.len()).collect_vec();
    lens.sort_by(|a, b| b.cmp(a));
    lens.iter().take(3).map(|&x| x as u64).product()
}

fn edges(graph: &UnGraphMap<V3, ()>) -> Vec<(i64, (V3, V3))> {
    graph
        .nodes()
        .tuple_combinations()
        .map(|(a, b)| (dist((a, b)), (a, b)))
        .sorted_by(|(da, _), (db, _)| da.cmp(db))
        .collect()
}

fn dist((a, b): (V3, V3)) -> i64 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)
}

#[test]
fn part2() {
    dbg!(solve2(&input!()));
}

fn solve2(input: &str) -> u64 {
    let mut graph = parse(input);

    let edges = edges(&graph);
    let mut edges = edges.into_iter();
    loop {
        let edge = edges.next().unwrap();
        graph.add_edge((edge.1).0, (edge.1).1, ());
        if number_connected_components(&graph) == 1 {
            return edge.1.0.0 as u64 * edge.1.1.0 as u64;
        }
    }
}

const EXAMPLE: &str = indoc! {"
    162,817,812
    57,618,57
    906,360,560
    592,479,940
    352,342,300
    466,668,158
    542,29,236
    431,825,988
    739,650,466
    52,470,668
    216,146,977
    819,987,18
    117,168,530
    805,96,715
    346,949,466
    970,615,88
    941,993,340
    862,61,35
    984,92,344
    425,690,689
"};

#[test]
fn test_example() {
    assert_eq!(solve1(EXAMPLE, 10), 40);
    assert_eq!(solve2(EXAMPLE), 25272);
}
