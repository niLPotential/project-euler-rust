const TARGET: u32 = 1001;

fn main() {
    let mut sum_of_diagonals = 1; // start with 1
    let mut n = 3; // n by n spiral

    while n <= TARGET {
        // upper right is n*n
        // upper left is n*n-n+1
        // down left is n*n-2n+2
        // down right is n*n-3n+3
        sum_of_diagonals += 4 * n * n - 6 * n + 6; // sum of four corners

        n += 2; // n is odd
    }

    println!("Problem 028: {}", sum_of_diagonals);
}
