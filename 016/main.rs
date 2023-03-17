const TARGET: usize = 1000;
// TEN = 1010
fn main() {
    let mut binary = [false; TARGET + 1];
    binary[TARGET] = true;

    let mut sum = 0;

    loop {
        let quotient = divide_by_ten(&mut binary);
        let mut remainder = 0;
        for i in 0..=3 {
            if binary[i] && remainder < 10 {
                remainder += 2usize.pow(i as u32);
            }
        }
        sum += remainder;
        // println!("{quotient:?} {binary:?} {remainder}");

        let mut zero = true;
        quotient.map(|b| {
            if b {
                zero = false
            }
        });
        if zero {
            break;
        }
        binary = quotient
    }

    println!("Problem 016: {sum}");
}

fn divide_by_ten(binary: &mut [bool; TARGET + 1]) -> [bool; TARGET + 1] {
    let mut quotient = [false; TARGET + 1];
    for i in (4..=TARGET).rev() {
        if binary[i] {
            // 1?1?...
            if binary[i - 2] {
                binary[i] = false;
                binary[i - 2] = false;
                quotient[i - 3] = true;
            }
            // 110?...
            else if binary[i - 1] {
                binary[i] = false;
                binary[i - 1] = false;
                binary[i - 2] = true;
                quotient[i - 3] = true;
            }
            // 1001?...
            else if binary[i - 3] {
                binary[i] = false;
                binary[i - 1] = true;
                binary[i - 3] = false;
                quotient[i - 4] = true;
            }
            // 1000?...
            else {
                binary[i] = false;
                binary[i - 2] = true;
                binary[i - 3] = true;
                quotient[i - 4] = true;
            }
        }
    }

    if binary[3] {
        // 1?1?
        if binary[1] {
            binary[3] = false;
            binary[1] = false;
            quotient[0] = true;
        }
        // 110?
        else if binary[2] {
            binary[3] = false;
            binary[2] = false;
            binary[1] = true;
            quotient[0] = true;
        } // 100?
    }

    quotient
}
