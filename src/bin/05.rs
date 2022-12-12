#![allow(unused)]

use std::borrow::Borrow;
use std::process::id;

use regex::Regex;

type Stack = [[char; 100]; 9];

struct Command {
    amount: u32,
    from: usize,
    to: usize,
}

fn parse_commands(p0: Vec<&str>) -> Vec<Command> {
    let re = Regex::new(r"move\s(\d+)\sfrom\s(\d+)\sto\s(\d+)").unwrap();
    return p0.into_iter()
        .filter(|line| { !line.is_empty() })
        .map(|c| {
            let capture = re.captures(c).into_iter().next().unwrap();

            Command {
                amount: capture[1].parse::<u32>().unwrap(),
                from: usize::try_from(capture[2].parse::<u32>().unwrap() - 1).unwrap(),
                to: usize::try_from(capture[3].parse::<u32>().unwrap() - 1).unwrap(),
            }
        }).collect();
}


pub fn part_one(input: &str) -> Option<String> {
    let lines: Vec<&str> = input.split("\n").collect();
    let stack_command_divider = lines.iter().position(|x| { x == &"" }).unwrap();
    let (stack_lines, commands_lines) = lines.split_at(stack_command_divider);
    let mut stacks: Stack = [[' '; 100]; 9];
    let mut stack_height = [0u32; 9];

    let mut vec = stack_lines.to_vec();
    vec.pop();
    vec.reverse();
    for (i_stack, line) in vec.iter().enumerate() {
        for (i, stack) in stacks.iter_mut().enumerate() {
            let idx = i * 3 + 1 + i;
            let chars = line.chars().collect::<Vec<_>>();
            if chars.len() < idx { break; }
            let char = chars[idx];
            if char.is_numeric() || char == ' ' { continue; }
            stack[usize::try_from(stack_height[i]).unwrap()] = char;
            stack_height[i] += 1;
        }
    }

    let commands = parse_commands(commands_lines.to_vec());

    for command in commands.iter() {
        let mut heap: Vec<char> = Vec::new();

        for _ in 0..command.amount {
            stack_height[command.from] -= 1;
            let x1 = stacks[command.from][usize::try_from(stack_height[command.from]).unwrap()];
            heap.push(x1);
        }
        for c in heap {
            let to_heigt = stack_height[command.to];
            stacks[command.to][usize::try_from(to_heigt).unwrap()] = c;
            stack_height[command.to] += 1;
        }
    }
    let mut result = String::from("");
    for (i, height) in stack_height.into_iter().enumerate() {
        if height < 1 { continue; }
        result.push(stacks[i][usize::try_from(height - 1).unwrap()]);
    }
    return Some(result);
}


pub fn part_two(input: &str) -> Option<String> {
    let lines: Vec<&str> = input.split("\n").collect();
    let stack_command_divider = lines.iter().position(|x| { x == &"" }).unwrap();
    let (stack_lines, commands_lines) = lines.split_at(stack_command_divider);
    let mut stacks: Stack = [[' '; 100]; 9];
    let mut stack_height = [0u32; 9];

    let mut vec = stack_lines.to_vec();
    vec.pop();
    vec.reverse();
    for (i_stack, line) in vec.iter().enumerate() {
        for (i, stack) in stacks.iter_mut().enumerate() {
            let idx = i * 3 + 1 + i;
            let chars = line.chars().collect::<Vec<_>>();
            if chars.len() < idx { break; }
            let char = chars[idx];
            if char.is_numeric() || char == ' ' { continue; }
            stack[usize::try_from(stack_height[i]).unwrap()] = char;
            stack_height[i] += 1;
        }
    }

    let commands = parse_commands(commands_lines.to_vec());

    for command in commands.iter() {
        let mut heap: Vec<char> = Vec::new();

        for _ in 0..command.amount {
            stack_height[command.from] -= 1;
            let x1 = stacks[command.from][usize::try_from(stack_height[command.from]).unwrap()];
            heap.push(x1);
        }
        heap.reverse();
        for c in heap {
            let to_heigt = stack_height[command.to];
            stacks[command.to][usize::try_from(to_heigt).unwrap()] = c;
            stack_height[command.to] += 1;
        }
    }
    let mut result = String::from("");
    for (i, height) in stack_height.into_iter().enumerate() {
        if height < 1 { continue; }
        result.push(stacks[i][usize::try_from(height - 1).unwrap()]);
    }
    return Some(result);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
