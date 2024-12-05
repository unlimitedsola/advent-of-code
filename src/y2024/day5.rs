use itertools::Itertools;

use crate::aoc::input;

type Rule = (u32, u32);
type Update = Vec<u32>;

fn parse(input: &str) -> (Vec<Rule>, Vec<Update>) {
    let mut lines = input.lines();
    let mut rules = vec![];
    for line in &mut lines {
        if line.is_empty() {
            break;
        } else {
            let (a, b) = line.split_once('|').unwrap();
            let rule: Rule = (a.parse().unwrap(), b.parse().unwrap());
            rules.push(rule);
        }
    }
    let mut updates = vec![];
    for line in &mut lines {
        let update = line.split(',').map(|n| n.parse().unwrap()).collect_vec();
        updates.push(update);
    }
    (rules, updates)
}

#[test]
fn part1() {
    let (rules, updates) = parse(&input!());
    dbg!(solve1(&rules, &updates));
}

fn solve1(rules: &[Rule], updates: &[Update]) -> u32 {
    let mut sum = 0;
    for update in updates {
        if valid(rules, update) {
            let mid = update.len() / 2;
            sum += update[mid];
        }
    }
    sum
}

fn valid(rules: &[Rule], update: &Update) -> bool {
    for rule in rules {
        if let Some((i, _)) = update.iter().find_position(|&&n| n == rule.0) {
            if let Some((j, _)) = update.iter().find_position(|&&n| n == rule.1) {
                if i > j {
                    return false;
                }
            }
        }
    }
    true
}

#[test]
fn part2() {
    let (rules, updates) = parse(&input!());
    dbg!(solve2(&rules, &updates));
}

fn solve2(rules: &[Rule], updates: &[Update]) -> u32 {
    let mut sum = 0;
    for update in updates {
        if !valid(rules, update) {
            let update = order(rules, update);
            let mid = update.len() / 2;
            sum += update[mid];
        }
    }
    sum
}

fn order(rules: &[Rule], update: &Update) -> Update {
    let mut update = update.clone();
    loop {
        let mut changed = false;
        for rule in rules {
            if let Some((i, _)) = update.iter().find_position(|&&n| n == rule.0) {
                if let Some((j, _)) = update.iter().find_position(|&&n| n == rule.1) {
                    if i > j {
                        update.swap(i, j);
                        changed = true;
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }
    update
}
