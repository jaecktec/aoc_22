#![allow(unused)]

extern crate array_tool;
extern crate core;

use std::borrow::{Borrow, BorrowMut};
use std::collections::HashSet;
use std::ops::{Deref, Index};

use array_tool::vec::Intersect;

const PRIORITIES: [char; 53] = ['_', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    for line in input.split("\n") {
        let (dep1, dep2) = line.split_at(line.len() / 2);
        let mut known: Vec<char> = Vec::new();
        for c in dep1.chars() {
            if dep2.contains(c) && !known.contains(c.borrow()) {
                known.push(c);
                let value = PRIORITIES.iter().position(|&i| i == c).unwrap() as u32;
                println!("{} - {}", c, value);
                result += value;
            }
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let lines: Vec<&str> = input.split("\n").collect();
    let chunks = lines.chunks(3);

    for chunk in chunks {
        if chunk.len() < 3 { continue; }
        let r0 = chunk[0].chars().collect::<Vec<_>>();
        let r1 = chunk[1].chars().collect::<Vec<_>>();
        let r2 = chunk[2].chars().collect::<Vec<_>>();

        for c in r0.intersect(r1.intersect(r2)) {
            let value = PRIORITIES.iter().position(|&i| i == c).unwrap() as u32;
            // println!("{} - {}", c, value);
            result += value;
        }
    }
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    // advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
