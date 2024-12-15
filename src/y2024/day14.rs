use grid::Grid;
use indoc::indoc;
use itertools::Itertools;
use regex::Regex;

use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve(&input!(), (101, 103)));
}

type Robot = [i64; 4];

fn parse(input: &str) -> Vec<Robot> {
    let re = Regex::new(r#"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)"#).unwrap();
    input
        .lines()
        .map(|l| {
            re.captures(l)
                .unwrap()
                .iter()
                .skip(1)
                .map(|c| c.unwrap().as_str().parse::<i64>().unwrap())
                .collect_vec()
                .try_into()
                .unwrap()
        })
        .collect_vec()
}

fn solve(input: &str, (gx, gy): (i64, i64)) -> u64 {
    let robots = parse(input);
    let [mx, my] = [gx / 2, gy / 2];
    let mut quadrants = [0u64; 4]; // [top-left, top-right, bottom-left, bottom-right]
    for robot in robots {
        let [x, y] = tick(robot, 100, (gx, gy));
        if x < mx && y < my {
            quadrants[0] += 1;
        }
        if x > mx && y < my {
            quadrants[1] += 1;
        }
        if x < mx && y > my {
            quadrants[2] += 1;
        }
        if x > mx && y > my {
            quadrants[3] += 1;
        }
        println!("{:?}", [x, y]);
    }
    println!("{:?}", quadrants);
    quadrants.iter().product()
}

fn tick(robot: Robot, time: i64, (gx, gy): (i64, i64)) -> [i64; 2] {
    let [px, py, vx, vy] = robot;
    let mut px = (px + vx * time) % gx;
    if px < 0 {
        px += gx;
    }
    let mut py = (py + vy * time) % gy;
    if py < 0 {
        py += gy;
    }
    [px, py]
}

#[test]
fn part2() {
    dbg!(solve2(&input!(), (101, 103)));
}

fn solve2(input: &str, (gx, gy): (i64, i64)) -> u64 {
    let robots = parse(input);
    let mut time = 0i64;
    loop {
        let loc = robots
            .iter()
            .map(|r| tick(*r, time, (gx, gy)))
            .collect_vec();
        if loc.iter().all_unique() {
            let mut grid = Grid::init(gx as usize, gy as usize, ' ');
            for l in loc {
                let [x, y] = l.map(|x| x as usize);
                grid[(x, y)] = '#'
            }
            for r in grid.iter_rows() {
                for c in r {
                    print!("{}", c);
                }
                println!();
            }
            println!();
            dbg!(time);
            break;
        }
        time += 1;
    }
    time as u64
}

const EXAMPLE: &str = indoc! {"
    p=0,4 v=3,-3
    p=6,3 v=-1,-3
    p=10,3 v=-1,2
    p=2,0 v=2,-1
    p=0,0 v=1,3
    p=3,0 v=-2,-2
    p=7,6 v=-1,-3
    p=3,0 v=-1,-2
    p=9,3 v=2,3
    p=7,3 v=-1,2
    p=2,4 v=2,-3
    p=9,5 v=-3,-3
"};

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE, (11, 7)), 12);
    // assert_eq!(solve(EXAMPLE, (11, 7), true), 0);
}
