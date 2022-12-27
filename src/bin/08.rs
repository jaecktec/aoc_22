#![allow(unused)]

use std::collections::HashSet;
use std::io::Read;
use std::num::FpCategory::Subnormal;

const SIZE: usize = 99;

fn parse_input(input: &str) -> [[u32; SIZE]; SIZE] {
    let lines = input.clone().split("\n").filter(|&it| it != "").collect::<Vec<&str>>();
    let input_width = lines[0].chars().collect::<Vec<char>>().len();
    let input_height = lines.len();

    if input_height != SIZE || input_height != SIZE {
        panic!("{} / {} do not match {}", input_height, input_width, SIZE)
    }

    let mut forest = [[0u32; SIZE]; SIZE];
    for (i, line) in input.split("\n").enumerate() {
        for (j, char) in line.chars().enumerate() {
            forest[i][j] = char.to_digit(10).unwrap();
        }
    }
    forest
}

pub fn part_one(input: &str) -> Option<u32> {
    let forest = parse_input(input);

    let mut result = (SIZE * 4 - 4) as u32;
    // println!("got {} at the edges", result);

    for i in 1..(SIZE - 1) {
        for j in 1..(SIZE - 1) {
            let current = forest[i][j];
            let horizontal = forest[i];
            let vertical = forest.iter().map(|it| it[j]).collect::<Vec<u32>>();

            let (to_left, rest_to_right) = horizontal.split_at(j);
            let to_right = rest_to_right.iter().skip(1).map(|x| *x).collect::<Vec<_>>();
            let (to_top, rest_to_bottom) = vertical.split_at(i);
            let to_bottom = rest_to_bottom.iter().skip(1).map(|x| *x).collect::<Vec<_>>();

            let max_from_left = *to_left.iter().max().unwrap();
            let max_from_right = *to_right.iter().max().unwrap();
            let max_from_top = *to_top.iter().max().unwrap();
            let max_from_bottom = *to_bottom.iter().max().unwrap();

            let from_bottom = max_from_bottom < current;
            let from_top = max_from_top < current;
            let from_right = max_from_right < current;
            let from_left = max_from_left < current;

            // print!("the {} at {}-{} is visible from ", current, i, j);

            // if from_bottom { print!("bottom ") }
            // if from_top { print!("top ") }
            // if from_left { print!("left ") }
            // if from_right { print!("right ") }
            // if !from_right && !from_left && !from_bottom && !from_top { print!("nowhere") };

            if from_bottom {
                result += 1;
            } else if from_top {
                result += 1;
            } else if from_right {
                result += 1;
            } else if from_left {
                result += 1;
            }
            // println!();
        }
    }

    Some(result)
    // None
}

pub fn part_two(input: &str) -> Option<u32> {
    let forest = parse_input(input);

    let mut result = 0u32;
    // println!("got {} at the edges", result);

    let mut result = 0u32;

    for i in 1..(SIZE - 1) {
        for j in 1..(SIZE - 1) {
            let current = forest[i][j];
            let horizontal = forest[i];
            let vertical = forest.iter().map(|it| it[j]).collect::<Vec<u32>>();

            let (arr_left, rest_right) = horizontal.split_at(j);
            let mut right = rest_right.iter().skip(1).map(|x| *x).collect::<Vec<_>>();
            let (arr_top, rest_bottom) = vertical.split_at(i);
            let mut bottom = rest_bottom.iter().skip(1).map(|x| *x).collect::<Vec<_>>();

            let mut left = arr_left.to_vec();
            let mut top = arr_top.to_vec();
            left.reverse();
            top.reverse();

            let to_top = count_increasing_sequence(current, top);
            let to_left = count_increasing_sequence(current, left);
            let to_right = count_increasing_sequence(current, right);
            let to_bottom = count_increasing_sequence(current, bottom);

            let iter_result = to_left * to_right * to_top * to_bottom;
            println!("at {}-{} with val: {}: up: {}, left: {}, right: {}, down: {}", i, j, current, to_top, to_left, to_right, to_bottom);

            if iter_result as u32 > result {
                result = iter_result as u32;
            }
        }
    }

    Some(result)
}

fn count_increasing_sequence(current: u32, vec: Vec<u32>) -> u32 {
    let mut result = 0;
    let mut previous_value = 0u32;
    for x in vec {
        if x < current {
            result += 1;
        }else {
            result += 1;
            break;
        }
    }

    return result;
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
