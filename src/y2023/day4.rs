use crate::aoc::input;

fn parse_cards(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| {
            let (_, numbers) = line.split_once(": ").unwrap();
            let (winning_nums, my_nums) = numbers.split_once(" | ").unwrap();
            (parse_nums(winning_nums), parse_nums(my_nums))
        })
        .map(|(winning, my)| wins(&winning, &my))
        .collect()
}

fn parse_nums(nums: &str) -> Vec<usize> {
    nums.split_ascii_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect()
}

fn wins(winning: &[usize], my: &[usize]) -> usize {
    my.iter().filter(|n| winning.contains(n)).count()
}

#[test]
fn part1() {
    let wins = parse_cards(&input!());

    let sum: usize = wins.iter().filter(|&&w| w > 0).map(|&w| 1 << (w - 1)).sum();

    dbg!(sum);
}

#[test]
fn part2() {
    let wins = parse_cards(&input!());

    let mut cards = 0u32;
    let mut stack: Vec<usize> = (0..wins.len()).collect();
    while let Some(i) = stack.pop() {
        cards += 1;
        for j in 1..=wins[i] {
            stack.push(i + j);
        }
    }

    dbg!(cards);
}
