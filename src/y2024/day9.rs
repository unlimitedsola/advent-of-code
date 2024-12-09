use std::iter::repeat_n;
use std::num::NonZeroU64;

use crate::aoc::input;

#[test]
fn part1() {
    dbg!(solve1(&input!()));
}

fn solve1(input: &str) -> u64 {
    let mut file_id = NonZeroU64::new(1).unwrap();
    let mut empty = false;
    let mut blocks: Vec<Option<NonZeroU64>> = Vec::new();
    for c in input
        .chars()
        .filter(char::is_ascii_digit)
        .map(|c| c.to_digit(10).unwrap())
    {
        if empty {
            blocks.extend(repeat_n(None, c as usize));
        } else {
            blocks.extend(repeat_n(Some(file_id), c as usize));
            file_id = file_id.checked_add(1).unwrap();
        }
        empty = !empty;
    }
    dbg_print(&blocks);

    let mut i = 0usize;
    while i < blocks.len() {
        if blocks[i].is_none() {
            blocks.swap_remove(i);
        } else {
            i += 1;
        }
    }
    dbg_print(&blocks);

    blocks
        .iter()
        .enumerate()
        .map(|(i, b)| {
            if b.is_none() {
                return 0;
            }
            let b = b.unwrap().get();
            let b = b - 1;
            i as u64 * b
        })
        .sum()
}

fn dbg_print(blocks: &[Option<NonZeroU64>]) {
    for b in blocks.iter() {
        match b {
            None => {
                print!(".")
            }
            Some(c) => print!("[{}]", c.get() - 1),
        }
    }
    println!();
}

#[test]
fn part2() {
    dbg!(solve2(&input!()));
}

fn solve2(input: &str) -> u64 {
    let mut file_id = NonZeroU64::new(1).unwrap();
    let mut empty = false;
    let mut blocks: Vec<Option<NonZeroU64>> = Vec::new();
    for c in input
        .chars()
        .filter(char::is_ascii_digit)
        .map(|c| c.to_digit(10).unwrap())
    {
        if empty {
            blocks.extend(repeat_n(None, c as usize));
        } else {
            blocks.extend(repeat_n(Some(file_id), c as usize));
            file_id = file_id.checked_add(1).unwrap();
        }
        empty = !empty;
    }
    dbg_print(&blocks);

    let max_file_id = file_id.get() - 1;
    for fid in (1..=max_file_id).rev() {
        let fid = NonZeroU64::new(fid).unwrap();
        let fsz = blocks
            .iter()
            .filter(|b| b.is_some_and(|f| f == fid))
            .count();
        for free_start in 0..blocks.len() {
            if blocks[free_start] == Some(fid) {
                break;
            }
            if blocks[free_start].is_none() {
                let free_end = free_start + fsz;
                if (free_start..free_end).all(|i| blocks[i].is_none()) {
                    // Remove all blocks with file id `fid`.
                    for x in blocks.iter_mut() {
                        if x.is_some_and(|f| f == fid) {
                            *x = None;
                        }
                    }
                    blocks[free_start..free_end].fill(Some(fid));
                    break;
                }
            }
        }
    }
    dbg_print(&blocks);

    blocks
        .iter()
        .enumerate()
        .map(|(i, b)| {
            if b.is_none() {
                return 0;
            }
            let b = b.unwrap().get();
            let b = b - 1;
            i as u64 * b
        })
        .sum()
}

const EXAMPLE: &str = "2333133121414131402";

#[test]
fn test_example() {
    assert_eq!(solve1(EXAMPLE), 1928);
    assert_eq!(solve2(EXAMPLE), 2858);
}
