fn main() {
    let mut largest_palindrome_product = 0;
    for i in (100..1000).rev() {
        for j in (100..1000).rev() {
            let product = i * j;
            if product > largest_palindrome_product && is_palindrome(product) {
                largest_palindrome_product = product
            }
        }
    }
    println!("Problem 004: {}", largest_palindrome_product);
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
