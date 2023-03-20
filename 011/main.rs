use std::fs;
use std::str::FromStr;
const LENGTH: usize = 4;
fn main() {
    let grid = read_grid("011/grid.txt");

    let mut greatest_product = 0;

    // horizontal
    for row in 0..grid.len() {
        for col in 0..(grid[0].len() - LENGTH) {
            let mut product = 1;
            for j in col..(col + LENGTH) {
                product *= grid[row][j];
            }
            if product > greatest_product {
                greatest_product = product;
            }
        }
    }

    // vertical
    for col in 0..grid.len() {
        for row in 0..(grid.len() - LENGTH) {
            let mut product = 1;
            for slice in grid.iter().skip(row).take(LENGTH) {
                product *= slice[col]
            }
            if product > greatest_product {
                greatest_product = product;
            }
        }
    }

    // diagonal ↘
    for row in 0..(grid.len() - LENGTH) {
        for col in 0..(grid[0].len() - LENGTH) {
            let mut product = 1;
            for i in 0..LENGTH {
                product *= grid[row + i][col + i];
            }
            if product > greatest_product {
                greatest_product = product;
            }
        }
    }

    // diagonal ↙
    for row in (LENGTH - 1)..grid.len() {
        for col in 0..(grid[0].len() - LENGTH) {
            let mut product = 1;
            for i in 0..LENGTH {
                product *= grid[row - i][col + i];
            }
            if product > greatest_product {
                greatest_product = product;
            }
        }
    }

    println!("Problem 011: {greatest_product}");
}

fn read_grid(path: &str) -> Vec<Vec<u32>> {
    String::from_utf8(fs::read(path).expect("Failed to open file from path"))
        .expect("Failed to read file to string")
        .lines()
        .map(|row| {
            row.split_whitespace()
                .map(|s| u32::from_str(s).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}
