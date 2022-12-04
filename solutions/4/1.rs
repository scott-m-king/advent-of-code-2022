// https://adventofcode.com/2022/day/4

use std::fs;

fn get_tuple(v: Option<&Vec<i32>>) -> (i32, i32) {
    match v.unwrap().as_slice() {
        [a, b] => (*a, *b),
        _ => (0, 0),
    }
}

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let items: Vec<((i32, i32), (i32, i32))> = data
        .lines()
        .map(|s| {
            let pair: Vec<Vec<i32>> = s
                .split(",")
                .map(|p| {
                    p.split("-")
                        .map(|n| n.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect();

            (get_tuple(pair.get(0)), get_tuple(pair.get(1)))
        })
        .collect();

    let overlaps = items
        .iter()
        .filter(|((x1, x2), (y1, y2))| x1 >= y1 && x2 <= y2 || y1 >= x1 && y2 <= x2);

    println!("{}", overlaps.count());
}
