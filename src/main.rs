// https://adventofcode.com/2022/day/11

use std::{collections::VecDeque, fs};

struct Item {
    worry_level: i32,
}

struct Monkey {
    num: usize,
    items: VecDeque<Item>,
}

fn main() {
    let data = fs::read_to_string("test.txt").unwrap();
    let lines = data.lines().collect::<Vec<&str>>();
    let chunks = lines
        .chunks(7)
        .map(|chunk| {
            chunk
                .iter()
                .map(|line| line.trim().split(' ').collect::<Vec<&str>>())
                .collect::<Vec<Vec<&str>>>()
        })
        .collect::<Vec<Vec<Vec<&str>>>>();

    let monkeys: Vec<Monkey> = Vec::new();

    for chunk in chunks {
        for line in chunk {
            match line.as_slice() {
                ["Monkey", num] => {}
                ["Starting", "items:", items @ ..] => {}
                ["Operation:", "new", "=", a, operator, b] => {}
                ["Test:", "divisible", "by", num] => {}
                ["If", cond, "throw", "to", "monkey", num] => {}
                [""] => {}
                _ => {
                    panic!("Invalid note: {:?}", line)
                }
            }
        }
    }
}
