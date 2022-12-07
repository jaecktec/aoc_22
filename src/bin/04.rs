#![allow(unused)]

fn parse_pair(input: &str) -> [u32; 2] {
    return input
        .split("-")
        .collect::<Vec<_>>()
        .iter().map(|it| it.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .try_into().unwrap();
}

fn is_included_completely(a: [u32; 2], b: [u32; 2]) -> bool {
    return a[0] >= b[0] && a[1] <= b[1];
}

fn is_between([l, r]: [u32; 2], b: u32) -> bool {
    return r >= b && b >= l;
}

fn is_included_partially(a: [u32; 2], b: [u32; 2]) -> bool {
    return is_between(a, b[0])
        || is_between(a, b[1])
        || is_between(b, a[0])
        || is_between(b, a[1])
    ;
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split("\n");
    let mut result = 0u32;
    for line in lines {
        if line.is_empty() { continue; }
        let pair: Vec<&str> = line.split(",").collect();
        let left = parse_pair(pair[0]);
        let right = parse_pair(pair[1]);
        if is_included_completely(right, left) {
            result += 1;
        } else if is_included_completely(left, right) {
            result += 1;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split("\n");
    let mut result = 0u32;
    for line in lines {
        if line.is_empty() { continue; }
        let pair: Vec<&str> = line.split(",").collect();
        let left = parse_pair(pair[0]);
        let right = parse_pair(pair[1]);
        if is_included_partially(right, left) {
            result += 1;
        } else if is_included_partially(left, right) {
            result += 1;
        }
    }

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
