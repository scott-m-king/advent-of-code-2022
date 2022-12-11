// https://adventofcode.com/2022/day/9

use core::panic;
use std::{collections::HashSet, fs};

// x = left/right of start, y = up/down of start
type Pos = (i32, i32);
type Move = (String, i32);

#[derive(Debug)]
struct Knot {
    head: Pos,
    tail: Pos,
    tail_visited: HashSet<Pos>,
}

impl Knot {
    pub fn process_move(&mut self, (dir, count): &Move) {
        // println!("{}, {}", dir, count);

        for _ in 0..*count {
            let (x, y) = self.head;
            match dir.as_str() {
                "U" => {
                    self.head = (x, y + 1);
                }
                "D" => {
                    self.head = (x, y - 1);
                }
                "L" => {
                    self.head = (x - 1, y);
                }
                "R" => {
                    self.head = (x + 1, y);
                }
                _ => {
                    panic!("invalid move: {} {}", dir, count)
                }
            };
            self._update_tail();
            // self.print_path();
        }
    }

    fn _update_tail(&mut self) {
        if !self._is_adjacent(self.tail) {
            let new_tail = self._move_tail();
            self.tail_visited.insert(new_tail);
            self.tail = new_tail;
        }
    }

    fn _move_tail(&mut self) -> Pos {
        let (head_x, head_y) = self.head;
        let (tail_x, tail_y) = self.tail;

        // moving left/right
        if head_y == tail_y {
            return (tail_x + if tail_x < head_x { 1 } else { -1 }, tail_y);
        }

        // moving up/down
        if head_x == tail_x {
            return (tail_x, tail_y + if tail_y < head_y { 1 } else { -1 });
        }

        for x in -1..=1 {
            for y in -1..=1 {
                if x != 0 && y != 0 {
                    let new_pos = (tail_x + x, tail_y + y);
                    if self._is_adjacent(new_pos) {
                        return new_pos;
                    }
                }
            }
        }

        panic!("uh oh");
    }

    fn _is_adjacent(&self, (x2, y2): Pos) -> bool {
        let (x1, y1) = self.head;

        for x in -1..=1 {
            for y in -1..=1 {
                if x1 + x == x2 && y1 + y == y2 {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn print_path(&self) {
        let max_x = *self.tail_visited.iter().map(|(x, _)| x).max().unwrap();
        let min_x = *self.tail_visited.iter().map(|(x, _)| x).min().unwrap();
        let max_y = *self.tail_visited.iter().map(|(_, y)| y).max().unwrap();
        let min_y = *self.tail_visited.iter().map(|(_, y)| y).min().unwrap();

        let mut grid: Vec<Vec<&str>> = Vec::new();

        for _ in min_x..(max_x + 2) {
            let mut row: Vec<&str> = Vec::new();
            for _ in min_y..(max_y + 2) {
                row.push(" ");
            }
            grid.push(row);
        }

        for (x, y) in &self.tail_visited {
            grid[(*x - min_x) as usize][(*y - min_y) as usize] = "#";
        }

        grid[(self.tail.0 - min_x) as usize][(self.tail.1 - min_y) as usize] = "T";
        grid[(self.head.0 - min_x) as usize][(self.head.1 - min_y) as usize] = "H";

        println!("\n");
        println!("head: {:?}", self.head);
        println!("tail: {:?}", self.tail);

        for row in grid {
            println!("{:?}", row);
        }
        println!("\n");
    }
}

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let moves = data
        .lines()
        .map(|item| {
            let split = item.split(" ").collect::<Vec<&str>>();
            match split.as_slice() {
                [dir, count] => (String::from(*dir), count.parse::<i32>().unwrap()),
                _ => {
                    panic!("Couldn't parse")
                }
            }
        })
        .collect::<Vec<Move>>();

    let mut knot = Knot {
        head: (0, 0),
        tail: (0, 0),
        tail_visited: HashSet::from([(0, 0)]),
    };

    for mov in moves {
        knot.process_move(&mov);
    }

    // 6358 too high
    // knot.print_path();
    println!("{}", knot.tail_visited.len());
}
