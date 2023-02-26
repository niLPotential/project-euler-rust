fn main() {
    let mut largest_palindrome_product = 0;
    for a in (100..1000).rev() {
        let (mut b, db) = get_b(a);
        while b >= a {
            let product = a * b;
            if product > largest_palindrome_product && is_palindrome(product) {
                largest_palindrome_product = product;
            }
            b -= db;
        }
    }
    println!("Problem 004: {}", largest_palindrome_product);
}

fn get_b(a: u32) -> (u32, u32) {
    if a % 11 == 0 {
        return (999, 1);
    } else {
        return (990, 11);
    }
}

fn is_palindrome(n: u32) -> bool {
    return n == reverse_int(n);
}

fn reverse_int(mut n: u32) -> u32 {
    let mut reversed = 0;
    while n > 0 {
        reversed = 10 * reversed + n % 10;
        n /= 10;
    }
    return reversed;
}
