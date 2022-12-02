
use std::fs;

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let items = data.lines();
}