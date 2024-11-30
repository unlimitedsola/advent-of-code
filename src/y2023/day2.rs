use crate::aoc::input;

#[test]
fn part1() {
    let input = input!();
    let mut sum = 0u32;
    'game: for line in input.lines() {
        let (header, sets) = line.split_once(": ").unwrap();
        let gid: u32 = header["Game ".len()..].parse().unwrap();
        for set in sets.split("; ") {
            for cubes in set.split(", ") {
                let (count, color) = cubes.split_once(' ').unwrap();
                let count: u32 = count.parse().unwrap();
                let valid = match color {
                    "red" if count <= 12 => true,
                    "green" if count <= 13 => true,
                    "blue" if count <= 14 => true,
                    _ => false,
                };
                if !valid {
                    continue 'game;
                }
            }
        }
        sum += gid;
    }
    dbg!(sum);
}

#[test]
fn part2() {
    let input = input!();
    let mut sum = 0u32;
    for line in input.lines() {
        let (_, sets) = line.split_once(": ").unwrap();
        let mut red = 0u32;
        let mut green = 0u32;
        let mut blue = 0u32;
        for set in sets.split("; ") {
            for cubes in set.split(", ") {
                let (count, color) = cubes.split_once(' ').unwrap();
                let count: u32 = count.parse().unwrap();
                match color {
                    "red" if red < count => red = count,
                    "green" if green < count => green = count,
                    "blue" if blue < count => blue = count,
                    _ => {}
                };
            }
        }
        sum += red * green * blue;
    }
    dbg!(sum);
}
