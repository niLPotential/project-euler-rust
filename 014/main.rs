const MILLION: usize = 1_000_000;

fn main() {
    let mut marked = [false; MILLION];
    let mut longest_start = 0;
    let mut longest_length = 0;

    for i in (1..MILLION).rev() {
        if !marked[i - 1] {
            let mut length = 1;
            let mut n = i;
            while n > 1 {
                n = collatz(n);
                if n < MILLION {
                    marked[n - 1] = true;
                }
                length += 1;
            }
            if length > longest_length {
                longest_start = i;
                longest_length = length;
            }
        }
    }

    println!("Problem 014: {longest_start}")
}

fn collatz(n: usize) -> usize {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}
