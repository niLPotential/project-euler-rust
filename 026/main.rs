const TARGET: u32 = 1000;

fn main() {
    let mut longest_cycle = 0;
    let mut longest_n = 0;
    for n in 2..TARGET {
        let cycle_length = reciprocal_cycle(n);
        if cycle_length > longest_cycle {
            longest_cycle = cycle_length;
            longest_n = n;
        }
    }
    println!("Problem 026: {longest_n}");
}

fn reciprocal_cycle(n: u32) -> usize {
    let mut remainders = vec![1];
    loop {
        if let Some(remainder) = remainders.last() {
            let new_remainder = (remainder * 10) % n;
            if new_remainder == 0 {
                return 0;
            }
            if remainders.contains(&new_remainder) {
                let index = remainders.iter().position(|&v| v == new_remainder).unwrap();
                return remainders.len() - index;
            }
            remainders.push(new_remainder);
        } else {
            return 0;
        }
    }
}
