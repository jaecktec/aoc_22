#![allow(unused)]

use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let chars: Vec<char> = input.chars().collect();
    // find start maker
    return find_start(chars, 4);
}

fn find_start(chars: Vec<char>, number_of_unique: usize) -> Option<u32> {
    let chars_length = chars.len();
    for idx in (number_of_unique - 1)..chars_length {
        let mut values: HashSet<char> = HashSet::new();

        for x in 0..number_of_unique {
            values.insert(chars[idx - x]);
        }
        if values.len() == number_of_unique {
            return Some(idx as u32 + 1);
        }
    }
    return None;
}

pub fn part_two(input: &str) -> Option<u32> {
    let chars: Vec<char> = input.chars().collect();
    return find_start(chars, 14);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(7));
        assert_eq!(part_one("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        assert_eq!(part_one("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
        assert_eq!(part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
        assert_eq!(part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(19));
        assert_eq!(part_two("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(23));
        assert_eq!(part_two("nppdvjthqldpwncqszvftbrmjlhg"), Some(23));
        assert_eq!(part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(29));
        assert_eq!(part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(26));
    }
}
