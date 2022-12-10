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

    let result: i32 = grid
        .iter()
        .enumerate()
        .map(|(row_i, row)| {
            row.iter()
                .enumerate()
                .filter(move |(col_i, height)| {
                    if row_i == 0
                        || *col_i == 0
                        || row_i == grid_ref.len()
                        || *col_i == row.len() - 1
                    {
                        return true;
                    }

                    let col = grid_ref.iter().map(|r| r[*col_i]).collect::<Vec<i32>>();

                    return [
                        row.get(..*col_i).unwrap(),       // before in row
                        row.get((*col_i + 1)..).unwrap(), // after in row
                        col.get(..row_i).unwrap(),        // before in col
                        col.get((row_i + 1)..).unwrap(),  // after in col
                    ]
                    .iter()
                    .any(|view| view.iter().all(|view_height| *height > view_height));
                })
                .fold(0, |acc, _| acc + 1)
            // .filter(|x| *x == true)
        })
        .sum();

    println!("{:?}", result);
}
