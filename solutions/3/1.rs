// https://adventofcode.com/2022/day/3

use std::collections::HashSet;
use std::fs;

fn get_index(c: &char) -> usize {
    if c.is_lowercase() {
        *c as usize - 'a' as usize + 1
    } else {
        26 + *c as usize - 'A' as usize + 1
    }
}

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let items: Vec<(&str, &str)> = data
        .lines()
        .map(|x| {
            let first = x.get(0..(x.len() / 2)).unwrap();
            let second = x.get((x.len() / 2)..).unwrap();
            (first, second)
        })
        .collect();

    let mut priorities: Vec<i32> = vec![0; 53];

    for (first, second) in items {
        let set: HashSet<char> = first.chars().collect();
        for char in second.chars() {
            if set.contains(&char) {
                priorities[get_index(&char)] += 1;
                break;
            }
        }
    }

    let result = priorities
        .iter()
        .enumerate()
        .fold(0, |acc, (i, item)| acc + i as i32 * *item);

    println!("{}", result);
}
