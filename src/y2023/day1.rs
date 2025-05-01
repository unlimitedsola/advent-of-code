use crate::aoc::input;

#[test]
fn part1() {
    let input = input!();
    let sum: u32 = input
        .lines()
        .map(|l| {
            let mut digits = l.chars().filter(char::is_ascii_digit);
            let first = digits.next().unwrap();
            let last = digits.next_back().unwrap_or(first);
            (first, last)
        })
        .map(|(a, b)| a.to_digit(10).unwrap() * 10 + b.to_digit(10).unwrap())
        .sum();
    dbg!(sum);
}

#[test]
fn part2() {
    let input = input!();
    let nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut sum = 0;
    for line in input.lines() {
        let mut offset = 0;
        let mut first = None;
        'outer: while offset < line.len() {
            let char = line.chars().nth(offset).unwrap();
            if char.is_ascii_digit() {
                first = Some(char.to_digit(10).unwrap());
                break;
            }
            for (i, &num) in nums.iter().enumerate() {
                if line[offset..].starts_with(num) {
                    first = Some(i as u32 + 1); // 0-indexed
                    break 'outer;
                }
            }
            offset += 1;
        }
        let mut limit = line.len();
        let mut last = None;
        'outer: while limit > 0 {
            let char = line.chars().nth(limit - 1).unwrap();
            if char.is_ascii_digit() {
                last = Some(char.to_digit(10).unwrap());
                break;
            }
            for (i, &num) in nums.iter().enumerate() {
                if line[..limit].ends_with(num) {
                    last = Some(i as u32 + 1); // 0-indexed
                    break 'outer;
                }
            }
            limit -= 1;
        }
        sum += first.unwrap() * 10 + last.unwrap();
    }
    dbg!(sum);
}
