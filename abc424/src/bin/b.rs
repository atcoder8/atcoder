use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m, k): (usize, usize, usize),
        ab: [(Usize1, Usize1); k],
    }

    let mut counts = vec![0; n];
    let mut participants = vec![];
    for &(a, _b) in &ab {
        counts[a] += 1;
        if counts[a] == m {
            participants.push(a);
        }
    }

    if !participants.is_empty() {
        let output = participants.iter().map(|v| v + 1).join(" ");
        println!("{output}");
    }
}
