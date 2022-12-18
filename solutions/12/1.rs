// https://adventofcode.com/2022/day/12

use std::{
    collections::{HashSet, VecDeque},
    fs,
};

type Pos = (i32, i32);

#[derive(Debug)]
struct Path {
    next: Pos,
    path_so_far: Vec<Pos>,
}

fn get_height_from_letter(letter: char) -> i32 {
    if letter == 'S' {
        return 0;
    }

    if letter == 'E' {
        return 25;
    }

    ('a'..='z').into_iter().position(|c| c == letter).unwrap() as i32
}

fn bfs(grid: &Vec<Vec<char>>, start: Pos, end: Pos) -> usize {
    let mut queue: VecDeque<Path> = VecDeque::from([Path {
        next: start,
        path_so_far: Vec::from([start]),
    }]);
    let mut visited: HashSet<Pos> = HashSet::from([start]);

    let mut path_lens: Vec<usize> = Vec::new();

    while !queue.is_empty() {
        let mut path = queue.pop_front().unwrap();
        let (curr_r, curr_c) = path.next;
        path.path_so_far.push((curr_r, curr_c));

        let curr_height = get_height_from_letter(grid[curr_r as usize][curr_c as usize]);

        for (r, c) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let (next_r, next_c) = (curr_r + r, curr_c + c);

            if next_r >= 0
                && next_r < grid.len() as i32
                && next_c >= 0
                && next_c < grid[0].len() as i32
                && !visited.contains(&(next_r, next_c))
            {
                let next_height = get_height_from_letter(grid[next_r as usize][next_c as usize]);

                if next_height - curr_height <= 1 {
                    if next_r == end.0 && next_c == end.1 {
                        path_lens.push(path.path_so_far.len() - 1);
                        continue;
                    }

                    visited.insert((next_r, next_c));

                    queue.push_back(Path {
                        next: (next_r, next_c),
                        path_so_far: path.path_so_far.clone(),
                    });
                }
            }
        }
    }

    return *path_lens.iter().min().unwrap();
}

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let mut start_pos: Pos = (0, 0);
    let mut end_pos: Pos = (0, 0);
    let items = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for (row, item) in items.iter().enumerate() {
        let start = item.iter().position(|c| *c == 'S');
        let end = item.iter().position(|c| *c == 'E');
        match start {
            Some(col) => start_pos = (row as i32, col as i32),
            None => {}
        }
        match end {
            Some(col) => end_pos = (row as i32, col as i32),
            None => {}
        }
    }

    let result = bfs(&items, start_pos, end_pos);

    println!("{}", result);
}
