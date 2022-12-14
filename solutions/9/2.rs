// https://adventofcode.com/2022/day/9

use core::panic;
use std::{collections::HashSet, fs};

// x = left/right of start, y = up/down of start
type Pos = (i32, i32);
type Move = (String, i32);

struct Head {
    pos: Pos,
    knots: Vec<Knot>,
}

#[derive(Debug, Clone)]
struct Knot {
    head: Pos,
    pos: Pos,
    visited: HashSet<Pos>,
}

impl Head {
    pub fn process_move(&mut self, (dir, count): &Move) {
        for _ in 0..*count {
            let (x, y) = self.pos;
            match dir.as_str() {
                "U" => {
                    self.pos = (x, y + 1);
                }
                "D" => {
                    self.pos = (x, y - 1);
                }
                "L" => {
                    self.pos = (x - 1, y);
                }
                "R" => {
                    self.pos = (x + 1, y);
                }
                _ => {
                    panic!("invalid move: {} {}", dir, count)
                }
            };

            self._update_knots();
        }
    }

    fn _update_knots(&mut self) {
        let mut new_pos = self.pos;
        let new_knots = self
            .knots
            .iter()
            .cloned()
            .map(|mut knot| {
                new_pos = knot.process_move(new_pos);
                knot
            })
            .collect();
        self.knots = new_knots;
    }
}

impl Knot {
    pub fn process_move(&mut self, new_head: Pos) -> Pos {
        self.head = new_head;
        self._update_pos();
        self.pos
    }

    fn _update_pos(&mut self) {
        if !self._is_adjacent(self.pos) {
            let new_tail = self._move_pos();
            self.visited.insert(new_tail);
            self.pos = new_tail;
        }
    }

    fn _move_pos(&mut self) -> Pos {
        let (head_x, head_y) = self.head;
        let (tail_x, tail_y) = self.pos;

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
        false
    }
}

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let moves = data
        .lines()
        .map(|item| {
            let split = item.split(' ').collect::<Vec<&str>>();
            match split.as_slice() {
                [dir, count] => (String::from(*dir), count.parse::<i32>().unwrap()),
                _ => {
                    panic!("Couldn't parse")
                }
            }
        })
        .collect::<Vec<Move>>();

    let mut head = Head {
        pos: (0, 0),
        knots: vec![
            Knot {
                head: (0, 0),
                pos: (0, 0),
                visited: HashSet::from([(0, 0)])
            };
            9
        ],
    };

    moves.iter().for_each(|mov| head.process_move(mov));

    match head.knots.last() {
        Some(tail) => println!("{:?}", tail.visited.len()),
        None => {
            panic!("Something went wrong")
        }
    }
}
