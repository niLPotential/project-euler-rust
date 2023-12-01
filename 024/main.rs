const TARGET: usize = 1_000_000;

fn main() {
    let mut result = Vec::new();
    let mut digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut steps_taken = 1;

    while steps_taken < TARGET {
        let steps_left = TARGET - steps_taken;
        let step = factorial(digits.len() - 1);
        let times = steps_left / step;
        steps_taken += times * step;
        result.push(digits.remove(times));
    }
    result.append(&mut digits);

    println!("Problem 024: {:?}", result);
}

// n should not be too big
fn factorial(n: usize) -> usize {
    if n <= 1 {
        return 1;
    }
    factorial(n - 1) * n
}
