// https://adventofcode.com/2022/day/7

use std::{collections::HashMap, fs};

#[derive(Debug)]
struct File {
    name: String,
    index: usize,
    parent: usize,
    size: i64,
}

fn directory_sum(
    files: &Vec<File>,
    directories: &Vec<File>,
    dir_index: usize,
    visited: &mut HashMap<usize, i64>,
) -> i64 {
    let file_sum: i64 = files
        .iter()
        .filter(|file| file.parent == dir_index)
        .map(|file| file.size)
        .sum();

    let directory_sum = directories
        .iter()
        .filter(|dir| dir.parent == dir_index)
        .fold(file_sum, |sum, dir| {
            if visited.contains_key(&dir.index) {
                return sum + visited.get(&dir.index).unwrap();
            }
            return sum + directory_sum(files, directories, dir.index, visited);
        });

    visited.insert(dir_index, directory_sum);
    return directory_sum;
}

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let commands = data.lines();

    let mut current_index = 0;
    let mut current_parent: Vec<usize> = Vec::new();

    let mut directories: Vec<File> = Vec::new();
    let mut files: Vec<File> = Vec::new();

    for command in commands {
        match command.split(" ").collect::<Vec<&str>>().as_slice() {
            ["$", "cd", ".."] => {
                current_index = current_parent.pop().unwrap();
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
                current_parent.push(current_index);
                current_index = directories
                    .iter()
                    .find(|dir| dir.parent == current_index && dir.name == *dirname)
                    .unwrap()
                    .index;
            }
            ["$", "ls"] => {}
            ["dir", dirname] => {
                directories.push(File {
                    name: String::from(*dirname),
                    index: directories.len(),
                    parent: current_index,
                    size: 0,
                });
            }
            [size, filename] => {
                files.push(File {
                    name: String::from(*filename),
                    parent: current_index,
                    index: 0,
                    size: size.parse::<i64>().unwrap(),
                });
            }
            _ => {}
        }
    }

    let mut visited: HashMap<usize, i64> = HashMap::new();

    let result: i64 = directories
        .iter()
        .filter(|dir| dir.name != "/")
        .map(|dir| directory_sum(&files, &directories, dir.index, &mut visited))
        .filter(|size| *size <= 100000)
        .sum();

    println!("{}", result);
}
