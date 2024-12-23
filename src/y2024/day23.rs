use std::collections::HashSet;

use indoc::indoc;
use itertools::Itertools;
use petgraph::graphmap::UnGraphMap;
use rustworkx_core::connectivity::connected_components;

use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve1(&input!()));
}

fn parse_name(name: &str) -> [u8; 2] {
    name.bytes().collect_vec().try_into().unwrap()
}

fn parse(input: &str) -> UnGraphMap<[u8; 2], ()> {
    let mut g = UnGraphMap::new();
    for line in input.lines() {
        let (a, b) = line.split('-').map(parse_name).collect_tuple().unwrap();
        g.add_edge(a, b, ());
    }
    g
}

fn solve1(input: &str) -> u64 {
    let g = parse(input);
    let components = connected_components(&g);
    let mut sum = 0;
    for set in components {
        for nodes in set.iter().copied().combinations(3) {
            let contain_t = nodes.iter().copied().any(|n| n[0] == b't');
            let interconnect = nodes
                .iter()
                .copied()
                .combinations(2)
                .all(|pair| g.contains_edge(pair[0], pair[1]));
            if contain_t && interconnect {
                sum += 1;
            }
        }
    }
    sum
}

#[test]
fn part2() {
    dbg!(solve2(&input!()));
}

fn solve2(input: &str) -> String {
    let g = parse(input);
    let mut net = g
        .nodes()
        .map(|n| {
            let mut set = HashSet::new();
            set.insert(n);
            set
        })
        .collect_vec();

    for net in net.iter_mut() {
        for n in g.nodes() {
            if net.iter().copied().all(|m| g.contains_edge(n, m)) {
                net.insert(n);
            }
        }
    }

    let mut max = net
        .iter()
        .max_by_key(|n| n.len())
        .unwrap()
        .iter()
        .copied()
        .map(|s| String::from_utf8(s.to_vec()).unwrap())
        .collect_vec();
    max.sort();

    max.join(",")
}

const EXAMPLE: &str = indoc! {"
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"};

#[test]
fn test_example() {
    assert_eq!(solve1(EXAMPLE), 7);
    assert_eq!(solve2(EXAMPLE), "co,de,ka,ta");
}
