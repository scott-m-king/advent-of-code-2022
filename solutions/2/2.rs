// https://adventofcode.com/2022/day/2

use std::fs;

#[derive(PartialEq, Clone)]
enum Item {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

struct Hand {
    item: Item,
    points: i32,
    wins_against: Item,
    loses_against: Item,
}

static ROCK: Hand = Hand {
    item: Item::Rock,
    points: 1,
    wins_against: Item::Scissors,
    loses_against: Item::Paper,
};

static PAPER: Hand = Hand {
    item: Item::Paper,
    points: 2,
    wins_against: Item::Rock,
    loses_against: Item::Scissors,
};

static SCISSORS: Hand = Hand {
    item: Item::Scissors,
    points: 3,
    wins_against: Item::Paper,
    loses_against: Item::Rock,
};

impl Hand {
    fn item_needed_to_win(&self, outcome: &Outcome) -> Item {
        match outcome {
            Outcome::Win => self.loses_against.clone(),
            Outcome::Loss => self.wins_against.clone(),
            Outcome::Draw => self.item.clone(),
        }
    }
}

fn get_outcome(letter: &str) -> Outcome {
    match letter {
        "Z" => Outcome::Win,
        "X" => Outcome::Loss,
        _ => Outcome::Draw,
    }
}

fn get_outcome_points(outcome: &Outcome) -> i32 {
    match outcome {
        Outcome::Win => 6,
        Outcome::Loss => 0,
        Outcome::Draw => 3,
    }
}

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let items: Vec<(&str, &str)> = data
        .lines()
        .map(|x| {
            let pair: Vec<&str> = x.split(" ").collect();
            (pair[0], pair[1])
        })
        .collect();

    let result = items.iter().fold(0, |acc, (l1, l2)| {
        let opponent_hand = match *l1 {
            "A" => &ROCK,
            "B" => &PAPER,
            _ => &SCISSORS,
        };

        let outcome = get_outcome(*l2);
        let outcome_points = get_outcome_points(&outcome);
        let item_points = match opponent_hand.item_needed_to_win(&outcome) {
            Item::Rock => ROCK.points,
            Item::Paper => PAPER.points,
            Item::Scissors => SCISSORS.points,
        };

        acc + outcome_points + item_points
    });

    println!("Result: {}", result);
}
