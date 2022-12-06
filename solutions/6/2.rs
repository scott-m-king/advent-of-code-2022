// https://adventofcode.com/2022/day/6

use std::{collections::HashSet, fs};

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let chars: Vec<char> = data.chars().collect();

    let mut set: HashSet<char> = HashSet::new();

    let mut i = 1;
    let mut left = 0;

    for char in chars.as_slice() {
        while set.contains(&char) {
            set.remove(&chars[left]);
            left += 1;
        }
        set.insert(*char);
        if set.len() == 14 {
            break;
        }
        i += 1;
    }

    println!("{}", i);
}
