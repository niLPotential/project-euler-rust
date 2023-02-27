fn main() {
    let mut lcm = 2;
    for i in 3..20 {
        lcm = least_common_multiple(lcm, i)
    }
    println!("Problem 005: {}", lcm);
}

fn least_common_multiple(a: u32, b: u32) -> u32 {
    return a * b / greatest_common_divisor(a, b);
}

fn greatest_common_divisor(mut a: u32, mut b: u32) -> u32 {
    if b > a {
        a += b;
        b = a - b;
    }
    while b != 0 {
        (a, b) = (b, a % b);
    }
    return a;
}
