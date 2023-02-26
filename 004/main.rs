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
    let str = n.to_string();
    let mut chars = str.chars().collect::<Vec<char>>();
    chars.reverse();
    let mut reverse_str = String::new();
    for ch in chars {
        reverse_str.push(ch);
    }
    return str == reverse_str;
}
