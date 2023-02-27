use std::fs;
use std::str::FromStr;

const TARGET: usize = 13;
fn main() {
    let num_string = read_number("008/number.txt");

    let num_vec = num_string
        .chars()
        .map(|c| u64::from_str(&c.to_string()).unwrap())
        .collect::<Vec<u64>>();

    let mut largest_product = 0;

    for i in 0..(num_vec.len() - TARGET) {
        let mut product = 1;
        for n in &num_vec[i..i + TARGET] {
            product *= n;
        }
        if product > largest_product {
            largest_product = product;
        }
    }

    println!("Problem 008: {}", largest_product);
}

fn read_number(path: &str) -> String {
    String::from_utf8(fs::read(path).expect("Failed to open file from path"))
        .expect("Failed to read file to string")
        .split_whitespace()
        .collect()
}
