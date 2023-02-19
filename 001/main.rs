fn main() {
    let sum_of_threes = sum_below_thousand(3);
    let sum_of_fives = sum_below_thousand(5);
    let sum_of_fifteens = sum_below_thousand(15);
    println!(
        "Problem 001: {}",
        sum_of_threes + sum_of_fives - sum_of_fifteens
    );
}

fn sum_below_thousand(n: i32) -> i32 {
    let div = (1000 - 1) / n;
    return n * div * (div + 1) / 2;
}
