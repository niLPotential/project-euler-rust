const TARGET: usize = 10001;

fn main() {
    let mut primes = vec![2, 3];
    let mut i = 0;
    while primes.len() < TARGET {
        i += 1;
        let maybe_prime = 6 * i - 1;
        if is_prime(maybe_prime, &primes) {
            primes.push(maybe_prime);
        }
        let maybe_prime = 6 * i + 1;
        if is_prime(maybe_prime, &primes) {
            primes.push(maybe_prime);
        }
    }
    println!("Problem 007: {}", primes[TARGET - 1]);
}

fn is_prime(n: u32, primes: &Vec<u32>) -> bool {
    for p in primes {
        if n % p == 0 {
            return false;
        }
    }
    true
}
