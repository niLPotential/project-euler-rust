fn main() {
    println!("Problem 015: {}", collapse(expand(20)));
}

fn expand(n: u32) -> Vec<u64> {
    let mut diagonal = vec![1];

    for _ in 0..n {
        let mut next = vec![1];
        for i in 0..diagonal.len() - 1 {
            next.push(diagonal[i] + diagonal[i + 1]);
        }
        next.push(1);
        diagonal = next;
    }

    diagonal
}

fn collapse(mut diagonal: Vec<u64>) -> u64 {
    while diagonal.len() > 1 {
        let mut next = Vec::new();
        for i in 0..diagonal.len() - 1 {
            next.push(diagonal[i] + diagonal[i + 1]);
        }
        diagonal = next
    }
    diagonal[0]
}
