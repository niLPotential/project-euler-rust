const LIMIT: u32 = 100;

fn main() {
    let mut count = 0;

    for a in 2..=LIMIT {
        let log = log_base_smth(a);
        count += LIMIT - 1;
        for b in 2..=LIMIT {
            if (1..log).any(|i| (log * b) / i <= LIMIT && (log * b) % i == 0) {
                count -= 1;
            };
        }
    }

    println!("Problem 029: {}", count);
}

fn log_base_smth(n: u32) -> u32 {
    let mut base = 2;
    while base * base <= n {
        let log = n.ilog(base);
        if base.pow(log) == n {
            return log;
        }
        base += 1;
    }
    1
}
