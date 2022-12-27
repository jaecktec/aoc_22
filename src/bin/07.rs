#![allow(unused)]

extern crate core;

use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell};
use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};
use std::slice::Iter;

#[derive(Debug)]
struct File {
    absolute: String,
    name: String,
    size: u32,
}

enum Command {
    Ls,
    ChangeDir(String),
    Output(String),
}

fn parse_fs(lines: Vec<&str>, files: &mut HashMap<String, u32>, folders: &mut HashSet<String>) {
    let mut current_dir = vec![];
    folders.insert(String::from("/"));
    for line in lines {
        if line.is_empty() { continue; }
        let parts: Vec<&str> = line.splitn(3, " ").collect();
        let cmd = match parts[..] {
            ["$", "ls"] => Command::Ls,
            ["$", "cd", _] => Command::ChangeDir(String::from(parts[2])),
            _ => Command::Output(line.to_string()),
        };

        match cmd {
            Command::ChangeDir(name) => {
                match name.as_str() {
                    ".." => { current_dir.pop(); }
                    "/" => { current_dir.clear(); }
                    _ => {
                        current_dir.push(name.clone());
                        folders.insert(format!("/{}", current_dir.join("/")));
                    }
                }
                // println!("change dir {} - {}", name, current_dir.join("/"));
            }
            Command::Output(value) => {
                let parts: Vec<&str> = line.splitn(2, " ").collect();
                match parts[..] {
                    ["dir", name] => {}
                    [size, name] => {
                        let current = current_dir.join("/");
                        folders.insert(current.clone());
                        let absolute_path = if current == "" {
                            format!("/{}", name)
                        } else {
                            format!("/{}/{}", current, name)
                        };
                        files.insert(absolute_path, size.parse().unwrap());
                    }
                    _ => {}
                }
            }
            _ => {}
        };
    };
}

fn calculate_size_of_directory(files: &HashMap<String, u32>, path: &String) -> u32 {
    files
        .iter()
        .filter(|&p| {
            let x1 = p.0.starts_with(format!("{}/", path).as_str());
            let x2 = *path == "/";
            x1 || x2
        })
        .map(|x| x.1)
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut files: HashMap<String, u32> = HashMap::new();
    let mut folders: HashSet<String> = HashSet::new();

    parse_fs(lines, &mut files, &mut folders);

    let mut result = 0u32;
    for path in folders.iter() {
        let total_size = calculate_size_of_directory(&files, path);

        if total_size <= 100000 {
            result += total_size;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut files: HashMap<String, u32> = HashMap::new();
    let mut folders: HashSet<String> = HashSet::new();

    parse_fs(lines, &mut files, &mut folders);

    let fs_size = 70000000u32;
    let required_free_space = 30000000u32;
    let total = calculate_size_of_directory(&files, &"/".to_string());
    let min_size_for_deletion = required_free_space - (fs_size - total);
    println!("required space {}", min_size_for_deletion);
    let result = folders.iter().fold(fs_size, |acc, curr| {
        let dir_total = calculate_size_of_directory(&files, curr);
        if dir_total < min_size_for_deletion { acc }
        else if dir_total > acc { acc }
        else { dir_total }
    });

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        let option = part_one(&input);
        assert_eq!(option, Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
