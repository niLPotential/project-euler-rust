use std::fs;

fn main() {
    let mut names = read_names("022/0022_names.txt");
    names.sort_unstable();

    let names_scores: usize = names
        .iter()
        .enumerate()
        .map(|(i, name)| (i + 1) * value_of_name(name))
        .sum();

    println!("Problem 022: {names_scores}");
}

fn value_of_name(name: &String) -> usize {
    name.chars()
        .map(|char| match char {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            'D' => 4,
            'E' => 5,
            'F' => 6,
            'G' => 7,
            'H' => 8,
            'I' => 9,
            'J' => 10,
            'K' => 11,
            'L' => 12,
            'M' => 13,
            'N' => 14,
            'O' => 15,
            'P' => 16,
            'Q' => 17,
            'R' => 18,
            'S' => 19,
            'T' => 20,
            'U' => 21,
            'V' => 22,
            'W' => 23,
            'X' => 24,
            'Y' => 25,
            'Z' => 26,
            _ => 0,
        })
        .sum()
}

fn read_names(path: &str) -> Vec<String> {
    String::from_utf8(fs::read(path).expect("Failed to open file from path"))
        .expect("Failed to read file to string")
        .split(",")
        .map(|str| String::from(str))
        .collect()
}
