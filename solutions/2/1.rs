// https://adventofcode.com/2022/day/2

use std::fs;

static WIN_POINTS: i32 = 6;
static DRAW_POINTS: i32 = 3;

#[derive(PartialEq)]
enum Item {
    Rock,
    Paper,
    Scissors,
}

struct Hand {
    item: Item,
    points: i32,
    beats_item: Item,
}

impl Hand {
    fn beats(&self, hand: &Hand) -> bool {
        self.beats_item == hand.item
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
        beats_item: Item::Scissors,
    };

    let paper = Hand {
        item: Item::Paper,
        points: 2,
        beats_item: Item::Rock,
    };
    let scissors = Hand {
        item: Item::Scissors,
        points: 3,
        beats_item: Item::Paper,
    };

    let mut final_score = 0;

    for (p1, p2) in items {
        let opponent_hand = match p1 {
            "A" => &rock,
            "B" => &paper,
            "C" => &scissors,
            _ => continue,
        };

        let my_hand = match p2 {
            "X" => &rock,
            "Y" => &paper,
            "Z" => &scissors,
            _ => continue,
        };

        if my_hand.beats(opponent_hand) {
            final_score += my_hand.points + WIN_POINTS;
        } else if opponent_hand.beats(my_hand) {
            final_score += my_hand.points;
        } else {
            final_score += my_hand.points + DRAW_POINTS;
        }
    }

    println!("Result: {}", final_score);
}
