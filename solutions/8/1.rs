// https://adventofcode.com/2022/day/8

use std::fs;

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let items = data.lines();

    let grid = items
        .map(|row| {
            row.chars()
                .map(|height| height.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect::<Vec<Vec<i32>>>();

    let grid_ref = &grid;

    let result = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, height)| {
                    if i == 0 || j == 0 || i == grid_ref.len() || j == row.len() - 1 {
                        return true;
                    }

                    let r_before = row.get(..j).unwrap();
                    let r_after = row.get((j + 1)..).unwrap();

                    let col = grid_ref.iter().map(|r| r[j]).collect::<Vec<i32>>();
                    let c_before = col.get(..i).unwrap();
                    let c_after = col.get((i + 1)..).unwrap();

                    return [r_before, r_after, c_before, c_after]
                        .iter()
                        .any(|view| view.iter().all(|view_height| height > view_height));
                })
                .filter(|x| *x == true)
        })
        .fold(0, |acc, curr| acc + curr.collect::<Vec<bool>>().len());

    println!("{:?}", result);
}
