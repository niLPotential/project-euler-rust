const TARGET: usize = 1000;

fn main() {
    let mut fibonacci_1 = vec![1];
    let mut fibonacci_2 = vec![1];
    let mut index = 2;

    while fibonacci_2.len() < TARGET {
        (fibonacci_2, fibonacci_1) = (add_large_numbers(fibonacci_1, &fibonacci_2), fibonacci_2);
        index += 1;
    }

    println!("Problem 025: {index}");
}

fn add_large_numbers(mut first: Vec<u32>, second: &Vec<u32>) -> Vec<u32> {
    first.resize(second.len(), 0); // Resize first (which is always smaller) for zip to work
    let mut quotient = 0;
    let mut result: Vec<u32> = second
        .iter()
        .zip(first.iter())
        .map(|(m, n)| m + n)
        .map(|n| {
            let tmp = n + quotient;
            quotient = tmp / 10;
            tmp % 10
        })
        .collect();
    while quotient > 0 {
        result.push(quotient % 10);
        quotient /= 10;
    }
    result
}
