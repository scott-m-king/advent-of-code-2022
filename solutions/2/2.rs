// https://adventofcode.com/2022/day/2

use std::fs;

static WIN_POINTS: i32 = 6;
static DRAW_POINTS: i32 = 3;

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

impl Hand {
    fn item_needed_to_win(&self, outcome: &Outcome) -> Item {
        match outcome {
            Outcome::Win => self.loses_against.clone(),
            Outcome::Draw => self.item.clone(),
            Outcome::Loss => self.wins_against.clone(),
        }
    }
}

fn get_outcome(letter: &str) -> Outcome {
    match letter {
        "X" => Outcome::Loss,
        "Y" => Outcome::Draw,
        _ => Outcome::Win,
    }
}

fn get_outcome_points(outcome: &Outcome) -> i32 {
    match outcome {
        Outcome::Win => WIN_POINTS,
        Outcome::Loss => 0,
        Outcome::Draw => DRAW_POINTS,
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

    let rock = Hand {
        item: Item::Rock,
        points: 1,
        wins_against: Item::Scissors,
        loses_against: Item::Paper,
    };

    let paper = Hand {
        item: Item::Paper,
        points: 2,
        wins_against: Item::Rock,
        loses_against: Item::Scissors,
    };
    let scissors = Hand {
        item: Item::Scissors,
        points: 3,
        wins_against: Item::Paper,
        loses_against: Item::Rock,
    };

    let score = items.iter().fold(0, |acc, (l1, l2)| {
        let opponent_hand = match *l1 {
            "A" => &rock,
            "B" => &paper,
            _ => &scissors,
        };

        let outcome = get_outcome(*l2);
        let outcome_points = get_outcome_points(&outcome);
        let item_points = match opponent_hand.item_needed_to_win(&outcome) {
            Item::Rock => rock.points,
            Item::Paper => paper.points,
            Item::Scissors => scissors.points,
        };

        acc + outcome_points + item_points
    });

    println!("Result: {}", score);
}
