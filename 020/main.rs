fn main() {
    println!("Problem 020: {}", factorial_digit_sum(100))
}

fn factorial_digit_sum(n: u32) -> u32 {
    let mut digits = Vec::new();
    digits.push(1);

    for i in 2..=n {
        let mut quotient = 0;
        digits = digits
            .iter()
            .map(|digit| digit * i)
            .map(|n| {
                let tmp = n + quotient;
                quotient = tmp / 10;
                tmp % 10
            })
            .collect();
        while quotient > 0 {
            digits.push(quotient % 10);
            quotient /= 10;
        }
    }
    digits.iter().sum()
}
