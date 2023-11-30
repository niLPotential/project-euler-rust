fn main() {
    let mut sum = 0;
    for i in 2..=10000 {
        if amicable(i) {
            let pair = sum_of_proper_divisors(i);
            println!("{i} {pair}");
            sum += i + pair;
        }
    }
    println!("Problem 021: {sum}");
}

fn amicable(n: u32) -> bool {
    let potential_pair = sum_of_proper_divisors(n);
    (n < potential_pair) && (n == sum_of_proper_divisors(potential_pair))
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
