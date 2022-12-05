// https://adventofcode.com/2022/day/5

use std::fs;

#[derive(Debug)]
struct Move {
    count: i32,
    start: usize,
    dest: usize,
}

fn main() {
    let data = fs::read_to_string("test.txt").unwrap();
    let items = data.lines();

    let n = items.clone().position(|x| x == "").unwrap();

    let raw_stacks = items.clone().take(n);
    let raw_moves = items.skip(n + 1);

    let stacks = raw_stacks.clone().last().unwrap().chars().enumerate().fold(
        Vec::new(),
        |mut acc, (i, char)| {
            if char != ' ' {
                let current_stack = raw_stacks
                    .clone()
                    .map(|str| *str.chars().collect::<Vec<char>>().get(i).unwrap())
                    .filter(|c| *c != ' ')
                    .collect::<Vec<char>>();
                acc.push(
                    current_stack
                        .get(..(current_stack.len() - 1))
                        .unwrap()
                        .to_vec(),
                );
            }
            acc
        },
    );

    let moves: Vec<Move> = raw_moves
        .map(|s| {
            let positions = s
                .chars()
                .filter(|c| c.is_numeric())
                .map(|c| c.to_digit(10).unwrap() as i32)
                .take(3)
                .collect::<Vec<i32>>();

            match positions.as_slice() {
                [a, b, c] => Move {
                    count: *a,
                    start: *b as usize,
                    dest: *c as usize,
                },
                _ => Move {
                    count: 0,
                    start: 0,
                    dest: 0,
                },
            }
        })
        .collect();

    println!("{:?}", stacks);
    println!("{:?}", moves);
}
