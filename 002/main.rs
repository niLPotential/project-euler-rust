const MAX_LIMIT: u32 = 4000000;

fn main() {
    let mut sum_of_even_fibonacci = 0;

    let mut first_fibonacci = 1;
    let mut second_fibonacci = 2;

    while second_fibonacci < MAX_LIMIT {
        if second_fibonacci % 2 == 0 {
            sum_of_even_fibonacci += second_fibonacci;
        }

        second_fibonacci += first_fibonacci;
        first_fibonacci = second_fibonacci - first_fibonacci;
    }

    println!("Problem 002: {}", sum_of_even_fibonacci)
}
