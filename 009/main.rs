const TARGET: u32 = 1000;
fn main() {
    for c in (TARGET / 3)..(TARGET / 2) {
        for b in ((TARGET - c) / 2)..(TARGET - c) {
            let a = TARGET - b - c;
            if is_pythagorean(a, b, c) {
                println!("Problem 009: {}", a * b * c);
            }
        }
    }
}

fn is_pythagorean(a: u32, b: u32, c: u32) -> bool {
    a * a + b * b == c * c
}
