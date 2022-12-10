// https://adventofcode.com/2022/day/7

use std::{collections::HashMap, fs};

#[derive(Debug)]
struct File {
    name: String,
    index: usize,
    parent: usize,
    size: i64,
}

fn get_dir_sum(
    files: &Vec<File>,
    directories: &Vec<File>,
    dir_index: usize,
    visited: &mut HashMap<String, i64>,
) -> i64 {
    let mut level_sum: i64 = 0;

    for file in files.iter().filter(|file| file.parent == dir_index) {
        level_sum += file.size;
    }

    for directory in directories.iter().filter(|dir| dir.parent == dir_index) {
        if visited.contains_key(&directory.name) {
            level_sum += visited.get(&directory.name).unwrap();
        } else {
            level_sum += get_dir_sum(files, directories, directory.index, visited);
        }
    }

    visited.insert(
        directories
            .iter()
            .find(|dir| dir.index == dir_index)
            .unwrap()
            .name
            .clone(),
        level_sum,
    );

    // println!("{:?}", visited);
    return level_sum;
}

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let commands = data.lines();

    let mut index = 0;
    let mut directories: Vec<File> = Vec::new();
    let mut files: Vec<File> = Vec::new();

    for command in commands {
        match command.split(" ").collect::<Vec<&str>>().as_slice() {
            ["$", "cd", ".."] => {
                index -= 1;
            }
            ["$", "cd", "/"] => {
                directories.push(File {
                    name: String::from("/"),
                    index: 0,
                    parent: 0,
                    size: 0,
                });
            }
            ["$", "cd", dirname] => {
                let new_index = directories
                    .iter()
                    .find(|dir| dir.name == *dirname)
                    .unwrap()
                    .index;
                index = new_index;
            }
            ["$", "ls"] => {}
            ["dir", dirname] => {
                directories.push(File {
                    name: String::from(*dirname),
                    index: directories.len(),
                    parent: index,
                    size: 0,
                });
            }
            [size, filename] => {
                files.push(File {
                    name: String::from(*filename),
                    parent: index,
                    index: 0,
                    size: size.parse::<i64>().unwrap(),
                });
            }
            _ => {}
        }
    }

    let mut visited: HashMap<String, i64> = HashMap::new();

    let result: i64 = directories
        .iter()
        .filter(|dir| dir.name != "/")
        .map(|dir| get_dir_sum(&files, &directories, dir.index, &mut visited))
        .filter(|size| *size <= 100000)
        .sum();

    println!("{}", result);
}
