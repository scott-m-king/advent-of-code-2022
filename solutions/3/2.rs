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
    let lines = data.lines().collect::<Vec<&str>>();
    let groups = lines.chunks(3).collect::<Vec<&[&str]>>();

    let mut priorities: Vec<i32> = vec![0; 53];

    for group in groups {
        match group {
            [a, b, c] => {
                let first: HashSet<char> = a.chars().collect();
                let second: HashSet<char> = b.chars().collect();
                let third: HashSet<char> = c.chars().collect();

                let first_pass: HashSet<char> = first.intersection(&second).cloned().collect();
                let final_pass: Vec<char> = first_pass.intersection(&third).cloned().collect();
                priorities[get_index(&final_pass[0])] += 1;
            }
            _ => break,
        }
    }

    let result = priorities
        .iter()
        .enumerate()
        .fold(0, |acc, (i, item)| acc + i as i32 * *item);

    println!("{}", result);
}
