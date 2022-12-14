// https://adventofcode.com/2022/day/8

use std::fs;

fn get_viewing_distance(line: &[i32], height: &i32, is_reverse: bool) -> i32 {
    let mut result = 1;
    if is_reverse {
        for x in line.iter().rev() {
            if height > x {
                result += 1;
            } else {
                break;
            }
        }
    } else {
        for x in line.iter() {
            if height > x {
                result += 1;
            } else {
                break;
            }
        }
    }
    result
}

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
        .map(|(row_i, row)| {
            row.iter()
                .enumerate()
                .map(move |(col_i, height)| {
                    let col = grid_ref.iter().map(|r| r[col_i]).collect::<Vec<i32>>();

                    let lines = [
                        row.get(1..col_i),                     // before in row (need to reverse)
                        row.get((col_i + 1)..(row.len() - 1)), // after in row
                        col.get(1..row_i),                     // before in col (need to reverse)
                        col.get((row_i + 1)..(col.len() - 1)), // after in col
                    ];

                    match lines {
                        [Some(r_before), Some(r_after), Some(c_before), Some(c_after)] => {
                            let (a, b, c, d) = (
                                get_viewing_distance(r_before, height, true),
                                get_viewing_distance(r_after, height, false),
                                get_viewing_distance(c_before, height, true),
                                get_viewing_distance(c_after, height, false),
                            );

                            a * b * c * d
                        }
                        _ => 0,
                    }
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("{:?}", result);
}
