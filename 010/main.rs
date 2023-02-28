const LIMIT: usize = 2_000_000;
const SIEVEBOUND: usize = LIMIT / 2 - 1;
fn main() {
    // index i corresponds to p=2*i+1
    let mut sieve = [false; SIEVEBOUND];
    // 1 is not a prime number
    sieve[0] = true;

    let crosslimit = ((LIMIT as f32).sqrt() as usize - 1) / 2;

    for i in 1..=crosslimit {
        if !sieve[i] {
            let mut j = 2 * i * (i + 1);
            while j < SIEVEBOUND {
                sieve[j] = true;
                j += 2 * i + 1;
            }
        }
    }

    // 2 is a prime number
    let mut sum = 2;
    for (i, p) in sieve.iter().enumerate() {
        if !p {
            sum += 2 * i + 1;
        }
    }

    println!("Problem 010: {sum}");
}
