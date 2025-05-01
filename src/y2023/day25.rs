use indoc::indoc;
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::prelude::UnGraphMap;

use crate::aoc::input;

#[test]
fn part1() {
    let input = input!();
    let g = parse(&input);
    dbg!(solve(&g));
}

type G<'a> = UnGraphMap<&'a str, ()>;

fn parse(input: &str) -> G {
    UnGraphMap::from_edges(input.lines().flat_map(|line| {
        let (src, dst) = line.split_once(": ").unwrap();
        dst.split_ascii_whitespace().map(move |dst| (src, dst))
    }))
}

fn solve(g: &G) -> usize {
    let (cut, partition) = stoer_wagner_min_cut(g, |_| anyhow::Ok(1)).unwrap().unwrap();
    assert_eq!(cut, 3);
    partition.len() * (g.node_count() - partition.len())
}

const EXAMPLE: &str = indoc! {"
    jqt: rhn xhk nvd
    rsh: frs pzl lsr
    xhk: hfx
    cmg: qnr nvd lhk bvb
    rhn: xhk bvb hfx
    bvb: xhk hfx
    pzl: lsr hfx nvd
    qnr: nvd
    ntq: jqt hfx bvb xhk
    nvd: lhk
    lsr: lhk
    rzs: qnr cmg lsr rsh
    frs: qnr lhk lsr
"};

#[test]
fn test_example() {
    let g = parse(EXAMPLE);
    assert_eq!(solve(&g), 54);
}
