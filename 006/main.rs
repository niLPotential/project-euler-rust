const TARGET: u32 = 100;
fn main() {
    println!(
        "Problem 006: {}",
        square_of_sum(TARGET) - sum_of_square(TARGET)
    );
}

fn sum_of_square(n: u32) -> u32 {
    n * (n + 1) * (2 * n + 1) / 6
}

fn square_of_sum(n: u32) -> u32 {
    n * n * (n + 1) * (n + 1) / 4
}
