const TARGET: u32 = 2_000_000;
fn main() {
    let mut primes = vec![2, 3];

    let mut i = 1;
    while 6 * i < TARGET {
        let maybe_prime = 6 * i - 1;
        if is_prime(maybe_prime, &primes) {
            primes.push(maybe_prime);
        }
        let maybe_prime = 6 * i + 1;
        if is_prime(maybe_prime, &primes) {
            primes.push(maybe_prime);
        }
        i += 1;
    }

    let mut prime_sum: u64 = 0;
    for p in primes {
        prime_sum += u64::from(p);
    }

    println!("Problem 010: {prime_sum}");
}

fn is_prime(n: u32, primes: &Vec<u32>) -> bool {
    for p in primes {
        if n % p == 0 {
            return false;
        }
    }
    true
}
