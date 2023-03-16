const TARGET: u32 = 500;
fn main() {
    let mut n = 0;

    loop {
        n += 1;

        let d = if n % 2 == 0 {
            divisors(n / 2) * divisors(n + 1)
        } else {
            divisors(n) * divisors((n + 1) / 2)
        };

        if d > TARGET {
            break;
        }
    }

    println!("Problem 012: {}", triangle(n));
}

fn divisors(mut n: u32) -> u32 {
    let limit = (n as f32).sqrt() as u32;
    let primes = primes(limit);

    let mut d = 1;
    for p in primes {
        let mut pow = 0;
        while n % p == 0 {
            pow += 1;
            n /= p;
        }
        d *= pow + 1;
    }
    d
}

fn primes(limit: u32) -> Vec<u32> {
    // prevent subtraction overflow or index out of bounds
    if limit < 4 {
        return Vec::new();
    }
    let sievebound = (limit / 2 - 1) as usize;
    let mut sieve = vec![false; sievebound];
    sieve[0] = true;

    let crosslimit = ((limit as f32).sqrt() as usize - 1) / 2;

    for i in 1..=crosslimit {
        if !sieve[i] {
            let mut j = 2 * i * (i + 1);
            while j < sievebound {
                sieve[j] = true;
                j += 2 * i + 1;
            }
        }
    }

    let mut primes = vec![2];
    primes.append(
        &mut sieve
            .iter()
            .enumerate()
            .filter_map(|(i, &p)| if p { None } else { Some((2 * i + 1) as u32) })
            .collect(),
    );
    primes
}

fn triangle(n: u32) -> u32 {
    n * (n + 1) / 2
}
