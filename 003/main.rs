const TARGET: i64 = 600851475143;

fn main() {
    let mut primes = vec![2, 3];
    let mut prime_factors = Vec::new();

    let mut target_copy = TARGET;
    let mut i = 0;

    while 36 * i * i < target_copy {
        i += 1;
        let maybe_prime = 6 * i - 1;
        if is_prime(maybe_prime, &primes) {
            primes.push(maybe_prime);
            if is_prime_factor(target_copy, maybe_prime) {
                target_copy /= maybe_prime;
                prime_factors.push(maybe_prime)
            }
        }
        let maybe_prime = 6 * i + 1;
        if is_prime(maybe_prime, &primes) {
            primes.push(maybe_prime);
            if is_prime_factor(target_copy, maybe_prime) {
                target_copy /= maybe_prime;
                prime_factors.push(maybe_prime)
            }
        }
    }

    println!("Problem 003: {}", target_copy);
}

fn is_prime_factor(n: i64, p: i64) -> bool {
    return n % p == 0;
}

fn is_prime(n: i64, primes: &Vec<i64>) -> bool {
    for p in primes {
        if n % p == 0 {
            return false;
        }
    }
    return true;
}
