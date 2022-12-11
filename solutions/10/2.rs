// https://adventofcode.com/2022/day/10

use std::{collections::VecDeque, fs};

#[derive(Debug)]
enum Command {
    Addx,
    Noop,
}

#[derive(Debug)]
struct Instruction {
    kind: Command,
    val: i32,
    cycles_left: usize,
}

struct CRT {
    pixels: Vec<Vec<String>>,
    current_pos: (usize, usize),
    sprite_pos: [usize; 3],
}

impl CRT {
    pub fn draw(&mut self) {
        let (col, row) = self.current_pos;
        if self.sprite_pos.contains(&col) {
            self.pixels[row][col] = String::from("#");
        }
        if col == 39 {
            self.current_pos = (0, row + 1);
        } else {
            self.current_pos = (col + 1, row);
        }
    }

    pub fn move_sprite(&mut self, new_middle: i32) {
        let x = new_middle as usize;
        self.sprite_pos = [x - 1, x, x + 1];
    }

    pub fn print(&self) {
        println!();
        for row in self.pixels.clone() {
            row.iter().for_each(|char| print!("{}{}", char, char));
            println!();
        }
    }
}

struct CPU {
    cycle: usize,
    register: i32,
    queue: VecDeque<Instruction>,
    display: CRT,
}

impl CPU {
    pub fn tick(&mut self) {
        self.display.draw();
        let current_instruction = self.queue.get_mut(0);
        match current_instruction {
            Some(mut instruction) => {
                let Instruction {
                    kind,
                    val,
                    cycles_left,
                } = instruction;

                match kind {
                    Command::Addx => {
                        if *cycles_left == 0 {
                            self.register += *val;
                            self.display.move_sprite(self.register);
                            self.queue.pop_front();
                        } else {
                            instruction.cycles_left -= 1;
                        }
                    }
                    Command::Noop => {
                        self.queue.pop_front();
                    }
                }
                self.cycle += 1;
            }
            None => {
                panic!("Instruction error")
            }
        };
    }

    pub fn has_instructions_left(&self) -> bool {
        !self.queue.is_empty()
    }
}

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let instructions = data
        .lines()
        .map(|item| {
            let command = item.split(' ').collect::<Vec<&str>>();
            match command.as_slice() {
                ["addx", num] => Instruction {
                    kind: Command::Addx,
                    val: num.parse::<i32>().unwrap(),
                    cycles_left: 1,
                },
                ["noop"] => Instruction {
                    kind: Command::Noop,
                    val: 0,
                    cycles_left: 0,
                },
                _ => {
                    panic!("Failed to parse instruction")
                }
            }
        })
        .collect::<Vec<Instruction>>();

    let mut cpu = CPU {
        cycle: 1,
        register: 1,
        queue: VecDeque::from(instructions),
        display: CRT {
            pixels: vec![vec![String::from("."); 40]; 6],
            current_pos: (0, 0),
            sprite_pos: [0, 1, 2],
        },
    };

    while cpu.has_instructions_left() {
        cpu.tick();
    }

    cpu.display.print();
}

