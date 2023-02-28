const LIMIT: usize = 2_000_000;

fn main() {
    // zero-based array indexing
    let mut sieve = [false; LIMIT];
    sieve[0] = true;
    sieve[LIMIT - 1] = true;

    let mut n = 4;
    while n < LIMIT {
        sieve[n - 1] = true;
        n += 2;
    }

    let mut n = 3;
    while n * n < LIMIT {
        if !sieve[n - 1] {
            let mut m = n * n;
            while m < LIMIT {
                sieve[m - 1] = true;
                m += 2 * n
            }
        }
        n += 2;
    }

    let mut sum = 0;
    for (i, p) in sieve.iter().enumerate() {
        if !p {
            sum += i + 1;
        }
    }

    println!("Problem 010: {sum}");
}
