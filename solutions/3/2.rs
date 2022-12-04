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
    let groups = data
        .lines()
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, item)| {
            if i % 3 == 0 {
                acc.push(Vec::new());
            }
            acc.last_mut().unwrap().push(item);
            acc
        });

    let mut priorities: Vec<i32> = vec![0; 53];

    for group in groups {
        match group.as_slice() {
            [a, b, c] => {
                let first: HashSet<char> = a.chars().collect();
                let second: HashSet<char> = b.chars().collect();
                let third: HashSet<char> = c.chars().collect();

                let first_pass: HashSet<char> = first.intersection(&second).cloned().collect();
                let final_pass: Vec<char> = first_pass.intersection(&third).cloned().collect();
                priorities[get_index(&final_pass[0])] += 1;
            },
            _ => break
        }
    }

    let result = priorities
        .iter()
        .enumerate()
        .fold(0, |acc, (i, item)| acc + i as i32 * *item);

    println!("{}", result);
}
