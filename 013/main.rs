use std::fs;

const TEN_BILLION: u64 = 10_000_000_000;

struct BigNumber(Vec<u64>);

impl BigNumber {
    fn from_string(mut s: String) -> Self {
        let mut big_num = Vec::new();

        while s.len() > 0 {
            let small_num = s.drain(..10).collect::<String>();
            big_num.push(small_num.parse().expect("Failed to parse String to u64"));
        }
        BigNumber(big_num)
    }

    fn add(&mut self, big_num: BigNumber) {
        self.0 = self.0.iter().zip(big_num.0).map(|(a, b)| a + b).collect();
    }
}

fn main() {
    let mut added = BigNumber(vec![0; 5]);

    for row in read_numbers("013/numbers.txt") {
        added.add(BigNumber::from_string(row));
    }

    while added.0.len() > 1 {
        let pop = added.0.pop().expect("Failed to pop");
        if let Some(last) = added.0.last_mut() {
            *last += pop / TEN_BILLION;
        }
    }

    println!(
        "Problem 013: {}",
        added.0[0].to_string().drain(..10).collect::<String>()
    );
}

fn read_numbers(path: &str) -> Vec<String> {
    String::from_utf8(fs::read(path).expect("Failed to open file from path"))
        .expect("Failed to read file to string")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}
