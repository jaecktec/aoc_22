pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split("\n");

    let mut idx = 0;
    let mut elves: Vec<u32> = vec![0; 0];

    for line in lines {
        if line.is_empty() {
            idx += 1;
            continue;
        }
        if elves.len() < (idx + 1) {
            elves.push(0);
        }
        let value: u32 = line.parse().unwrap();
        elves[idx] += value;
    }

    let result = elves.iter().max().unwrap();

    return Some(*result);
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split("\n");

    let mut idx = 0;
    let mut elves: Vec<u32> = vec![0; 0];

    for line in lines {
        if line.is_empty() {
            idx += 1;
            continue;
        }
        if elves.len() < (idx + 1) {
            elves.push(0);
        }
        let value: u32 = line.parse().unwrap();
        elves[idx] += value;
    }

    elves.sort();
    elves.reverse();

    return Some(elves[0] + elves[1] + elves[2]);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
