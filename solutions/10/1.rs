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

struct CPU {
    cycle: usize,
    register: i32,
    queue: VecDeque<Instruction>,
    signals: Vec<i32>,
}

impl CPU {
    pub fn tick(&mut self) {
        self._capture_signal();
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

    fn _capture_signal(&mut self) {
        match self.cycle {
            20 | 60 | 100 | 140 | 180 | 220 => {
                println!(
                    "Cycle: {}, Register: {}, Instruction: {:?}",
                    self.cycle,
                    self.register,
                    self.queue.front().unwrap()
                );
                self.signals.push(self.register * (self.cycle as i32));
            }
            _ => {}
        }
    }
}

fn main() {
    let data = fs::read_to_string("test.txt").unwrap();
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
        signals: Vec::new(),
    };

    while cpu.has_instructions_left() {
        cpu.tick();
    }

    let result: i32 = cpu.signals.iter().sum();
    println!("{:?}", result);
}

