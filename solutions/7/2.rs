// https://adventofcode.com/2022/day/7

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs,
};

static TOTAL_SPACE: i32 = 70000000;
static UNUSED_SPACE: i32 = 30000000;

#[derive(Debug)]
struct File {
    name: String,
    index: usize,
    parent: usize,
    size: i32,
}

fn get_directory_size(
    files: &Vec<File>,
    directories: &Vec<File>,
    dir_index: usize,
    visited: &mut HashMap<usize, i32>,
) -> i32 {
    let file_size_sum: i32 = files
        .iter()
        .filter(|file| file.parent == dir_index)
        .map(|file| file.size)
        .sum();

    let directory_size = directories
        .iter()
        .filter(|dir| dir.index != 0 && dir.parent == dir_index)
        .fold(file_size_sum, |sum, dir| {
            if visited.contains_key(&dir.index) {
                return sum + visited.get(&dir.index).unwrap();
            }
            return sum + get_directory_size(files, directories, dir.index, visited);
        });

    visited.insert(dir_index, directory_size);

    return directory_size;
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
                    size: size.parse::<i32>().unwrap(),
                });
            }
            _ => {}
        }
    }

    let mut visited: HashMap<usize, i32> = HashMap::new();

    let unsorted = directories
        .iter()
        .map(|dir| get_directory_size(&files, &directories, dir.index, &mut visited))
        .collect::<Vec<i32>>();

    let mut sorted = unsorted.iter().fold(BinaryHeap::new(), |mut heap, curr| {
        heap.push(Reverse(curr));
        return heap;
    });

    let used_space = unsorted.first().unwrap();
    let threshold = UNUSED_SPACE - (TOTAL_SPACE - used_space);

    while !sorted.is_empty() && sorted.peek().unwrap().0 <= &threshold {
        let result = sorted.pop();
        if sorted.is_empty() {
            println!("{}", result.unwrap().0);
            return;
        }
    }

    println!("{:?}", sorted.peek().unwrap().0);
}
