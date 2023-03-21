use std::fs;
use std::str::FromStr;

fn main() {
    let mut triangle = read_triangle("018/triangle.txt");

    let mut sum = triangle.pop().unwrap();

    for row in triangle.iter().rev() {
        sum = sum
            .windows(2)
            .map(|window| window[0].max(window[1]))
            .collect();
        for (i, v) in row.iter().enumerate() {
            sum[i] += v;
        }
    }

    println!("Problem 018: {sum:?}");
}

fn read_triangle(path: &str) -> Vec<Vec<u32>> {
    String::from_utf8(fs::read(path).expect("Failed to open file from path"))
        .expect("Failed to read file to string")
        .lines()
        .map(|row| {
            row.split_whitespace()
                .map(|s| u32::from_str(s).unwrap())
                .collect()
        })
        .collect()
}
