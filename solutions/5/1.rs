// https://adventofcode.com/2022/day/5

use std::fs;

#[derive(Debug)]
struct Move {
    count: i32,
    start: usize,
    end: usize,
}

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let items = data.lines();

    let n = items.clone().position(|x| x == "").unwrap();

    let raw_stacks = items.clone().take(n);
    let raw_moves = items.skip(n + 1);

    let mut stacks = raw_stacks.clone().last().unwrap().chars().enumerate().fold(
        Vec::new(),
        |mut acc, (i, char)| {
            if char != ' ' {
                let mut current_stack = raw_stacks
                    .clone()
                    .map(|str| *str.chars().collect::<Vec<char>>().get(i).unwrap())
                    .filter(|c| *c != ' ')
                    .collect::<Vec<char>>();

                current_stack.reverse();

                acc.push(current_stack.get(1..).unwrap().to_vec());
            }
            acc
        },
    );

    let moves: Vec<Move> = raw_moves
        .map(|s| {
            let positions = s
                .split(" ")
                .filter(|s| s.parse::<i32>().is_ok())
                .map(|s| s.parse::<i32>().unwrap())
                .take(3)
                .collect::<Vec<i32>>();

            match positions.as_slice() {
                [a, b, c] => Move {
                    count: *a,
                    start: *b as usize,
                    end: *c as usize,
                },
                _ => Move {
                    count: 0,
                    start: 0,
                    end: 0,
                },
            }
        })
        .collect();

    for mov in moves {
        let Move { count, start, end } = mov;
        for _ in 0..count {
            let origin_item = stacks.get_mut(start - 1).unwrap().pop().unwrap();
            stacks[end - 1].push(origin_item);
        }
    }

    let top_chars = stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>();

    println!("{:?}", top_chars);
}
