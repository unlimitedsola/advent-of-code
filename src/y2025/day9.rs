use indoc::indoc;
use itertools::Itertools;

use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve1(&input!()));
}

type V2 = (i64, i64);
type Line = (V2, V2);

fn parse(input: &str) -> Vec<V2> {
    input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec()
}

fn solve1(input: &str) -> u64 {
    let tiles = parse(input);
    tiles
        .into_iter()
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| {
            let dx = (x1 - x2).abs() + 1;
            let dy = (y1 - y2).abs() + 1;
            dx * dy
        })
        .max()
        .unwrap() as u64
}

#[test]
fn part2() {
    dbg!(solve2(&input!()));
}

fn side_of_line(l: Line, p: V2) -> i64 {
    let ((ax, ay), (bx, by)) = l;
    let (px, py) = p;
    // Compute the cross product of `(B-A)x(P-A)` to determine the side
    let v = (bx - ax) * (py - ay) - (by - ay) * (px - ax);
    v.signum()
}

fn point_on_line(l: Line, p: V2) -> bool {
    side_of_line(l, p) == 0 && {
        let ((ax, ay), (bx, by)) = l;
        let (px, py) = p;
        // Check if p is within the bounding box of the line segment
        (px >= ax.min(bx) && px <= ax.max(bx)) && (py >= ay.min(by) && py <= ay.max(by))
    }
}

fn do_lines_intersect(l1: Line, l2: Line) -> bool {
    let l1_a = side_of_line(l1, l2.0);
    let l1_b = side_of_line(l1, l2.1);
    let l2_a = side_of_line(l2, l1.0);
    let l2_b = side_of_line(l2, l1.1);
    // Lines intersect if the points of one line are on opposite sides of the other line
    // i.e., the signs of the side_of_line results are different.
    // We only consider proper intersections here (not touching at endpoints)
    (l1_a * l1_b < 0) && (l2_a * l2_b < 0)
}

fn point_inside(p: V2, lines: &[Line]) -> bool {
    // Ray-casting algorithm to determine if point is inside polygon
    // Cast a horizontal ray to the right from point p and count intersections with polygon edges
    // If the count is odd, the point is inside; if even, it's outside
    let mut inside = false;
    for &line in lines {
        if point_on_line(line, p) {
            return true; // On the boundary
        }
        let ((ax, ay), (bx, by)) = line;
        let (px, py) = p;
        let dx = bx - ax;
        let dy = by - ay;
        // Ignore lines that do not intersect with the ray to the right
        if ((ay > py) == (by > py)) || dy == 0 {
            continue;
        }
        // Compute the x-coordinate of the intersection of the line with the horizontal ray
        // `dx = bx - ax; dy = by - ay;`
        // Parametric form of line: `(x(t), y(t)) = (ax + dx*t, ay + dy*t)` for t in [0,1]
        // Compute `t` where `y(t) = py`: `py = ay + dy*t` => `t = (py - ay) / dy`
        // Then `x(t) = ax + dx*t`
        let intersect_x = ax + dx * (py - ay) / dy;
        if intersect_x > px {
            // Toggle if on the right side
            inside = !inside;
        }
    }
    inside
}

fn rect_inside(p1: V2, p2: V2, lines: &[Line]) -> bool {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let corners = [(x1, y1), (x1, y2), (x2, y1), (x2, y2)];
    if !corners.iter().all(|&corner| point_inside(corner, lines)) {
        return false;
    }
    let edges = [
        ((x1, y1), (x1, y2)),
        ((x1, y2), (x2, y2)),
        ((x2, y2), (x2, y1)),
        ((x2, y1), (x1, y1)),
    ];
    lines
        .iter()
        .all(|&line| edges.iter().all(|&edge| !do_lines_intersect(line, edge)))
}

fn solve2(input: &str) -> u64 {
    let tiles = parse(input);
    let lines: Vec<Line> = tiles.iter().copied().circular_tuple_windows().collect_vec();
    tiles
        .into_iter()
        .tuple_combinations()
        .filter(|&(p1, p2)| rect_inside(p1, p2, &lines))
        .map(|((x1, y1), (x2, y2))| {
            let dx = (x1 - x2).abs() + 1;
            let dy = (y1 - y2).abs() + 1;
            dx * dy
        })
        .max()
        .unwrap() as u64
}

const EXAMPLE: &str = indoc! {"
    7,1
    11,1
    11,7
    9,7
    9,5
    2,5
    2,3
    7,3
"};

#[test]
fn test_example() {
    assert_eq!(solve1(EXAMPLE), 50);
    assert_eq!(solve2(EXAMPLE), 24);
}
