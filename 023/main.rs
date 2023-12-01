const LIMIT: u32 = 28123;

fn main() {
    let mut abundant_numbers = Vec::new();
    let mut sum_of_non_abundant_sum = 0;

    for n in 1..=LIMIT {
        if is_abundant(n) {
            abundant_numbers.push(n);
        }
        if !is_abundant_sum(n, &abundant_numbers) {
            sum_of_non_abundant_sum += n;
        }
    }

    println!("Problem 023: {}", sum_of_non_abundant_sum);
}

fn is_abundant_sum(n: u32, abundant_numbers: &Vec<u32>) -> bool {
    abundant_numbers
        .iter()
        .filter(|&abundant| *abundant <= n / 2)
        .any(|abundant| abundant_numbers.binary_search(&(n - abundant)).is_ok())
}

fn is_abundant(n: u32) -> bool {
    n < sum_of_proper_divisors(n)
}

fn sum_of_proper_divisors(n: u32) -> u32 {
    let mut sum = 1;
    let mut i = 2;
    while i * i < n {
        if n % i == 0 {
            sum += i + n / i;
        }
        i += 1;
    }
    if i * i == n {
        sum += i;
    }
    sum
}
