const LIMIT: i32 = 1000;

fn main() {
    // Only primes under LIMIT is needed
    let primes = list_primes(LIMIT);

    let mut max = 0;
    let mut max_a = 0;
    let mut max_b = 0;

    // b must be a prime
    for &b in primes.iter().rev() {
        let mut a = 1;
        // upper limit of a
        while a * a < 4 * b {
            let length = consecutive_primes(a, b, &primes);
            if length > max {
                max = length;
                max_a = a;
                max_b = b;
            }

            let length = consecutive_primes(-a, b, &primes);
            if length > max {
                max = length;
                max_a = -a;
                max_b = b;
            }

            a += 2; // a is odd
        }
    }

    println!("Problem 027: {}", max_a * max_b);
}

fn consecutive_primes(a: i32, b: i32, primes: &Vec<i32>) -> i32 {
    let mut n = 0;
    while is_prime(n * n + a * n + b, primes) {
        n += 1;
    }
    return n;
}

fn list_primes(n: i32) -> Vec<i32> {
    let mut primes = vec![2, 3];
    let mut i = 1;
    while 6 * i < n {
        if is_prime(6 * i - 1, &primes) {
            primes.push(6 * i - 1);
        }
        if is_prime(6 * i + 1, &primes) {
            primes.push(6 * i + 1);
        }
        i += 1;
    }
    primes
}

fn is_prime(n: i32, primes: &Vec<i32>) -> bool {
    primes.contains(&n) || primes.iter().filter(|&p| p * p <= n).all(|p| n % p != 0)
}
