// https://adventofcode.com/2022/day/11

use std::{
    collections::{BinaryHeap, HashSet, VecDeque},
    fs,
};

#[derive(Debug, Clone)]
struct Item {
    worry_level: i64,
}

type WorryFn = fn(i64, i64) -> i64;

fn add(a: i64, b: i64) -> i64 {
    a + b
}

fn multiply(a: i64, b: i64) -> i64 {
    a * b
}

fn increase_worry(a: i64, b: i64, f: fn(i64, i64) -> i64) -> i64 {
    f(a, b)
}

#[derive(Debug)]
struct Monkey {
    num: usize,
    items: VecDeque<Item>,
    worry_fn: WorryFn,
    multiplier: String,
    divisible_by: usize,
    true_throw_to: usize,
    false_throw_to: usize,
    times_inspected: usize,
    supermodulo: usize,
}

impl Monkey {
    pub fn new() -> Monkey {
        Monkey {
            num: 0,
            items: VecDeque::new(),
            worry_fn: add,
            multiplier: String::from(""),
            divisible_by: 0,
            true_throw_to: 0,
            false_throw_to: 0,
            times_inspected: 0,
            supermodulo: 0,
        }
    }

    pub fn catch(&mut self, item: Item) {
        self.items.push_back(item);
    }

    pub fn take_turn(&mut self) -> Vec<(usize, Item)> {
        let mut result: Vec<(usize, Item)> = Vec::new();
        while !self.items.is_empty() {
            let item_to_process = self.items.pop_front();
            match item_to_process {
                Some(mut item) => {
                    self._inspect(&mut item);
                    result.push(self._throw_to(&item));
                }
                None => {}
            };
        }
        result
    }

    fn _inspect(&mut self, item: &mut Item) {
        match self.multiplier.as_str() {
            "old" => item.worry_level = item.worry_level.pow(2),
            _ => {
                item.worry_level = increase_worry(
                    item.worry_level,
                    self.multiplier.parse::<i64>().unwrap(),
                    self.worry_fn,
                )
            }
        }
        item.worry_level %= self.supermodulo as i64;
        self.times_inspected += 1;
    }

    fn _test(&self, item: &Item) -> bool {
        item.worry_level % (self.divisible_by as i64) == 0
    }

    fn _throw_to(&self, item: &Item) -> (usize, Item) {
        match self._test(item) {
            true => (self.true_throw_to, item.clone()),
            false => (self.false_throw_to, item.clone()),
        }
    }
}

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
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

    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut supermod: HashSet<usize> = HashSet::new();

    for chunk in chunks {
        let mut monkey = Monkey::new();

        for line in chunk {
            match line.as_slice() {
                ["Monkey", num] => {
                    monkey.num = num.chars().next().unwrap().to_digit(10).unwrap() as usize;
                }
                ["Starting", "items:", rest @ ..] => {
                    let item_strings = rest.join(" ");
                    for item in item_strings.split(", ") {
                        monkey.items.push_back(Item {
                            worry_level: item.parse::<i64>().unwrap(),
                        });
                    }
                }
                ["Operation:", "new", "=", "old", operator, multiplier] => {
                    monkey.worry_fn = match *operator {
                        "+" => add,
                        "*" => multiply,
                        _ => {
                            panic!("Invalid worry_fn operator: {}", *operator)
                        }
                    };

                    monkey.multiplier = String::from(*multiplier);
                }
                ["Test:", "divisible", "by", num] => {
                    monkey.divisible_by = num.parse::<usize>().unwrap();
                    supermod.insert(monkey.divisible_by);
                }
                ["If", cond, "throw", "to", "monkey", num] => {
                    match *cond {
                        "true:" => monkey.true_throw_to = num.parse::<usize>().unwrap(),
                        "false:" => monkey.false_throw_to = num.parse::<usize>().unwrap(),
                        _ => (),
                    };
                }
                [""] => {}
                _ => {
                    panic!("Invalid note: {:?}", line)
                }
            }
        }

        monkeys.push(monkey);
    }

    let len = monkeys.len();

    for i in 0..(len * 10000) {
        let monkey = monkeys.get_mut(i % len).unwrap();
        monkey.supermodulo = supermod.iter().product();
        let throws = monkey.take_turn();
        for (throw_to, item) in throws {
            monkeys[throw_to].catch(item);
        }
    }

    let results: usize = monkeys
        .iter()
        .map(|monkey| monkey.times_inspected)
        .collect::<BinaryHeap<usize>>()
        .iter()
        .take(2)
        .product();

    // 13198681713 too high
    println!("{:#?}", monkeys);
    println!("{}", results);
}
