// https://adventofcode.com/2022/day/1

use std::fs;

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let items = data.lines();

    let mut i = 0;
    let mut calorie_counts: Vec<Vec<i32>> = Vec::new();
    calorie_counts.push(Vec::new());

    for item in items {
        let calorie_count = match item.parse::<i32>() {
            Ok(val) => val,
            Err(_) => {
                calorie_counts.push(Vec::new());
                i += 1;
                continue;
            }
        };
        calorie_counts[i].push(calorie_count);
    }

    let result = calorie_counts.iter().fold(0, |acc, curr| {
        let sum_current = curr.iter().fold(0, |sum, x| sum + x);
        if sum_current > acc {
            sum_current
        } else {
            acc
        }
    });

    println!("Result: {}", result);
}