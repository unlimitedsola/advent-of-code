use grid::Grid;
use indoc::indoc;
use itertools::Itertools;
use pathfinding::directed::bfs::bfs_reach;

use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve1(&input!()));
}

fn parse(input: &str) -> (Grid<char>, Vec<char>) {
    let mut map = vec![];
    let mut lines = input.lines();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        map.push(line.chars().collect_vec());
    }
    let grid = Grid::from(map);
    let moves = lines.flat_map(|l| l.chars()).collect_vec();
    (grid, moves)
}

fn solve1(input: &str) -> u64 {
    let (mut grid, moves) = parse(input);
    let bot_pos = grid
        .indexed_iter()
        .find_map(|(pos, &c)| if c == '@' { Some(pos) } else { None })
        .unwrap();
    let mut bot_pos = (bot_pos.0 as isize, bot_pos.1 as isize);
    for &c in &moves {
        let dir = parse_move(c);
        bot_pos = tick2(&mut grid, bot_pos, dir);
    }
    print_grid(&grid);

    grid.indexed_iter()
        .filter(|(_, &c)| c == 'O')
        .map(|(pos, _)| (pos.0 * 100 + pos.1) as u64)
        .sum()
}

fn v2p(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
    (a.0 + b.0, a.1 + b.1)
}

fn parse_move(c: char) -> (isize, isize) {
    match c {
        '<' => (0, -1),
        '>' => (0, 1),
        '^' => (-1, 0),
        'v' => (1, 0),
        _ => unreachable!(),
    }
}

fn print_grid(grid: &Grid<char>) {
    for r in grid.iter_rows() {
        for c in r {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn expand_grid(grid: &Grid<char>) -> Grid<char> {
    grid.iter_rows()
        .map(|r| {
            r.flat_map(|&c| match c {
                '#' => "##".chars(),
                'O' => "[]".chars(),
                '.' => "..".chars(),
                '@' => "@.".chars(),
                _ => unreachable!(),
            })
            .collect_vec()
        })
        .collect_vec()
        .into()
}

#[test]
fn part2() {
    dbg!(solve2(&input!()));
}

fn solve2(input: &str) -> u64 {
    let (grid, moves) = parse(input);
    let mut grid = expand_grid(&grid);
    let bot_pos = grid
        .indexed_iter()
        .find_map(|(pos, &c)| if c == '@' { Some(pos) } else { None })
        .unwrap();
    let mut bot_pos = (bot_pos.0 as isize, bot_pos.1 as isize);
    for &c in &moves {
        let dir = parse_move(c);
        bot_pos = tick2(&mut grid, bot_pos, dir);
    }
    print_grid(&grid);

    grid.indexed_iter()
        .filter(|(_, &c)| c == '[')
        .map(|(pos, _)| (pos.0 * 100 + pos.1) as u64)
        .sum()
}

fn tick2(grid: &mut Grid<char>, s_pos: (isize, isize), dir: (isize, isize)) -> (isize, isize) {
    let c = grid.get(s_pos.0, s_pos.1).copied().unwrap();
    let mut to_push = bfs_reach((s_pos, c), |&(pos, c)| {
        let mut next = vec![];
        match c {
            '[' => {
                let pp = (pos.0, pos.1 + 1);
                let cc = grid.get(pp.0, pp.1).copied().unwrap();
                next.push((pp, cc))
            }
            ']' => {
                let pp = (pos.0, pos.1 - 1);
                let cc = grid.get(pp.0, pp.1).copied().unwrap();
                next.push((pp, cc))
            }
            _ => {}
        }
        let np = v2p(pos, dir);
        match grid.get(np.0, np.1) {
            Some(&c) if matches!(c, '[' | ']' | 'O') => next.push((np, c)),
            _ => {}
        };
        next
    })
    .collect_vec();

    let mut to_remove = vec![];

    for (pos, c) in to_push.iter_mut() {
        let to_set = v2p(*pos, dir);
        match grid.get(to_set.0, to_set.1) {
            Some('#') => return s_pos,
            _ => {
                to_remove.push(*pos);
                *pos = to_set;
            }
        }
    }

    for pos in to_remove {
        *grid.get_mut(pos.0, pos.1).unwrap() = '.';
    }

    for (pos, c) in to_push {
        *grid.get_mut(pos.0, pos.1).unwrap() = c;
    }

    *grid.get_mut(s_pos.0, s_pos.1).unwrap() = '.';

    v2p(s_pos, dir)
}

const EXAMPLE: &str = indoc! {"
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
"};

const EXAMPLE2: &str = indoc! {"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"};

#[test]
fn test_example() {
    assert_eq!(solve1(EXAMPLE), 2028);
    assert_eq!(solve1(EXAMPLE2), 10092);
    assert_eq!(solve2(EXAMPLE2), 9021);
}
